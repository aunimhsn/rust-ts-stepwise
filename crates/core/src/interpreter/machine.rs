//! The machine: runs the instructions of a program one step at a time, and keeps the whole
//! state of the program between two steps.

use std::rc::Rc;

use super::code::{Code, Instruction};
use super::compiler::{NotChecked, compile};
use super::error::{RuntimeError, RuntimeErrorKind};
use super::operators;
use super::value::Value;
use crate::ast::{BinaryOperator, Program};
use crate::builtin::Builtin;
use crate::span::{Position, Span};

/// Where a program is, between two steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// The program runs: the next step runs `line`, the one to highlight.
    Running {
        /// The line of the next step: a statement, or the line of an `if`, of an `else if` or
        /// of a loop, with its `:`.
        line: Span,
    },
    /// The program has run to its end.
    Ended,
    /// The program has stopped on an error: what it had done until then stays.
    Failed(RuntimeError),
}

/// What a step did that the learner sees outside the editor. A step does at most one such
/// thing: a function that gives back no value is a statement alone on its line, or stops the
/// program once it has run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// `print` wrote this line to the console.
    Printed(String),
}

/// A program being run, one step at a time (see *Running a program* in `specs.md`).
#[derive(Debug)]
pub struct Machine {
    /// The instructions of the program, which never change.
    code: Code,
    /// What the program has changed so far.
    memory: Memory,
    state: State,
}

impl Machine {
    /// A machine about to run `program`, paused before its first line.
    ///
    /// The checker must have found `program` without error. A program the compiler can't
    /// translate (a name that is no variable, a `break` outside a loop…) starts stopped on
    /// [`RuntimeErrorKind::Internal`], at the place that stops the compiler. Any other
    /// program the checks refuse may run: it never panics, but what it does isn't specified.
    #[must_use]
    pub fn new(program: &Program) -> Self {
        match compile(program) {
            Ok(code) => Self::start(code),
            Err(NotChecked(span)) => {
                let kind = RuntimeErrorKind::Internal;
                Self {
                    code: Code::default(),
                    memory: Memory::default(),
                    state: State::Failed(RuntimeError { kind, span }),
                }
            }
        }
    }

    /// A machine about to run `code`, paused before its first line.
    fn start(code: Code) -> Self {
        let mut machine = Self {
            code,
            memory: Memory::default(),
            state: State::Ended,
        };
        // Up to the first line, where the first step starts: nothing runs before it.
        machine.state = machine.advance(Span::at(Position::new(1, 1)), &mut None);
        machine
    }

    /// Where the program is.
    #[must_use]
    pub const fn state(&self) -> &State {
        &self.state
    }

    /// Runs one step: the line of [`State::Running`], up to the next line. Once the program
    /// has ended or stopped on an error, nothing happens anymore.
    pub fn step(&mut self) -> Option<Event> {
        let State::Running { line } = self.state else {
            return None;
        };
        let mut event = None;
        self.state = self.advance(line, &mut event);
        event
    }

    /// Runs the instructions up to the next pause, the end or an error. `line` is where the
    /// machine is, the place of an error that has no instruction.
    ///
    /// It always comes back: the compiler writes a [`Instruction::Line`], which pauses, on
    /// the way of every jump back, so no instruction runs twice between two pauses. The
    /// machine counts them all the same, and stops on [`RuntimeErrorKind::Internal`] rather
    /// than freeze the page if instructions ever broke that rule.
    fn advance(&mut self, line: Span, event: &mut Option<Event>) -> State {
        let mut place = line;
        let mut instructions_left = self.code.here();
        let kind = loop {
            let Some((instruction, span)) = self.code.get(self.memory.frame.next) else {
                break RuntimeErrorKind::Internal;
            };
            place = *span;
            let Some(left) = instructions_left.checked_sub(1) else {
                break RuntimeErrorKind::Internal;
            };
            instructions_left = left;
            self.memory.frame.next = self.memory.frame.next.saturating_add(1);
            match self.memory.execute(instruction, event) {
                Ok(Flow::Continue) => {}
                Ok(Flow::Pause) => return State::Running { line: place },
                Ok(Flow::End) => return State::Ended,
                Err(kind) => break kind,
            }
        };
        State::Failed(RuntimeError { kind, span: place })
    }
}

/// A variable alive at the place the program has reached.
#[derive(Debug)]
struct Variable {
    /// Its name, which the machine doesn't use: it reads a variable by its slot.
    #[expect(dead_code, reason = "read by the variables panel, from step 9b")]
    name: Rc<str>,
    value: Value,
}

