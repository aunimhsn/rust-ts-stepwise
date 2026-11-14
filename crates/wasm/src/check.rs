//! The checks before running, for the interface (see *Checks before running* in `specs.md`):
//! [`check`], and the errors it gives, each a code with its values and its place.

use serde::Serialize;
use stepwise_core::checker::{CheckError, CheckErrorKind, ReservedWord};
use stepwise_core::lexer::{LexError, LexErrorKind};
use stepwise_core::parser::{ParseError, ParseErrorKind, SyntaxError};
use stepwise_core::program::{self, Refused};
use tsify::{Ts, Tsify};
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::position::{Offsets, Span};

/// Checks a program before it runs, as the learner types and again when **Run** is pressed.
///
/// It gives nothing for a program that can run; else its first syntax error alone or, once
/// its syntax is right, every error of its names and of its structure, in the order of the
/// source.
///
/// # Errors
///
/// It throws when an error can't be made a JavaScript object, which none of them does.
#[wasm_bindgen]
pub fn check(source: &str) -> Result<Vec<Ts<ProgramError>>, JsError> {
    errors(source)
        .iter()
        .map(|error| Ok(Ts::from_rust(error)?))
        .collect()
}

/// The errors of `source`, as [`check`] gives them, before they cross to JavaScript.
#[must_use]
pub fn errors(source: &str) -> Vec<ProgramError> {
    let Err(refused) = program::check(source) else {
        return Vec::new();
    };
    let offsets = Offsets::new(source);
    match refused {
        Refused::Syntax(SyntaxError::Lex(error)) => vec![ProgramError::lex(error, &offsets)],
        Refused::Syntax(SyntaxError::Parse(error)) => vec![ProgramError::parse(&error, &offsets)],
        Refused::Check(errors) => errors
            .into_iter()
            .map(|error| ProgramError::check(error, &offsets))
            .collect(),
    }
}

/// An error found by the checks before running: its `code`, the stretch of the source to
/// underline, and the values its message needs.
///
/// Each variant is the error of the same name in the core, where it is described: those of
/// `LexErrorKind`, of `ParseErrorKind`, then of `CheckErrorKind`. No name is in two of them.
/// The words of the language (a keyword, a built-in function, a color) are given as they are
/// written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Tsify)]
#[serde(tag = "code", rename_all = "kebab-case")]
pub enum ProgramError {
    // The characters of the source: the lexer.
    Tab {
        span: Span,
    },
    WrongIndent {
        span: Span,
        found: u32,
        below: u32,
        above: u32,
    },
    UnclosedText {
        span: Span,
    },
    UnknownEscape {
        span: Span,
        escape: char,
    },
    SingleQuote {
        span: Span,
    },
    NumberTooLarge {
        span: Span,
    },
    NameStartsWithDigit {
        span: Span,
        word: String,
    },
    SymbolForKeyword {
        span: Span,
        symbol: String,
        keyword: String,
    },
    UnexpectedCharacter {
        span: Span,
        character: char,
    },

    // The order of its words: the parser.
    ExpectedValue {
        span: Span,
    },
    TrailingComma {
        span: Span,
    },
    NotAfterOperator {
        span: Span,
    },
    UnexpectedToken {
        span: Span,
    },
    UnclosedParen {
        span: Span,
    },
    UnmatchedParen {
        span: Span,
    },
    AssignInExpression {
        span: Span,
    },
    ChainedComparison {
        span: Span,
    },
    ForeignWord {
        span: Span,
        word: String,
    },
    ExpectedName {
        span: Span,
    },
    KeywordAsName {
        span: Span,
        keyword: String,
    },
    ExpectedEqual {
        span: Span,
    },
    UnexpectedIndent {
        span: Span,
    },
    ExpectedColon {
        span: Span,
    },
    AfterColon {
        span: Span,
    },
    EmptyBlock {
        span: Span,
    },
    CommentOnlyBlock {
        span: Span,
    },
    ElseWithoutIf {
        span: Span,
    },
    ElseAfterElse {
        span: Span,
    },
    ExpectedFunctionName {
        span: Span,
    },
    ExpectedParentheses {
        span: Span,
    },
    InvalidParameter {
        span: Span,
    },
    TooDeep {
        span: Span,
    },

