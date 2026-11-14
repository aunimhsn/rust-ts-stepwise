//! The memory of the machine: what changes while a program runs, and what each instruction
//! does to it.

use std::iter;
use std::mem;
use std::rc::Rc;

use super::builtins::{self, Outcome};
use super::code::{Function, Instruction};
use super::error::RuntimeErrorKind;
use super::event::Event;
use super::operators;
use super::value::Value;
use crate::ast::BinaryOperator;
use crate::span::{Position, Span};
use crate::world::World;

/// The most calls in progress, one inside the other, the main program apart: one more stops
/// the program (see *Functions* in `specs.md`).
///
/// It bounds what the machine keeps: a recursion that never ends can't fill the memory of
/// the page, nor does it use the stack of the browser, since a call in progress is data.
pub const MAX_CALL_DEPTH: usize = 200;

/// A call in progress, as the variables panel shows it: the main program, or a function that
/// has been called and hasn't ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallInProgress<'a> {
    /// The name of its function, `None` for the main program.
    pub function: Option<&'a str>,
    /// The line it is on. For the call being run: the line of the next step, or the last
    /// one run once the program has ended or stopped on an error. For the others: the line
    /// of the call they wait for.
    pub line: Span,
    /// Its variables alive, in the order they were declared, its parameters first.
    pub variables: Vec<Variable<'a>>,
}

/// A variable of a call in progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable<'a> {
    /// Its name.
    pub name: &'a str,
    /// Its value.
    pub value: &'a Value,
    /// Whether the last step wrote it: declared it, with a `let` or as a parameter of the
    /// call entered, or gave it a value with `=`, even the value it had.
    pub changed: bool,
}

/// A variable alive at the place the program has reached, as its frame keeps it.
#[derive(Debug)]
struct Slot {
    /// Its name, which the machine doesn't use: it reads a variable by its slot.
    name: Rc<str>,
    value: Value,
    /// The step that wrote it last.
    written: u64,
}

/// A call in progress: the main program, or a function that has been called and hasn't
/// ended.
#[derive(Debug)]
struct Frame {
    /// The name of its function, `None` for the main program.
    function: Option<Rc<str>>,
    /// The index of its next instruction.
    next: usize,
    /// Its variables alive, each at the index of its slot.
    variables: Vec<Slot>,
    /// The line it is on: the last one a step of it started on.
    line: Span,
    /// The number of operands when it started: the ones above are its own, and end with it.
    operands: usize,
}

/// What the machine does after an instruction.
pub(super) enum Flow {
    /// It runs the next one.
    Continue,
    /// It pauses on the line of the call being run: the step is over.
    Pause {
        /// Whether a call of that line has just ended, the rest of the line being left to
        /// run, rather than the line starting.
        after_call: bool,
    },
    /// The program has ended.
    End,
}

/// What changes in a program while it runs: where it is, its calls in progress with their
/// variables, and the values of the expression being evaluated. Apart from the instructions,
/// which it only reads, and from the world of the robot, which only the built-in functions
/// change.
#[derive(Debug)]
pub(super) struct Memory {
    /// The call being run: the main program, or the function called last.
    frame: Frame,
    /// The calls that wait for the one they made to end, the main program first. Their
    /// number is the depth of the call being run.
    callers: Vec<Frame>,
    /// The values the expression being evaluated has left for its operators, above the turns
    /// left of the `repeat`s around it, and above those of the callers.
    operands: Vec<Value>,
    /// What the call that has just ended gave back, a value or none, until the expression
    /// that call is in takes it.
    result: Option<Value>,
    /// The number of the step being run, from 1: 0 before the first.
    step: u64,
}

impl Memory {
    /// The memory of a program about to start: the main program alone, before its first
    /// line.
    pub(super) const fn new() -> Self {
        let main_program = Frame {
            function: None,
            next: 0,
            variables: Vec::new(),
            line: Span::at(Position::new(1, 1)),
            operands: 0,
        };
        Self {
            frame: main_program,
            callers: Vec::new(),
            operands: Vec::new(),
            result: None,
            step: 0,
        }
    }

    /// The index of the next instruction to run.
    pub(super) const fn next(&self) -> usize {
        self.frame.next
    }

    /// The line the call being run is on.
    pub(super) const fn line(&self) -> Span {
        self.frame.line
    }

    /// Starts a step: what it writes is told apart from what the steps before it wrote.
    pub(super) const fn start_step(&mut self) {
        self.step = self.step.saturating_add(1);
    }