/// The place of the program in its instructions, and its variables.
#[derive(Debug, Default)]
struct Frame {
    /// The index of the next instruction.
    next: usize,
    /// The variables alive, each at the index of its slot.
    variables: Vec<Variable>,
}

/// What the machine does after an instruction.
enum Flow {
    /// It runs the next one.
    Continue,
    /// It pauses: the step is over.
    Pause,
    /// The program has ended.
    End,
}

/// What changes while a program runs: where it is, its variables, and the values of the
/// expression being evaluated. Apart from the instructions, which it only reads.
#[derive(Debug, Default)]
struct Memory {
    frame: Frame,
    /// The values the expression being evaluated has left for its operators, above the turns
    /// left of the `repeat`s around it.
    operands: Vec<Value>,
}

impl Memory {
    fn execute(
        &mut self,
        instruction: &Instruction,
        event: &mut Option<Event>,
    ) -> Result<Flow, RuntimeErrorKind> {
        match instruction {
            Instruction::Line => return Ok(Flow::Pause),
            Instruction::End => return Ok(Flow::End),
            Instruction::Push(value) => self.operands.push(value.clone()),
            Instruction::Load(slot) => {
                let variable = self.frame.variables.get(*slot);
                let value = variable.ok_or(RuntimeErrorKind::Internal)?.value.clone();
                self.operands.push(value);
            }
            Instruction::Store(slot) => {
                let value = self.pop()?;
                let variable = self.frame.variables.get_mut(*slot);
                variable.ok_or(RuntimeErrorKind::Internal)?.value = value;
            }
            Instruction::Declare(name) => {
                let name = Rc::clone(name);
                let value = self.pop()?;
                self.frame.variables.push(Variable { name, value });
            }
            Instruction::Truncate(count) => self.frame.variables.truncate(*count),
            Instruction::Pop => {
                self.pop()?;
            }
            Instruction::Unary(operator) => {
                let operand = self.pop()?;
                self.operands.push(operators::unary(*operator, &operand)?);
            }
            Instruction::Binary(operator) => {
                let right = self.pop()?;
                let left = self.pop()?;
                self.operands
                    .push(operators::binary(*operator, &left, &right)?);
            }
            Instruction::ShortCircuit {
                operator,
                decisive,
                end,
            } => {
                if self.logic_operand(*operator)? == *decisive {
                    self.frame.next = *end;
                } else {
                    self.pop()?;
                }
            }
            Instruction::RequireBoolean(operator) => {
                self.logic_operand(*operator)?;
            }
            Instruction::Jump(target) => self.frame.next = *target,
            Instruction::JumpUnless(target) => match self.pop()? {
                Value::Boolean(true) => {}
                Value::Boolean(false) => self.frame.next = *target,
                other => {
                    let found = other.value_type();
                    return Err(RuntimeErrorKind::ConditionNotBoolean { found });
                }
            },
            Instruction::RepeatStart => match self.operands.last() {
                Some(&Value::Number(count)) if count >= 0 => {}
                Some(&Value::Number(count)) => {
                    return Err(RuntimeErrorKind::NegativeRepeatCount { count });
                }
                Some(other) => {
                    let found = other.value_type();
                    return Err(RuntimeErrorKind::RepeatCountNotNumber { found });
                }
                None => return Err(RuntimeErrorKind::Internal),
            },
            Instruction::RepeatNext(exit) => {
                let Some(Value::Number(turns_left)) = self.operands.last_mut() else {
                    return Err(RuntimeErrorKind::Internal);
                };
                if *turns_left > 0 {
                    *turns_left = turns_left.saturating_sub(1);
                } else {
                    self.operands.pop();
                    self.frame.next = *exit;
                }
            }
            Instruction::CallBuiltin {
                builtin,
                arguments,
                value_needed,
            } => {
                let values = self.take_arguments(*arguments)?;
                match builtin {
                    Builtin::Print => *event = Some(Event::Printed(printed_line(&values))),
                    // Until step 9c, which gives the machine the world of the robot.
                    _ => return Err(RuntimeErrorKind::Internal),
                }
                // `print` gives back no value.
                if *value_needed {
                    let function = builtin.as_str().to_owned();
                    return Err(RuntimeErrorKind::NoValue { function });
                }
            }
        }
        Ok(Flow::Continue)
    }

    /// Takes the last operand.
    fn pop(&mut self) -> Result<Value, RuntimeErrorKind> {
        self.operands.pop().ok_or(RuntimeErrorKind::Internal)
    }

