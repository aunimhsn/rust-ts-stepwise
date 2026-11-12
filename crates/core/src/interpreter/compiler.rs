//! The compiler: translates the syntax tree of a checked program into the instructions of
//! the machine.
//!
//! It is the only recursion of the interpreter, on a tree whose depth the parser limits (see
//! [`MAX_DEPTH`](crate::parser::MAX_DEPTH)). It decides here, once, what the machine would
//! otherwise look up at each step: the slot of each variable, the target of each jump.

use std::rc::Rc;

use super::code::{Code, Instruction, PendingJump};
use super::value::Value;
use crate::ast::{
    BinaryOperator, Branch, Expression, ExpressionKind, Name, Program, Statement, StatementKind,
};
use crate::builtin::Builtin;
use crate::color::Color;
use crate::span::{Position, Span};

/// A program that can't be translated, with the place that stops the compiler: something the
/// checks before running refuse, so the program wasn't checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct NotChecked(pub(super) Span);

/// The instructions of `program`, which the checker must have found without error.
pub(super) fn compile(program: &Program) -> Result<Code, NotChecked> {
    let mut compiler = Compiler::default();
    // The variables of the main program stay until its end: no block closes around them.
    for statement in &program.statements {
        compiler.statement(statement)?;
    }
    let start = Span::at(Position::new(1, 1));
    let last_line = program.statements.last().map_or(start, |last| last.span);
    compiler.code.push(Instruction::End, last_line);
    Ok(compiler.code)
}

/// A loop around the place the compiler has reached.
#[derive(Debug)]
struct Loop {
    /// The number of variables alive where the loop starts: a `break` forgets the others.
    variables: usize,
    /// Whether the loop keeps its turns left on the operands, as a `repeat` does: a `break`
    /// drops them.
    counted: bool,
    /// The jumps of its `break`s, which land after the loop.
    breaks: Vec<PendingJump>,
}

/// The instructions written so far, and what the compiler knows of the place it has reached.
#[derive(Debug, Default)]
struct Compiler<'a> {
    code: Code,
    /// The variables alive at this place, each at the index of its slot.
    variables: Vec<&'a str>,
    /// The loops around this place, the innermost last.
    loops: Vec<Loop>,
}

impl<'a> Compiler<'a> {
    /// The statements of a block, whose line is `span`: the variables it declares end with
    /// it.
    fn block(&mut self, statements: &'a [Statement], span: Span) -> Result<(), NotChecked> {
        let outside = self.variables.len();
        for statement in statements {
            self.statement(statement)?;
        }
        if self.variables.len() > outside {
            self.variables.truncate(outside);
            self.code.push(Instruction::Truncate(outside), span);
        }
        Ok(())
    }

    fn statement(&mut self, statement: &'a Statement) -> Result<(), NotChecked> {
        let span = statement.span;
        match &statement.kind {
            StatementKind::Let { name, value } => {
                self.code.push(Instruction::Line, span);
                // The value is translated before the variable exists: it can't read it.
                self.expression(value)?;
                let declare = Instruction::Declare(Rc::from(name.text.as_str()));
                self.code.push(declare, name.span);
                self.variables.push(&name.text);
            }
            StatementKind::Assign { name, value } => {
                self.code.push(Instruction::Line, span);
                self.expression(value)?;
                let slot = self.slot(&name.text).ok_or(NotChecked(name.span))?;
                self.code.push(Instruction::Store(slot), name.span);
            }
            StatementKind::Break => {
                self.code.push(Instruction::Line, span);
                self.leave_loop(span)?;
            }
            StatementKind::If {
                branches,
                otherwise,
            } => self.conditional(branches, otherwise.as_deref(), span)?,
            StatementKind::While { condition, body } => self.while_loop(condition, body, span)?,
            StatementKind::Repeat { count, body } => self.repeat_loop(count, body, span)?,
            StatementKind::Expression(expression) => {
                self.code.push(Instruction::Line, span);
                if let ExpressionKind::Call {
                    function,
                    arguments,
                } = &expression.kind
                {
                    self.call(function, arguments, expression.span, false)?;
                } else {
                    self.expression(expression)?;
                    self.code.push(Instruction::Pop, span);
                }
            }
            // The line of a definition is no step, and nothing runs when the program reaches
            // it. Until step 9b, its block isn't translated: no call can enter it.
            StatementKind::Function { .. } => {}
            // Until step 9b: a `return` is in a function, whose block isn't translated.
            StatementKind::Return { .. } => return Err(NotChecked(span)),
        }
        Ok(())
    }

    /// An `if` with its `else if`s and its `else`: the line of each condition evaluated is a
    /// step, the line of the `else` never is.
    fn conditional(
        &mut self,
        branches: &'a [Branch],
        otherwise: Option<&'a [Statement]>,
        span: Span,
    ) -> Result<(), NotChecked> {
        let mut ends = Vec::new();
        for branch in branches {
            self.code.push(Instruction::Line, branch.span);
            self.expression(&branch.condition)?;
            let next = self
                .code
                .jump(Instruction::JumpUnless, branch.condition.span);
            self.block(&branch.body, branch.span)?;
            ends.push(self.code.jump(Instruction::Jump, branch.span));
            self.code.land(next);
        }
        if let Some(body) = otherwise {
            self.block(body, span)?;
        }
        for end in ends {
            self.code.land(end);
        }
        Ok(())
    }