    // Its names and its structure: the checker.
    BreakOutsideLoop {
        span: Span,
    },
    ReturnOutsideFunction {
        span: Span,
    },
    NestedFunction {
        span: Span,
    },
    NotSnakeCase {
        span: Span,
    },
    ReservedName {
        span: Span,
        word: String,
        kind: ReservedKind,
    },
    DuplicateParameter {
        span: Span,
        name: String,
    },
    UnknownName {
        span: Span,
        name: String,
    },
    UndeclaredVariableChanged {
        span: Span,
        name: String,
    },
    VariableBeforeLet {
        span: Span,
        name: String,
        declared: Span,
    },
    VariableAfterBlock {
        span: Span,
        name: String,
        declared: Span,
    },
    MainVariableInFunction {
        span: Span,
        name: String,
    },
    AlreadyDeclared {
        span: Span,
        name: String,
        first: Span,
    },
    DuplicateFunction {
        span: Span,
        name: String,
        first: Span,
    },
    NameOfFunction {
        span: Span,
        name: String,
        defined: Span,
    },
    CalledBeforeDefinition {
        span: Span,
        name: String,
        defined: Span,
    },
    WrongArgumentCount {
        span: Span,
        name: String,
        expected: u32,
        given: u32,
    },
    FunctionWithoutParentheses {
        span: Span,
        name: String,
    },
    VariableCalled {
        span: Span,
        name: String,
    },
    UnknownFunction {
        span: Span,
        name: String,
    },
    ExpressionAlone {
        span: Span,
    },
}

/// What a reserved word belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Tsify)]
#[serde(rename_all = "kebab-case")]
pub enum ReservedKind {
    /// A built-in function.
    Function,
    /// A color.
    Color,
}

impl ProgramError {
    fn lex(error: LexError, offsets: &Offsets) -> Self {
        use LexErrorKind as K;
        let span = offsets.span(error.span);
        match error.kind {
            K::Tab => Self::Tab { span },
            K::WrongIndent {
                found,
                below,
                above,
            } => Self::WrongIndent {
                span,
                found,
                below,
                above,
            },
            K::UnclosedText => Self::UnclosedText { span },
            K::UnknownEscape { escape } => Self::UnknownEscape { span, escape },
            K::SingleQuote => Self::SingleQuote { span },
            K::NumberTooLarge => Self::NumberTooLarge { span },
            K::NameStartsWithDigit { word } => Self::NameStartsWithDigit { span, word },
            K::SymbolForKeyword { symbol, keyword } => Self::SymbolForKeyword {
                span,
                symbol: symbol.to_owned(),
                keyword: keyword.as_str().to_owned(),
            },
            K::UnexpectedCharacter { character } => Self::UnexpectedCharacter { span, character },
        }
    }

    fn parse(error: &ParseError, offsets: &Offsets) -> Self {
        use ParseErrorKind as K;
        let span = offsets.span(error.span);
        match error.kind {
            K::ExpectedValue => Self::ExpectedValue { span },
            K::TrailingComma => Self::TrailingComma { span },
            K::NotAfterOperator => Self::NotAfterOperator { span },
            K::UnexpectedToken => Self::UnexpectedToken { span },
            K::UnclosedParen => Self::UnclosedParen { span },
            K::UnmatchedParen => Self::UnmatchedParen { span },
            K::AssignInExpression => Self::AssignInExpression { span },
            K::ChainedComparison => Self::ChainedComparison { span },
            K::ForeignWord { word } => Self::ForeignWord {
                span,
                word: word.as_str().to_owned(),
            },
            K::ExpectedName => Self::ExpectedName { span },
            K::KeywordAsName { keyword } => Self::KeywordAsName {
                span,
                keyword: keyword.as_str().to_owned(),
            },
            K::ExpectedEqual => Self::ExpectedEqual { span },
            K::UnexpectedIndent => Self::UnexpectedIndent { span },
            K::ExpectedColon => Self::ExpectedColon { span },
            K::AfterColon => Self::AfterColon { span },
            K::EmptyBlock => Self::EmptyBlock { span },
            K::CommentOnlyBlock => Self::CommentOnlyBlock { span },
            K::ElseWithoutIf => Self::ElseWithoutIf { span },
            K::ElseAfterElse => Self::ElseAfterElse { span },
            K::ExpectedFunctionName => Self::ExpectedFunctionName { span },
            K::ExpectedParentheses => Self::ExpectedParentheses { span },
            K::InvalidParameter => Self::InvalidParameter { span },
            K::TooDeep => Self::TooDeep { span },
        }
    }