    /// The last operand, a side of `operator`, `and` or `or`, which must be a boolean. It
    /// stays on the operands.
    fn logic_operand(&self, operator: BinaryOperator) -> Result<bool, RuntimeErrorKind> {
        match self.operands.last() {
            Some(&Value::Boolean(side)) => Ok(side),
            Some(other) => {
                let found = other.value_type();
                Err(RuntimeErrorKind::LogicOperand { operator, found })
            }
            None => Err(RuntimeErrorKind::Internal),
        }
    }

    /// Takes the last `count` operands, the values of a call, in the order they were given.
    fn take_arguments(&mut self, count: usize) -> Result<Vec<Value>, RuntimeErrorKind> {
        let first = self.operands.len().checked_sub(count);
        let first = first.ok_or(RuntimeErrorKind::Internal)?;
        Ok(self.operands.drain(first..).collect())
    }
}

/// The line `print` writes for `values`: each as it is written, separated by spaces.
fn printed_line(values: &[Value]) -> String {
    let written: Vec<String> = values.iter().map(Value::to_string).collect();
    written.join(" ")
}

#[cfg(test)]
mod tests {
    use super::{Machine, State};
    use crate::ast::BinaryOperator;
    use crate::builtin::Builtin;
    use crate::interpreter::code::{Code, Instruction};
    use crate::interpreter::{RuntimeError, RuntimeErrorKind, Value};
    use crate::parser::parse;
    use crate::test_support::span;

    #[test]
    fn a_loop_that_never_ends_keeps_no_more_values_at_each_turn() {
        let source = "\
let turns = 0
while true:
    let double = turns * 2
    repeat 2:
        if double >= 0 and true:
            break
    turns = turns + 1
";
        let program = parse(source).unwrap();
        let mut machine = Machine::new(&program);
        let mut sizes = Vec::new();
        for _ in 0..1000 {
            machine.step();
            // Before line 3, the first of the block: only `turns` is alive, and no operand
            // is left from the turn before.
            if matches!(machine.state(), State::Running { line } if line.start.line == 3) {
                let memory = &machine.memory;
                sizes.push((memory.operands.len(), memory.frame.variables.len()));
            }
        }
        assert!(sizes.len() > 100, "{sizes:?}");
        assert!(sizes.iter().all(|&size| size == (0, 1)), "{sizes:?}");
    }

    /// What no program gives the machine: instructions that the compiler never writes.
    #[test]
    fn wrong_instructions_stop_the_machine_without_a_panic() {
        let print_two_values = Instruction::CallBuiltin {
            builtin: Builtin::Print,
            arguments: 2,
            value_needed: false,
        };
        let short_circuit = Instruction::ShortCircuit {
            operator: BinaryOperator::And,
            decisive: false,
            end: 0,
        };
        let number = || Instruction::Push(Value::Number(1));
        let boolean = || Instruction::Push(Value::Boolean(true));
        for wrong in [
            // Nothing to run, not even the end of the program.
            vec![],
            vec![number()],
            // No operand to take.
            vec![Instruction::Pop],
            vec![Instruction::Store(0)],
            vec![Instruction::Declare("side".into())],
            vec![Instruction::Unary(crate::ast::UnaryOperator::Negate)],
            vec![number(), Instruction::Binary(BinaryOperator::Add)],
            vec![Instruction::JumpUnless(0)],
            vec![short_circuit],
            vec![Instruction::RequireBoolean(BinaryOperator::Or)],
            vec![Instruction::RepeatStart],
            vec![Instruction::RepeatNext(0)],
            vec![number(), print_two_values],
            // No variable in this slot.
            vec![Instruction::Load(3)],
            vec![number(), Instruction::Store(3)],
            // The turns left of a `repeat` that are no number.
            vec![boolean(), Instruction::RepeatNext(0)],
            // A jump that never landed, and jumps back that meet no line.
            vec![Instruction::Jump(usize::MAX)],
            vec![Instruction::Jump(0)],
            vec![number(), Instruction::Pop, Instruction::Jump(0)],
        ] {
            let place = span(1, 1, 2);
            let mut code = Code::default();
            for instruction in &wrong {
                code.push(instruction.clone(), place);
            }
            let mut machine = Machine::start(code);
            let kind = RuntimeErrorKind::Internal;
            // An empty list has no instruction to give its place: the start of the program.
            let span = if wrong.is_empty() {
                span(1, 1, 1)
            } else {
                place
            };
            let failed = State::Failed(RuntimeError { kind, span });
            assert_eq!(*machine.state(), failed, "{wrong:?}");
            assert_eq!(machine.step(), None, "{wrong:?}");
            assert_eq!(*machine.state(), failed, "{wrong:?}");
        }
    }
}
