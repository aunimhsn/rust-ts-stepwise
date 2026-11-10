//! The errors of the lexer: a code with its values and its place, never a sentence, so that
//! the interface writes the message in the language of the learner.

use super::token::Keyword;
use crate::span::Span;

/// The first error found in the source, which stops the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    /// What is wrong.
    pub kind: LexErrorKind,
    /// The stretch of the source to underline.
    pub span: Span,
}

/// What is wrong in the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    /// A tab outside a text or a comment: the indentation uses spaces only.
    Tab,
    /// A line whose indentation is neither the one of an enclosing line nor four more spaces
    /// than the line above: `below` and `above` are the two closest levels it could take.
    ///
    /// Indented further than the line above, `below` is the level of that line and `above`
    /// four spaces more; indented less, they are the levels of the two enclosing lines around
    /// it (`6` under lines at `0`, `4` and `8` gives `4` and `8`).
    WrongIndent {
        /// The indentation found, in spaces.
        found: u32,
        /// The closest level allowed below it, in spaces.
        below: u32,
        /// The closest level allowed above it, in spaces.
        above: u32,
    },
    /// A text whose closing `"` is missing on its line.
    UnclosedText,
    /// A `\` followed, inside a text, by a character other than `"`, `\` or `n`.
    UnknownEscape {
        /// The character after the `\`.
        escape: char,
    },
    /// A `'`: texts are written between double quotes.
    SingleQuote,
    /// A whole number beyond the 64-bit range (above 9 223 372 036 854 775 807).
    NumberTooLarge,
    /// A word that starts with a digit, such as `2nd_side`: a name starts with a letter.
    NameStartsWithDigit {
        /// The whole word.
        word: String,
    },
    /// A symbol of another language where Stepwise has a keyword: `&&` or `&` for `and`,
    /// `||` or `|` for `or`, `!` for `not`.
    SymbolForKeyword {
        /// The symbol as written.
        symbol: &'static str,
        /// The keyword to write instead.
        keyword: Keyword,
    },
    /// A character that has no meaning outside a text or a comment, such as `@`, `;` or a
    /// non-breaking space.
    UnexpectedCharacter {
        /// The character.
        character: char,
    },
}
