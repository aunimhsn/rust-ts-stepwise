//! The machine: runs the instructions of a program one step at a time, and keeps the whole
//! state of the program between two steps.

use super::code::Code;
use super::compiler::{NotChecked, compile};
use super::error::{RuntimeError, RuntimeErrorKind};
use super::event::Event;
use super::memory::{CallInProgress, Flow, Memory};
use crate::ast::Program;
use crate::span::Span;

/// Where a program is, between two steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// The program runs: the next step runs `line`, the one to highlight.
    Running {
        /// The line of the next step: a statement, or the line of an `if`, of an `else if` or
        /// of a loop, with its `:`.
        line: Span,
        /// Whether a call of that line has just ended: the line is highlighted again, and
        /// the next step runs the rest of it. A breakpoint pauses a program before a line
        /// starts, not when the line is resumed.
        after_call: bool,
    },
    /// The program has run to its end.
    Ended,
    /// The program has stopped on an error: what it had done until then stays.
    Failed(RuntimeError),
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
                    memory: Memory::new(),
                    state: State::Failed(RuntimeError { kind, span }),
                }
            }
        }
    }

    /// A machine about to run `code`, paused before its first line.
    fn start(code: Code) -> Self {
        let mut machine = Self {
            code,
            memory: Memory::new(),
            state: State::Ended,
        };
        // Up to the first line, where the first step starts: nothing runs before it.
        machine.state = machine.advance(&mut None);
        machine
    }

    /// Where the program is.
    #[must_use]
    pub const fn state(&self) -> &State {
        &self.state
    }

    /// The calls in progress, from the main program to the function being run, each with
    /// its line and its variables: what the variables panel shows. They stay as they were
    /// once the program has ended or stopped on an error.
    #[must_use]
    pub fn calls(&self) -> Vec<CallInProgress<'_>> {
        self.memory.calls()
    }

    /// Runs one step: the line of [`State::Running`], up to the next line, to the first
    /// line of a function it calls, or to the end of the function it is in, which goes back
    /// to the line of its call. Once the program has ended or stopped on an error, nothing
    /// happens anymore.
    pub fn step(&mut self) -> Option<Event> {
        if !matches!(self.state, State::Running { .. }) {
            return None;
        }
        self.memory.start_step();
        let mut event = None;
        self.state = self.advance(&mut event);
        event
    }

    /// Runs the instructions up to the next pause, the end or an error.
    ///
    /// It always comes back: the compiler writes a line, which pauses, on the way of every
    /// jump back. A call keeps that rule, since it goes to the first instruction of a
    /// function, a line, and so does the end of a function, which pauses. So a step runs
    /// each instruction at most once, hence at most as many as the code holds. The machine
    /// counts them all the same, up to exactly that number, and stops on
    /// [`RuntimeErrorKind::Internal`] rather than freeze the page if instructions ever broke
    /// that rule.
    fn advance(&mut self, event: &mut Option<Event>) -> State {
        // Where the machine is: the place of an error that has no instruction.
        let mut place = self.memory.line();
        let mut instructions_left = self.code.len();
        let kind = loop {
            let Some((instruction, span)) = self.code.get(self.memory.next()) else {
                break RuntimeErrorKind::Internal;
            };
            place = *span;
            let Some(left) = instructions_left.checked_sub(1) else {
                break RuntimeErrorKind::Internal;
            };
            instructions_left = left;
            match self.memory.execute(instruction, place, event) {
                Ok(Flow::Continue) => {}
                Ok(Flow::Pause { after_call }) => {
                    let line = self.memory.line();
                    return State::Running { line, after_call };
                }
                Ok(Flow::End) => return State::Ended,
                Err(kind) => break kind,
            }
        };
        State::Failed(RuntimeError { kind, span: place })
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::{Machine, State};
    use crate::ast::BinaryOperator;
    use crate::builtin::Builtin;
    use crate::interpreter::code::{Code, Function, Instruction};
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
            if matches!(machine.state(), State::Running { line, .. } if line.start.line == 3) {
                sizes.push(machine.memory.sizes());
            }
        }
        assert!(sizes.len() > 100, "{sizes:?}");
        assert!(sizes.iter().all(|&size| size == (0, 1, 0)), "{sizes:?}");
    }

    #[test]
    fn a_loop_of_calls_keeps_no_more_values_at_each_turn() {
        let source = "\
function find(limit):
    repeat 3:
        let turn = 0
        repeat limit:
            turn = turn + 1
            if turn == limit:
                return turn * 10
    return 0

let total = 0
while true:
    total = find(2) + find(1) - 30
";
        let program = parse(source).unwrap();
        let mut machine = Machine::new(&program);
        let mut sizes = Vec::new();
        for _ in 0..3000 {
            machine.step();
            // Before the line 12 starts: only `total` is alive, and the calls of the turn
            // before left no operand, nor the turns of the loops they came back from.
            let line_starts = State::Running {
                line: span(12, 5, 35),
                after_call: false,
            };
            if *machine.state() == line_starts {
                sizes.push(machine.memory.sizes());
            }
        }
        assert!(sizes.len() > 100, "{sizes:?}");
        assert!(sizes.iter().all(|&size| size == (0, 1, 0)), "{sizes:?}");
    }

    /// What no program gives the machine: instructions that the compiler never writes.
    #[test]
    fn wrong_instructions_stop_the_machine_without_a_panic() {
        let print_two_values = Instruction::CallBuiltin {
            builtin: Builtin::Print,
            arguments: 2,
        };
        let short_circuit = Instruction::ShortCircuit {
            operator: BinaryOperator::And,
            decisive: false,
            end: 0,
        };
        let number = || Instruction::Push(Value::Number(1));
        let boolean = || Instruction::Push(Value::Boolean(true));
        let call = |parameters: &[&str], entry| {
            Instruction::Call(Rc::new(Function {
                name: "walk".into(),
                parameters: parameters.iter().map(|&name| name.into()).collect(),
                entry,
            }))
        };
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
            // A function left by the main program, which no call entered, or without the
            // value it gives back.
            vec![Instruction::Return { value: false }],
            vec![number(), Instruction::Return { value: true }],
            vec![call(&[], 1), Instruction::Return { value: true }],
            // A call without the values of its parameters, to a function that isn't
            // written, or that goes back to itself without a line.
            vec![call(&["length"], 1), Instruction::Line],
            vec![call(&[], usize::MAX)],
            vec![call(&[], 0)],
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