    fn check(error: CheckError, offsets: &Offsets) -> Self {
        use CheckErrorKind as K;
        let span = offsets.span(error.span);
        match error.kind {
            K::BreakOutsideLoop => Self::BreakOutsideLoop { span },
            K::ReturnOutsideFunction => Self::ReturnOutsideFunction { span },
            K::NestedFunction => Self::NestedFunction { span },
            K::NotSnakeCase => Self::NotSnakeCase { span },
            K::ReservedName { word } => Self::ReservedName {
                span,
                word: word.as_str().to_owned(),
                kind: match word {
                    ReservedWord::Builtin(_) => ReservedKind::Function,
                    ReservedWord::Color(_) => ReservedKind::Color,
                },
            },
            K::DuplicateParameter { name } => Self::DuplicateParameter { span, name },
            K::UnknownName { name } => Self::UnknownName { span, name },
            K::UndeclaredVariableChanged { name } => Self::UndeclaredVariableChanged { span, name },
            K::VariableBeforeLet { name, declared } => Self::VariableBeforeLet {
                span,
                name,
                declared: offsets.span(declared),
            },
            K::VariableAfterBlock { name, declared } => Self::VariableAfterBlock {
                span,
                name,
                declared: offsets.span(declared),
            },
            K::MainVariableInFunction { name } => Self::MainVariableInFunction { span, name },
            K::AlreadyDeclared { name, first } => Self::AlreadyDeclared {
                span,
                name,
                first: offsets.span(first),
            },
            K::DuplicateFunction { name, first } => Self::DuplicateFunction {
                span,
                name,
                first: offsets.span(first),
            },
            K::NameOfFunction { name, defined } => Self::NameOfFunction {
                span,
                name,
                defined: offsets.span(defined),
            },
            K::CalledBeforeDefinition { name, defined } => Self::CalledBeforeDefinition {
                span,
                name,
                defined: offsets.span(defined),
            },
            K::WrongArgumentCount {
                name,
                expected,
                given,
            } => Self::WrongArgumentCount {
                span,
                name,
                expected: count(expected),
                given: count(given),
            },
            K::FunctionWithoutParentheses { name } => {
                Self::FunctionWithoutParentheses { span, name }
            }
            K::VariableCalled { name } => Self::VariableCalled { span, name },
            K::UnknownFunction { name } => Self::UnknownFunction { span, name },
            K::ExpressionAlone => Self::ExpressionAlone { span },
        }
    }
}

