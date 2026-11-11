//! The variables the checker knows at a place of the program: those of the main program, or
//! those of one function, which sees none of the others.

use std::collections::{HashMap, HashSet};

use super::ReservedWord;
use crate::ast::{Branch, Statement, StatementKind};
use crate::span::Span;

/// Why a name is no variable at the place where it is read or changed. A case that knows
/// the variable meant holds its name in its `let`, for the line the message gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Missing {
    /// A variable whose `let` is further down, in the block or in a block around it.
    BeforeLet(Span),
    /// A variable whose block has ended.
    AfterBlock(Span),
    /// No variable that this place could have seen.
    Unknown,
}

/// A block that is still open.
#[derive(Debug, Default)]
struct Block<'a> {
    /// The first `let` of each name written in the block itself, read when the block opens:
    /// what tells a variable used before its `let` from an unknown name.
    lets: HashMap<&'a str, Span>,
    /// The variables declared so far, parameters included.
    declared: HashMap<&'a str, Span>,
}

/// The variables of the main program, or of one function.
#[derive(Debug, Default)]
pub(super) struct Variables<'a> {
    /// The open blocks, the innermost last.
    blocks: Vec<Block<'a>>,
    /// The variables whose block has ended. When several had the same name, the one whose
    /// block ended last, the closest above.
    ended: HashMap<&'a str, Span>,
}

impl<'a> Variables<'a> {
    /// Opens the block made of `statements`.
    pub(super) fn open(&mut self, statements: &'a [Statement]) {
        let mut lets = HashMap::new();
        for (name, span) in declared_names(statements) {
            lets.entry(name).or_insert(span);
        }
        self.blocks.push(Block {
            lets,
            declared: HashMap::new(),
        });
    }

    /// Closes the innermost block: its variables exist no more.
    pub(super) fn close(&mut self) {
        if let Some(block) = self.blocks.pop() {
            self.ended.extend(block.declared);
        }
    }

    /// Declares a variable in the innermost block.
    pub(super) fn declare(&mut self, name: &'a str, span: Span) {
        if let Some(block) = self.blocks.last_mut() {
            block.declared.insert(name, span);
        }
    }

    /// Where the variable `name` that exists here was declared, if there is one.
    pub(super) fn declared(&self, name: &str) -> Option<Span> {
        self.blocks
            .iter()
            .rev()
            .find_map(|block| block.declared.get(name).copied())
    }

    /// Why `name` is no variable here, or `None` when it is one. When several reasons hold,
    /// the variable that belongs to this place comes first: one declared further down, then
    /// one whose block has ended.
    pub(super) fn missing(&self, name: &str) -> Option<Missing> {
        if self.declared(name).is_some() {
            return None;
        }
        let later = self
            .blocks
            .iter()
            .rev()
            .find_map(|block| block.lets.get(name).copied());
        Some(if let Some(span) = later {
            Missing::BeforeLet(span)
        } else if let Some(span) = self.ended.get(name).copied() {
            Missing::AfterBlock(span)
        } else {
            Missing::Unknown
        })
    }

    /// Takes the variables of a function that has ended, `function`, as variables whose block
    /// has ended: the body of a function is a block. Its parameters, still open, are left out.
    pub(super) fn absorb(&mut self, function: Self) {
        self.ended.extend(function.ended);
    }
}

/// The name of every `let` written in `statements` themselves, not in their blocks, with its
/// place. The reserved names are left out: they never become variables. A precaution: the
/// walk never asks for a reserved name, and the lists stay right if it ever does.
fn declared_names(statements: &[Statement]) -> impl Iterator<Item = (&str, Span)> {
    statements
        .iter()
        .filter_map(|statement| match &statement.kind {
            StatementKind::Let { name, value: _ }
                if ReservedWord::from_word(&name.text).is_none() =>
            {
                Some((name.text.as_str(), name.span))
            }
            _ => None,
        })
}

/// Adds to `names` every variable of the main program made of `statements`, wherever its
/// `let` is, in a block or not. The variables of the functions are not among them.
pub(super) fn main_program_variables<'a>(
    statements: &'a [Statement],
    names: &mut HashSet<&'a str>,
) {
    names.extend(declared_names(statements).map(|(name, _)| name));
    for statement in statements {
        match &statement.kind {
            StatementKind::If {
                branches,
                otherwise,
            } => {
                for Branch { body, .. } in branches {
                    main_program_variables(body, names);
                }
                if let Some(body) = otherwise {
                    main_program_variables(body, names);
                }
            }
            StatementKind::While { condition: _, body }
            | StatementKind::Repeat { count: _, body } => main_program_variables(body, names),
            StatementKind::Let { .. }
            | StatementKind::Assign { .. }
            | StatementKind::Break
            | StatementKind::Return { .. }
            | StatementKind::Function { .. }
            | StatementKind::Expression(_) => {}
        }
    }
}
