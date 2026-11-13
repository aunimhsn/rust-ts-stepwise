//! The memory of the machine: what changes while a program runs, and what each instruction
//! does to it.

use std::rc::Rc;

use super::code::Instruction;
use super::error::RuntimeErrorKind;
use super::event::Event;
use super::operators;
use super::value::Value;
use crate::ast::BinaryOperator;
use crate::builtin::Builtin;

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
pub(super) enum Flow {
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
pub(super) struct Memory {
    frame: Frame,
    /// The values the expression being evaluated has left for its operators, above the turns
    /// left of the `repeat`s around it.
    operands: Vec<Value>,
}

impl Memory {
    /// The index of the next instruction to run.
    pub(super) const fn next(&self) -> usize {
        self.frame.next
    }

    /// The numbers of operands and of variables alive, which a loop must not let grow.
    #[cfg(test)]
    pub(super) const fn sizes(&self) -> (usize, usize) {
        (self.operands.len(), self.frame.variables.len())
    }

    /// Runs `instruction`, the next one, which the program moves past first: a jump then
    /// says where it goes on.
    pub(super) fn execute(
        &mut self,
        instruction: &Instruction,
        event: &mut Option<Event>,
    ) -> Result<Flow, RuntimeErrorKind> {
        self.frame.next = self.frame.next.saturating_add(1);
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