/// A count of the core as a number that JavaScript holds exactly.
fn count(count: usize) -> u32 {
    u32::try_from(count).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use stepwise_core::builtin::Builtin;
    use stepwise_core::checker::{CheckError, CheckErrorKind, ReservedWord};
    use stepwise_core::color::Color;
    use stepwise_core::lexer::{Keyword, LexError, LexErrorKind};
    use stepwise_core::parser::{ForeignWord, ParseError, ParseErrorKind};
    use stepwise_core::span::{self, Position};

    use super::{ProgramError, ReservedKind, errors};
    use crate::position::{Offsets, Span};

    const fn at(line: u32, from: u32, to: u32) -> Span {
        Span { line, from, to }
    }

    #[test]
    fn a_program_that_can_run_has_no_error() {
        assert_eq!(errors("let sides = 4\nrepeat sides:\n    forward()\n"), []);
        assert_eq!(errors(""), []);
    }

    #[test]
    fn the_first_syntax_error_comes_alone() {
        // `sidse` and `turn` are unknown, but nothing is read reliably around the line 2.
        assert_eq!(
            errors("sidse = 1\nforward(\nturn()\n"),
            [ProgramError::UnclosedParen {
                span: at(2, 17, 18)
            }]
        );
    }

    #[test]
    fn an_error_of_the_lexer_keeps_its_values() {
        assert_eq!(
            errors("if done:\n      forward()\n"),
            [ProgramError::WrongIndent {
                span: at(2, 9, 15),
                found: 6,
                below: 0,
                above: 4,
            }]
        );
        assert_eq!(
            errors("print(\"côté\\q\")\n"),
            [ProgramError::UnknownEscape {
                span: at(1, 11, 13),
                escape: 'q',
            }]
        );
        assert_eq!(
            errors("if a && b:\n    forward()\n"),
            [ProgramError::SymbolForKeyword {
                span: at(1, 5, 7),
                symbol: "&&".to_owned(),
                keyword: "and".to_owned(),
            }]
        );
    }

    #[test]
    fn an_error_of_the_parser_keeps_its_values() {
        assert_eq!(
            errors("let repeat = 3\n"),
            [ProgramError::KeywordAsName {
                span: at(1, 4, 10),
                keyword: "repeat".to_owned(),
            }]
        );
        assert_eq!(
            errors("if a:\n    forward()\nelif b:\n    forward()\n"),
            [ProgramError::ForeignWord {
                span: at(3, 20, 24),
                word: "elif".to_owned(),
            }]
        );
    }

    #[test]
    fn something_missing_at_the_end_of_a_line_is_a_place_between_two_characters() {
        assert_eq!(
            errors("let sides =\nforward()\n"),
            [ProgramError::ExpectedValue {
                span: at(1, 11, 11)
            }]
        );
    }

    #[test]
    fn every_error_of_names_comes_in_the_order_of_the_source_with_its_places() {
        // The emoji of the line 2 counts for two in the places of the line 3.
        let source = "print(côté)\nlet côté = \"🤖\"\nforward(côté)\n";
        assert_eq!(
            errors(source),
            [
                ProgramError::VariableBeforeLet {
                    span: at(1, 6, 10),
                    name: "côté".to_owned(),
                    declared: at(2, 16, 20),
                },
                ProgramError::NotSnakeCase {
                    span: at(2, 16, 20)
                },
                ProgramError::WrongArgumentCount {
                    span: at(3, 28, 35),
                    name: "forward".to_owned(),
                    expected: 0,
                    given: 1,
                },
            ]
        );
    }

    #[test]
    fn a_reserved_name_says_what_its_word_belongs_to() {
        assert_eq!(
            errors("let red = 3\nlet forward = 4\n"),
            [
                ProgramError::ReservedName {
                    span: at(1, 4, 7),
                    word: "red".to_owned(),
                    kind: ReservedKind::Color,
                },
                ProgramError::ReservedName {
                    span: at(2, 16, 23),
                    word: "forward".to_owned(),
                    kind: ReservedKind::Function,
                },
            ]
        );
    }

    /// The name of a variant, as `Debug` writes it first.
    fn variant(value: &impl std::fmt::Debug) -> String {
        format!("{value:?}")
            .chars()
            .take_while(char::is_ascii_alphanumeric)
            .collect()
    }

    /// A source and a stretch of it, for the errors built by hand.
    fn sample() -> (Offsets, span::Span) {
        let offsets = Offsets::new("let sides = 4\nsides = sides + 1\n");
        let span = span::Span::new(Position::new(2, 1), Position::new(2, 6));
        (offsets, span)
    }

    // Each error of the bridge is the error of the same name in the core: a line of the
    // conversions that gave another one would give the learner the message of another error.

    #[test]
    fn every_error_of_the_lexer_becomes_the_error_of_its_name() {
        let (offsets, span) = sample();
        let lex = [
            LexErrorKind::Tab,
            LexErrorKind::WrongIndent {
                found: 6,
                below: 4,
                above: 8,
            },
            LexErrorKind::UnclosedText,
            LexErrorKind::UnknownEscape { escape: 'q' },
            LexErrorKind::SingleQuote,
            LexErrorKind::NumberTooLarge,
            LexErrorKind::NameStartsWithDigit {
                word: "2nd".to_owned(),
            },
            LexErrorKind::SymbolForKeyword {
                symbol: "!",
                keyword: Keyword::Not,
            },
            LexErrorKind::UnexpectedCharacter { character: '@' },
        ];
        for kind in lex {
            let expected = variant(&kind);
            let error = ProgramError::lex(LexError { kind, span }, &offsets);
            assert_eq!(variant(&error), expected);
        }
    }

    #[test]
    fn every_error_of_the_parser_becomes_the_error_of_its_name() {
        let (offsets, span) = sample();
        let parse = [
            ParseErrorKind::ExpectedValue,
            ParseErrorKind::TrailingComma,
            ParseErrorKind::NotAfterOperator,
            ParseErrorKind::UnexpectedToken,
            ParseErrorKind::UnclosedParen,
            ParseErrorKind::UnmatchedParen,
            ParseErrorKind::AssignInExpression,
            ParseErrorKind::ChainedComparison,
            ParseErrorKind::ForeignWord {
                word: ForeignWord::Def,
            },
            ParseErrorKind::ExpectedName,
            ParseErrorKind::KeywordAsName {
                keyword: Keyword::While,
            },
            ParseErrorKind::ExpectedEqual,
            ParseErrorKind::UnexpectedIndent,
            ParseErrorKind::ExpectedColon,
            ParseErrorKind::AfterColon,
            ParseErrorKind::EmptyBlock,
            ParseErrorKind::CommentOnlyBlock,
            ParseErrorKind::ElseWithoutIf,
            ParseErrorKind::ElseAfterElse,
            ParseErrorKind::ExpectedFunctionName,
            ParseErrorKind::ExpectedParentheses,
            ParseErrorKind::InvalidParameter,
            ParseErrorKind::TooDeep,
        ];
        for kind in parse {
            let expected = variant(&kind);
            let error = ProgramError::parse(&ParseError { kind, span }, &offsets);
            assert_eq!(variant(&error), expected);
        }
    }

    #[test]
    fn every_error_of_the_checker_becomes_the_error_of_its_name() {
        let (offsets, span) = sample();
        let name = || "sides".to_owned();
        let check = [
            CheckErrorKind::BreakOutsideLoop,
            CheckErrorKind::ReturnOutsideFunction,
            CheckErrorKind::NestedFunction,
            CheckErrorKind::NotSnakeCase,
            CheckErrorKind::ReservedName {
                word: ReservedWord::Builtin(Builtin::Forward),
            },
            CheckErrorKind::ReservedName {
                word: ReservedWord::Color(Color::Red),
            },
            CheckErrorKind::DuplicateParameter { name: name() },
            CheckErrorKind::UnknownName { name: name() },
            CheckErrorKind::UndeclaredVariableChanged { name: name() },
            CheckErrorKind::VariableBeforeLet {
                name: name(),
                declared: span,
            },
            CheckErrorKind::VariableAfterBlock {
                name: name(),
                declared: span,
            },
            CheckErrorKind::MainVariableInFunction { name: name() },
            CheckErrorKind::AlreadyDeclared {
                name: name(),
                first: span,
            },
            CheckErrorKind::DuplicateFunction {
                name: name(),
                first: span,
            },
            CheckErrorKind::NameOfFunction {
                name: name(),
                defined: span,
            },
            CheckErrorKind::CalledBeforeDefinition {
                name: name(),
                defined: span,
            },
            CheckErrorKind::WrongArgumentCount {
                name: name(),
                expected: 1,
                given: 2,
            },
            CheckErrorKind::FunctionWithoutParentheses { name: name() },
            CheckErrorKind::VariableCalled { name: name() },
            CheckErrorKind::UnknownFunction { name: name() },
            CheckErrorKind::ExpressionAlone,
        ];
        for kind in check {
            let expected = variant(&kind);
            let error = ProgramError::check(CheckError { kind, span }, &offsets);
            assert_eq!(variant(&error), expected);
        }
    }

    #[test]
    fn the_places_kept_in_an_error_are_converted_as_its_own() {
        let source = "let 🤖 = 4\nsides = 1\n";
        let offsets = Offsets::new(source);
        let span = span::Span::new(Position::new(2, 1), Position::new(2, 6));
        let other = span::Span::new(Position::new(1, 7), Position::new(1, 8));
        let name = || "sides".to_owned();
        let (place, kept) = (at(2, 11, 16), at(1, 7, 8));
        let convert = |kind| ProgramError::check(CheckError { kind, span }, &offsets);
        assert_eq!(
            convert(CheckErrorKind::VariableAfterBlock {
                name: name(),
                declared: other
            }),
            ProgramError::VariableAfterBlock {
                span: place,
                name: name(),
                declared: kept
            }
        );
        assert_eq!(
            convert(CheckErrorKind::AlreadyDeclared {
                name: name(),
                first: other
            }),
            ProgramError::AlreadyDeclared {
                span: place,
                name: name(),
                first: kept
            }
        );
        assert_eq!(
            convert(CheckErrorKind::DuplicateFunction {
                name: name(),
                first: other
            }),
            ProgramError::DuplicateFunction {
                span: place,
                name: name(),
                first: kept
            }
        );
        assert_eq!(
            convert(CheckErrorKind::NameOfFunction {
                name: name(),
                defined: other
            }),
            ProgramError::NameOfFunction {
                span: place,
                name: name(),
                defined: kept
            }
        );
        assert_eq!(
            convert(CheckErrorKind::CalledBeforeDefinition {
                name: name(),
                defined: other
            }),
            ProgramError::CalledBeforeDefinition {
                span: place,
                name: name(),
                defined: kept
            }
        );
    }
}