    /// A `while`: its line is a step before each turn, and a last time when its condition
    /// is `false`.
    fn while_loop(
        &mut self,
        condition: &'a Expression,
        body: &'a [Statement],
        span: Span,
    ) -> Result<(), NotChecked> {
        let start = self.code.here();
        self.code.push(Instruction::Line, span);
        self.expression(condition)?;
        let exit = self.code.jump(Instruction::JumpUnless, condition.span);
        self.enter_loop(false);
        self.block(body, span)?;
        // Back to the line of the loop: a jump back always meets a line, where the step ends.
        self.code.push(Instruction::Jump(start), span);
        self.code.land(exit);
        self.land_breaks();
        Ok(())
    }

    /// A `repeat`: its count is evaluated once, then its line is a step again after each
    /// turn, the last time to find that no turn is left.
    fn repeat_loop(
        &mut self,
        count: &'a Expression,
        body: &'a [Statement],
        span: Span,
    ) -> Result<(), NotChecked> {
        self.code.push(Instruction::Line, span);
        self.expression(count)?;
        self.code.push(Instruction::RepeatStart, count.span);
        let turn = self.code.here();
        let exit = self.code.jump(Instruction::RepeatNext, span);
        self.enter_loop(true);
        self.block(body, span)?;
        // The line of the loop again, then back to its turns: a jump back always meets a
        // line, where the step ends.
        self.code.push(Instruction::Line, span);
        self.code.push(Instruction::Jump(turn), span);
        self.code.land(exit);
        self.land_breaks();
        Ok(())
    }

    /// Opens a loop at this place, before its block.
    fn enter_loop(&mut self, counted: bool) {
        self.loops.push(Loop {
            variables: self.variables.len(),
            counted,
            breaks: Vec::new(),
        });
    }

    /// Closes the innermost loop: its `break`s land here, after it.
    fn land_breaks(&mut self) {
        for jump in self
            .loops
            .pop()
            .map(|ended| ended.breaks)
            .unwrap_or_default()
        {
            self.code.land(jump);
        }
    }

    /// A `break`: forgets what the innermost loop holds at this place, then jumps after it.
    /// The variables stay known to the compiler, for the lines of the block under the `break`.
    fn leave_loop(&mut self, span: Span) -> Result<(), NotChecked> {
        let Some(&Loop {
            variables, counted, ..
        }) = self.loops.last()
        else {
            return Err(NotChecked(span));
        };
        if self.variables.len() > variables {
            self.code.push(Instruction::Truncate(variables), span);
        }
        if counted {
            self.code.push(Instruction::Pop, span);
        }
        let jump = self.code.jump(Instruction::Jump, span);
        if let Some(innermost) = self.loops.last_mut() {
            innermost.breaks.push(jump);
        }
        Ok(())
    }

    /// An expression, which leaves its value as the last operand.
    fn expression(&mut self, expression: &'a Expression) -> Result<(), NotChecked> {
        let span = expression.span;
        match &expression.kind {
            ExpressionKind::Number(number) => self.push_value(Value::Number(*number), span),
            ExpressionKind::Boolean(boolean) => self.push_value(Value::Boolean(*boolean), span),
            ExpressionKind::Text(text) => {
                self.push_value(Value::Text(Rc::from(text.as_str())), span);
            }
            ExpressionKind::Name(name) => {
                if let Some(slot) = self.slot(name) {
                    self.code.push(Instruction::Load(slot), span);
                } else if let Some(color) = Color::from_word(name) {
                    self.push_value(Value::Color(color), span);
                } else {
                    return Err(NotChecked(span));
                }
            }
            ExpressionKind::Call {
                function,
                arguments,
            } => self.call(function, arguments, span, true)?,
            ExpressionKind::Unary { operator, operand } => {
                self.expression(operand)?;
                self.code.push(Instruction::Unary(*operator), span);
            }
            ExpressionKind::Binary {
                operator: operator @ (BinaryOperator::And | BinaryOperator::Or),
                left,
                right,
            } => {
                let (operator, decisive) = (*operator, *operator == BinaryOperator::Or);
                self.expression(left)?;
                let short_circuit = |end| Instruction::ShortCircuit {
                    operator,
                    decisive,
                    end,
                };
                let end = self.code.jump(short_circuit, left.span);
                self.expression(right)?;
                self.code
                    .push(Instruction::RequireBoolean(operator), right.span);
                self.code.land(end);
            }
            ExpressionKind::Binary {
                operator,
                left,
                right,
            } => {
                self.expression(left)?;
                self.expression(right)?;
                self.code.push(Instruction::Binary(*operator), span);
            }
        }
        Ok(())
    }

    /// A call, whose place is `span`: its values in order, then the function. Until step 9b,
    /// only the built-in functions are known.
    fn call(
        &mut self,
        function: &'a Name,
        arguments: &'a [Expression],
        span: Span,
        value_needed: bool,
    ) -> Result<(), NotChecked> {
        for argument in arguments {
            self.expression(argument)?;
        }
        let builtin = Builtin::from_word(&function.text).ok_or(NotChecked(function.span))?;
        let call = Instruction::CallBuiltin {
            builtin,
            arguments: arguments.len(),
            value_needed,
        };
        self.code.push(call, span);
        Ok(())
    }

    fn push_value(&mut self, value: Value, span: Span) {
        self.code.push(Instruction::Push(value), span);
    }

    /// The slot of the variable `name` alive at this place, if there is one.
    fn slot(&self, name: &str) -> Option<usize> {
        self.variables
            .iter()
            .rposition(|&variable| variable == name)
    }
}