    /// The calls in progress, from the main program to the call being run.
    pub(super) fn calls(&self) -> Vec<CallInProgress<'_>> {
        let frames = self.callers.iter().chain(iter::once(&self.frame));
        frames
            .map(|frame| CallInProgress {
                function: frame.function.as_deref(),
                line: frame.line,
                variables: frame
                    .variables
                    .iter()
                    .map(|slot| Variable {
                        name: &slot.name,
                        value: &slot.value,
                        changed: slot.written == self.step,
                    })
                    .collect(),
            })
            .collect()
    }

    /// The numbers of operands, of variables of the call being run and of callers, which a
    /// loop must not let grow.
    #[cfg(test)]
    pub(super) const fn sizes(&self) -> (usize, usize, usize) {
        let variables = self.frame.variables.len();
        (self.operands.len(), variables, self.callers.len())
    }

    /// Runs `instruction`, the next one, which comes from `span` and which the program moves
    /// past first: a jump then says where it goes on. A built-in function acts on `world`,
    /// and what the learner sees of it goes to `event`.
    pub(super) fn execute(
        &mut self,
        instruction: &Instruction,
        span: Span,
        world: &mut World,
        event: &mut Option<Event>,
    ) -> Result<Flow, RuntimeErrorKind> {
        self.frame.next = self.frame.next.saturating_add(1);
        match instruction {
            Instruction::Line => {
                self.frame.line = span;
                return Ok(Flow::Pause { after_call: false });
            }
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
                let variable = variable.ok_or(RuntimeErrorKind::Internal)?;
                variable.value = value;
                variable.written = self.step;
            }
            Instruction::Declare(name) => {
                let value = self.pop()?;
                self.frame.variables.push(Slot {
                    name: Rc::clone(name),
                    value,
                    written: self.step,
                });
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
            Instruction::RepeatStart => self.repeat_count()?,
            Instruction::RepeatNext(exit) => self.next_turn(*exit)?,
            Instruction::CallBuiltin { builtin, arguments } => {
                let values = self.take_arguments(*arguments)?;
                self.result = match builtins::call(*builtin, &values, world)? {
                    Outcome::Acted(acted) => {
                        *event = Some(acted);
                        // What acts gives back no value.
                        None
                    }
                    Outcome::Sensed(sensed) => Some(Value::Boolean(sensed)),
                };
            }
            Instruction::Call(function) => self.enter(function, span)?,
            Instruction::Return { value } => {
                self.leave(*value)?;
                return Ok(Flow::Pause { after_call: true });
            }
            Instruction::TakeResult { function } => {
                let Some(result) = self.result.take() else {
                    let function = function.to_string();
                    return Err(RuntimeErrorKind::NoValue { function });
                };
                self.operands.push(result);
            }
        }
        Ok(Flow::Continue)
    }

    /// Checks the count of a `repeat`, the last operand, which stays there as its turns
    /// left.
    fn repeat_count(&self) -> Result<(), RuntimeErrorKind> {
        match self.operands.last() {
            Some(&Value::Number(count)) if count >= 0 => Ok(()),
            Some(&Value::Number(count)) => Err(RuntimeErrorKind::NegativeRepeatCount { count }),
            Some(other) => {
                let found = other.value_type();
                Err(RuntimeErrorKind::RepeatCountNotNumber { found })
            }
            None => Err(RuntimeErrorKind::Internal),
        }
    }

    /// Starts a turn of a `repeat`, whose turns left are the last operand, or goes on at
    /// `exit` when none is left.
    fn next_turn(&mut self, exit: usize) -> Result<(), RuntimeErrorKind> {
        let Some(Value::Number(turns_left)) = self.operands.last_mut() else {
            return Err(RuntimeErrorKind::Internal);
        };
        if *turns_left > 0 {
            *turns_left = turns_left.saturating_sub(1);
        } else {
            self.operands.pop();
            self.frame.next = exit;
        }
        Ok(())
    }

    /// Enters `function`, called at `span`: a new call in progress, whose parameters take
    /// the last operands as their values, in order.
    fn enter(&mut self, function: &Function, span: Span) -> Result<(), RuntimeErrorKind> {
        if self.callers.len() >= MAX_CALL_DEPTH {
            let function = function.name.to_string();
            return Err(RuntimeErrorKind::TooManyCalls { function });
        }
        let values = self.take_arguments(function.parameters.len())?;
        let parameters = function.parameters.iter().zip(values);
        let entered = Frame {
            function: Some(Rc::clone(&function.name)),
            next: function.entry,
            variables: parameters
                .map(|(name, value)| Slot {
                    name: Rc::clone(name),
                    value,
                    written: self.step,
                })
                .collect(),
            // Until its first line, the next instruction, where the step ends.
            line: span,
            operands: self.operands.len(),
        };
        let caller = mem::replace(&mut self.frame, entered);
        self.callers.push(caller);
        Ok(())
    }

    /// Leaves the function being run, back to its caller: its result is the last operand
    /// when it gives back a `value`, none otherwise.
    fn leave(&mut self, value: bool) -> Result<(), RuntimeErrorKind> {
        let result = if value { Some(self.pop()?) } else { None };
        // The main program has no caller: no `return` is translated there.
        let caller = self.callers.pop().ok_or(RuntimeErrorKind::Internal)?;
        let ended = mem::replace(&mut self.frame, caller);
        // What it left on the operands ends with it: the turns left of its `repeat`s.
        self.operands.truncate(ended.operands);
        self.result = result;
        Ok(())
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
