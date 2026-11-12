//! The instructions the machine runs: a program as one flat list, written by the compiler.
//!
//! The machine keeps its values in two places: the variables of the function being run, each
//! in a slot numbered in the order of the `let`s, and the operands, a stack where an
//! expression leaves its values until an operator or a statement takes them.

use std::rc::Rc;

use super::value::Value;
use crate::ast::{BinaryOperator, UnaryOperator};
use crate::builtin::Builtin;
use crate::span::Span;

/// The target of a jump written before the place it lands on is known: past the end of any
/// program, so that a jump left that way stops the machine rather than land anywhere.
const NOWHERE: usize = usize::MAX;

/// One thing the machine does. A target is the index of an instruction in the list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Instruction {
    /// The start of a line, where a step ends: the machine pauses, the line being the one
    /// to highlight. Its place is the line.
    Line,
    /// The end of the program.
    End,
    /// Puts a value on the operands.
    Push(Value),
    /// Puts the value of the variable of this slot on the operands.
    Load(usize),
    /// Takes the last operand as the new value of the variable of this slot.
    Store(usize),
    /// Takes the last operand as the value of a new variable of this name, in the next slot.
    Declare(Rc<str>),
    /// Forgets the variables beyond this number of them: the end of their block.
    Truncate(usize),
    /// Drops the last operand.
    Pop,
    /// Replaces the last operand with the result of the operator in front of it.
    Unary(UnaryOperator),
    /// Replaces the last two operands with the result of the operator between them.
    Binary(BinaryOperator),
    /// The left side of an `and` or of an `or`, the last operand, which must be a boolean:
    /// when it is `decisive`, it is the result and the machine jumps over the right side;
    /// otherwise it is dropped, and the right side gives the result.
    ShortCircuit {
        /// `and` or `or`, for the error.
        operator: BinaryOperator,
        /// The value of the left side that decides the result: `false` for `and`, `true` for
        /// `or`.
        decisive: bool,
        /// Where the machine goes on when the left side decides.
        end: usize,
    },
    /// The right side of an `and` or of an `or`, the last operand, which must be a boolean.
    RequireBoolean(BinaryOperator),
    /// Goes on at the target.
    Jump(usize),
    /// Takes the last operand, a condition, which must be a boolean: goes on at the target
    /// when it is `false`.
    JumpUnless(usize),
    /// The count of a `repeat`, the last operand, which must be a whole number, zero or more:
    /// it stays on the operands as the turns left, under what the block puts there.
    RepeatStart,
    /// Starts a turn of a `repeat`, the turns left being the last operand: one less when
    /// some are left, otherwise drops them and goes on at the target, after the loop.
    RepeatNext(usize),
    /// Calls a built-in function with the last operands as its values, in order.
    CallBuiltin {
        /// The function.
        builtin: Builtin,
        /// The number of values the call gives it.
        arguments: usize,
        /// Whether the call is in an expression, which needs its result.
        value_needed: bool,
    },
}

impl Instruction {
    /// Gives its target to a jump.
    const fn set_target(&mut self, target: usize) {
        match self {
            Self::Jump(to)
            | Self::JumpUnless(to)
            | Self::RepeatNext(to)
            | Self::ShortCircuit { end: to, .. } => *to = target,
            // Every instruction is listed, so that a new one that jumps can't be forgotten.
            Self::Line
            | Self::End
            | Self::Push(_)
            | Self::Load(_)
            | Self::Store(_)
            | Self::Declare(_)
            | Self::Truncate(_)
            | Self::Pop
            | Self::Unary(_)
            | Self::Binary(_)
            | Self::RequireBoolean(_)
            | Self::RepeatStart
            | Self::CallBuiltin { .. } => {}
        }
    }
}

/// A jump whose target isn't written yet: [`Code::land`] gives it the place reached then.
#[derive(Debug)]
#[must_use = "a jump that never lands stops the program"]
pub(super) struct PendingJump {
    /// The index of its instruction.
    index: usize,
}

/// A program as the machine runs it: its instructions, each with the stretch of the source
/// it comes from, where the machine places an error it raises.
#[derive(Debug, Default)]
pub(super) struct Code {
    instructions: Vec<(Instruction, Span)>,
}

impl Code {
    /// The instruction at `index`, with its place.
    pub(super) fn get(&self, index: usize) -> Option<&(Instruction, Span)> {
        self.instructions.get(index)
    }

    /// The index of the next instruction written: the target of a jump back to here.
    pub(super) const fn here(&self) -> usize {
        self.instructions.len()
    }

    /// Writes an instruction, which comes from `span`.
    pub(super) fn push(&mut self, instruction: Instruction, span: Span) {
        self.instructions.push((instruction, span));
    }

    /// Writes a jump forward, built by `jump` from its target, which [`Code::land`] gives
    /// later.
    pub(super) fn jump(
        &mut self,
        jump: impl FnOnce(usize) -> Instruction,
        span: Span,
    ) -> PendingJump {
        let index = self.here();
        self.push(jump(NOWHERE), span);
        PendingJump { index }
    }

    /// Makes `jump` land here, on the next instruction written.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "a jump lands once: landing takes it away"
    )]
    pub(super) fn land(&mut self, jump: PendingJump) {
        let here = self.here();
        if let Some((instruction, _)) = self.instructions.get_mut(jump.index) {
            instruction.set_target(here);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Code, Instruction, NOWHERE};
    use crate::ast::BinaryOperator;
    use crate::test_support::span;

    #[test]
    fn a_jump_lands_on_the_next_instruction_written() {
        let place = span(1, 1, 2);
        let short_circuit = |end| Instruction::ShortCircuit {
            operator: BinaryOperator::And,
            decisive: false,
            end,
        };
        for jump in [
            Instruction::Jump as fn(usize) -> Instruction,
            Instruction::JumpUnless,
            Instruction::RepeatNext,
            short_circuit,
        ] {
            let mut code = Code::default();
            code.push(Instruction::Line, place);
            let pending = code.jump(jump, place);
            assert_eq!(code.get(1), Some(&(jump(NOWHERE), place)));
            code.push(Instruction::Pop, place);
            code.land(pending);
            assert_eq!(code.get(1), Some(&(jump(3), place)));
        }
    }

    #[test]
    fn a_jump_that_never_lands_goes_past_the_end() {
        let mut code = Code::default();
        let pending = code.jump(Instruction::Jump, span(1, 1, 2));
        assert_eq!(
            code.get(pending.index).unwrap().0,
            Instruction::Jump(NOWHERE)
        );
        assert_eq!(code.get(NOWHERE), None);
    }
}
