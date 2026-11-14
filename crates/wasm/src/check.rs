//! The checks before running, for the interface (see *Checks before running* in `specs.md`):
//! [`check`], and the errors it gives, each a code with its values and its place.

use serde::Serialize;
use stepwise_core::checker::{CheckErrorKind, ReservedWord};
use stepwise_core::lexer::LexErrorKind;
use stepwise_core::parser::{ParseErrorKind, SyntaxError};
use stepwise_core::program::{self, Refused};
use stepwise_core::span;
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
    let error = |span: span::Span, kind| ProgramError {
        span: offsets.span(span),
        kind,
    };
    match refused {
        Refused::Syntax(SyntaxError::Lex(lex)) => {
            vec![error(lex.span, ProgramErrorKind::lex(lex.kind))]
        }
        Refused::Syntax(SyntaxError::Parse(parse)) => {
            vec![error(parse.span, ProgramErrorKind::parse(&parse.kind))]
        }
        Refused::Check(errors) => errors
            .into_iter()
            .map(|check| error(check.span, ProgramErrorKind::check(check.kind, &offsets)))
            .collect(),
    }
}

/// An error found by the checks before running, as the core gives it: where it is, and what
/// is wrong there.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Tsify)]
pub struct ProgramError {
    /// The stretch of the source to underline.
    pub span: Span,
    /// What is wrong there: its `code`, and the values its message needs.
    pub kind: ProgramErrorKind,
}

/// What the checks before running found wrong: a `code`, and the values its message needs
/// (`{ code: "unknown-name", name: "sidse" }`).
///
/// Each variant is the error of the same name in the core, where it is described: those of
/// `LexErrorKind`, of `ParseErrorKind`, then of `CheckErrorKind`. No name is in two of them.
/// The words of the language (a keyword, a built-in function, a color) are given as they are
/// written, and a place kept in an error as the place of the error itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Tsify)]
#[serde(
    tag = "code",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub enum ProgramErrorKind {
    // The characters of the source: the lexer.
    Tab,
    WrongIndent {
        found: u32,
        below: u32,
        above: u32,
    },
    UnclosedText,
    UnknownEscape {
        escape: char,
    },
    SingleQuote,
    NumberTooLarge,
    NameStartsWithDigit {
        word: String,
    },
    SymbolForKeyword {
        symbol: String,
        keyword: String,
    },
    UnexpectedCharacter {
        character: char,
    },

    // The order of its words: the parser.
    ExpectedValue,
    TrailingComma,
    NotAfterOperator,
    UnexpectedToken,
    UnclosedParen,
    UnmatchedParen,
    AssignInExpression,
    ChainedComparison,
    ForeignWord {
        word: String,
    },
    ExpectedName,
    KeywordAsName {
        keyword: String,
    },
    ExpectedEqual,
    UnexpectedIndent,
    ExpectedColon,
    AfterColon,
    EmptyBlock,
    CommentOnlyBlock,
    ElseWithoutIf,
    ElseAfterElse,
    ExpectedFunctionName,
    ExpectedParentheses,
    InvalidParameter,
    TooDeep,

    // Its names and its structure: the checker.
    BreakOutsideLoop,
    ReturnOutsideFunction,
    NestedFunction,
    NotSnakeCase,
    ReservedName {
        word: String,
        belongs_to: ReservedKind,
    },
    DuplicateParameter {
        name: String,
    },
    UnknownName {
        name: String,
    },
    UndeclaredVariableChanged {
        name: String,
    },
    VariableBeforeLet {
        name: String,
        declared: Span,
    },
    VariableAfterBlock {
        name: String,
        declared: Span,
    },
    MainVariableInFunction {
        name: String,
    },
    AlreadyDeclared {
        name: String,
        first: Span,
    },
    DuplicateFunction {
        name: String,
        first: Span,
    },
    NameOfFunction {
        name: String,
        defined: Span,
    },
    CalledBeforeDefinition {
        name: String,
        defined: Span,
    },
    WrongArgumentCount {
        name: String,
        expected: u32,
        given: u32,
    },
    FunctionWithoutParentheses {
        name: String,
    },
    VariableCalled {
        name: String,
    },
    UnknownFunction {
        name: String,
    },
    ExpressionAlone,
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

impl ProgramErrorKind {
    fn lex(kind: LexErrorKind) -> Self {
        use LexErrorKind as K;
        match kind {
            K::Tab => Self::Tab,
            K::WrongIndent {
                found,
                below,
                above,
            } => Self::WrongIndent {
                found,
                below,
                above,
            },
            K::UnclosedText => Self::UnclosedText,
            K::UnknownEscape { escape } => Self::UnknownEscape { escape },
            K::SingleQuote => Self::SingleQuote,
            K::NumberTooLarge => Self::NumberTooLarge,
            K::NameStartsWithDigit { word } => Self::NameStartsWithDigit { word },
            K::SymbolForKeyword { symbol, keyword } => Self::SymbolForKeyword {
                symbol: symbol.to_owned(),
                keyword: keyword.as_str().to_owned(),
            },
            K::UnexpectedCharacter { character } => Self::UnexpectedCharacter { character },
        }
    }

    fn parse(kind: &ParseErrorKind) -> Self {
        use ParseErrorKind as K;
        match *kind {
            K::ExpectedValue => Self::ExpectedValue,
            K::TrailingComma => Self::TrailingComma,
            K::NotAfterOperator => Self::NotAfterOperator,
            K::UnexpectedToken => Self::UnexpectedToken,
            K::UnclosedParen => Self::UnclosedParen,
            K::UnmatchedParen => Self::UnmatchedParen,
            K::AssignInExpression => Self::AssignInExpression,
            K::ChainedComparison => Self::ChainedComparison,
            K::ForeignWord { word } => Self::ForeignWord {
                word: word.as_str().to_owned(),
            },
            K::ExpectedName => Self::ExpectedName,
            K::KeywordAsName { keyword } => Self::KeywordAsName {
                keyword: keyword.as_str().to_owned(),
            },
            K::ExpectedEqual => Self::ExpectedEqual,
            K::UnexpectedIndent => Self::UnexpectedIndent,
            K::ExpectedColon => Self::ExpectedColon,
            K::AfterColon => Self::AfterColon,
            K::EmptyBlock => Self::EmptyBlock,
            K::CommentOnlyBlock => Self::CommentOnlyBlock,
            K::ElseWithoutIf => Self::ElseWithoutIf,
            K::ElseAfterElse => Self::ElseAfterElse,
            K::ExpectedFunctionName => Self::ExpectedFunctionName,
            K::ExpectedParentheses => Self::ExpectedParentheses,
            K::InvalidParameter => Self::InvalidParameter,
            K::TooDeep => Self::TooDeep,
        }
    }

    fn check(kind: CheckErrorKind, offsets: &Offsets) -> Self {
        use CheckErrorKind as K;
        match kind {
            K::BreakOutsideLoop => Self::BreakOutsideLoop,
            K::ReturnOutsideFunction => Self::ReturnOutsideFunction,
            K::NestedFunction => Self::NestedFunction,
            K::NotSnakeCase => Self::NotSnakeCase,
            K::ReservedName { word } => Self::ReservedName {
                word: word.as_str().to_owned(),
                belongs_to: match word {
                    ReservedWord::Builtin(_) => ReservedKind::Function,
                    ReservedWord::Color(_) => ReservedKind::Color,
                },
            },
            K::DuplicateParameter { name } => Self::DuplicateParameter { name },
            K::UnknownName { name } => Self::UnknownName { name },
            K::UndeclaredVariableChanged { name } => Self::UndeclaredVariableChanged { name },
            K::VariableBeforeLet { name, declared } => Self::VariableBeforeLet {
                name,
                declared: offsets.span(declared),
            },
            K::VariableAfterBlock { name, declared } => Self::VariableAfterBlock {
                name,
                declared: offsets.span(declared),
            },
            K::MainVariableInFunction { name } => Self::MainVariableInFunction { name },
            K::AlreadyDeclared { name, first } => Self::AlreadyDeclared {
                name,
                first: offsets.span(first),
            },
            K::DuplicateFunction { name, first } => Self::DuplicateFunction {
                name,
                first: offsets.span(first),
            },
            K::NameOfFunction { name, defined } => Self::NameOfFunction {
                name,
                defined: offsets.span(defined),
            },
            K::CalledBeforeDefinition { name, defined } => Self::CalledBeforeDefinition {
                name,
                defined: offsets.span(defined),
            },
            K::WrongArgumentCount {
                name,
                expected,
                given,
            } => Self::WrongArgumentCount {
                name,
                expected: count(expected),
                given: count(given),
            },
            K::FunctionWithoutParentheses { name } => Self::FunctionWithoutParentheses { name },
            K::VariableCalled { name } => Self::VariableCalled { name },
            K::UnknownFunction { name } => Self::UnknownFunction { name },
            K::ExpressionAlone => Self::ExpressionAlone,
        }
    }
}

/// A count of the core as a number that JavaScript holds exactly.
fn count(count: usize) -> u32 {
    u32::try_from(count).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use stepwise_core::builtin::Builtin;
    use stepwise_core::checker::{CheckErrorKind, ReservedWord};
    use stepwise_core::color::Color;
    use stepwise_core::lexer::{Keyword, LexErrorKind};
    use stepwise_core::parser::{ForeignWord, ParseErrorKind};
    use stepwise_core::span::{self, Position};

    use super::{ProgramError, ProgramErrorKind as Kind, ReservedKind, errors};
    use crate::position::{Offsets, Span};

    const fn at(line: u32, from: u32, to: u32) -> Span {
        Span { line, from, to }
    }

    /// The error of `kind` on the line `line`, from the offset `from` to the offset `to`.
    const fn error(line: u32, from: u32, to: u32, kind: Kind) -> ProgramError {
        ProgramError {
            span: at(line, from, to),
            kind,
        }
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
            [error(2, 17, 18, Kind::UnclosedParen)]
        );
    }

    #[test]
    fn an_error_of_the_lexer_keeps_its_values() {
        let wrong_indent = Kind::WrongIndent {
            found: 6,
            below: 0,
            above: 4,
        };
        let symbol = Kind::SymbolForKeyword {
            symbol: "&&".to_owned(),
            keyword: "and".to_owned(),
        };
        let digit = Kind::NameStartsWithDigit {
            word: "2nd".to_owned(),
        };
        for (source, expected) in [
            ("if done:\n      forward()\n", error(2, 9, 15, wrong_indent)),
            (
                "print(\"côté\\q\")\n",
                error(1, 11, 13, Kind::UnknownEscape { escape: 'q' }),
            ),
            ("if a && b:\n    forward()\n", error(1, 5, 7, symbol)),
            ("let 2nd = 1\n", error(1, 4, 7, digit)),
            (
                "let a = @\n",
                error(1, 8, 9, Kind::UnexpectedCharacter { character: '@' }),
            ),
        ] {
            assert_eq!(errors(source), [expected], "in {source:?}");
        }
    }

    #[test]
    fn an_error_of_the_parser_keeps_its_values() {
        let keyword = Kind::KeywordAsName {
            keyword: "repeat".to_owned(),
        };
        let foreign = Kind::ForeignWord {
            word: "elif".to_owned(),
        };
        for (source, expected) in [
            ("let repeat = 3\n", error(1, 4, 10, keyword)),
            (
                "if a:\n    forward()\nelif b:\n    forward()\n",
                error(3, 20, 24, foreign),
            ),
        ] {
            assert_eq!(errors(source), [expected], "in {source:?}");
        }
    }

    #[test]
    fn something_missing_at_the_end_of_a_line_is_a_place_between_two_characters() {
        assert_eq!(
            errors("let sides =\nforward()\n"),
            [error(1, 11, 11, Kind::ExpectedValue)]
        );
    }

    #[test]
    fn every_error_of_names_comes_in_the_order_of_the_source_with_its_places() {
        // The emoji of the line 2 counts for two in the places of the line 3.
        let source = "print(côté)\nlet côté = \"🤖\"\nforward(côté)\n";
        let before_let = Kind::VariableBeforeLet {
            name: "côté".to_owned(),
            declared: at(2, 16, 20),
        };
        let wrong_count = Kind::WrongArgumentCount {
            name: "forward".to_owned(),
            expected: 0,
            given: 1,
        };
        assert_eq!(
            errors(source),
            [
                error(1, 6, 10, before_let),
                error(2, 16, 20, Kind::NotSnakeCase),
                error(3, 28, 35, wrong_count),
            ]
        );
    }

    #[test]
    fn a_reserved_name_says_what_its_word_belongs_to() {
        let color = Kind::ReservedName {
            word: "red".to_owned(),
            belongs_to: ReservedKind::Color,
        };
        let function = Kind::ReservedName {
            word: "forward".to_owned(),
            belongs_to: ReservedKind::Function,
        };
        assert_eq!(
            errors("let red = 3\nlet forward = 4\n"),
            [error(1, 4, 7, color), error(2, 16, 23, function)]
        );
    }

    #[test]
    fn an_error_keeps_the_name_it_is_about() {
        let name = |name: &str| name.to_owned();
        for (source, expected) in [
            (
                "function paint(side, side):\n    forward()\n",
                error(1, 21, 25, Kind::DuplicateParameter { name: name("side") }),
            ),
            (
                "print(sidse)\n",
                error(
                    1,
                    6,
                    11,
                    Kind::UnknownName {
                        name: name("sidse"),
                    },
                ),
            ),
            (
                "sidse = 1\n",
                error(
                    1,
                    0,
                    5,
                    Kind::UndeclaredVariableChanged {
                        name: name("sidse"),
                    },
                ),
            ),
            (
                "let sides = 4\nfunction show():\n    print(sides)\n",
                error(
                    3,
                    41,
                    46,
                    Kind::MainVariableInFunction {
                        name: name("sides"),
                    },
                ),
            ),
            (
                "let go = forward\n",
                error(
                    1,
                    9,
                    16,
                    Kind::FunctionWithoutParentheses {
                        name: name("forward"),
                    },
                ),
            ),
            (
                "let sides = 4\nsides()\n",
                error(
                    2,
                    14,
                    19,
                    Kind::VariableCalled {
                        name: name("sides"),
                    },
                ),
            ),
            (
                "turn()\n",
                error(1, 0, 4, Kind::UnknownFunction { name: name("turn") }),
            ),
        ] {
            assert_eq!(errors(source), [expected], "in {source:?}");
        }
    }

    #[test]
    fn the_places_kept_in_an_error_are_converted_as_its_own() {
        // The emoji of the line 1 counts for two before the `=` kept by each error.
        let offsets = Offsets::new("let 🤖 = 4\nsides = 1\n");
        let (other, kept) = (place((1, 7), (1, 8)), at(1, 7, 8));
        let name = || "sides".to_owned();
        for (kind, expected) in [
            (
                CheckErrorKind::VariableBeforeLet {
                    name: name(),
                    declared: other,
                },
                Kind::VariableBeforeLet {
                    name: name(),
                    declared: kept,
                },
            ),
            (
                CheckErrorKind::VariableAfterBlock {
                    name: name(),
                    declared: other,
                },
                Kind::VariableAfterBlock {
                    name: name(),
                    declared: kept,
                },
            ),
            (
                CheckErrorKind::AlreadyDeclared {
                    name: name(),
                    first: other,
                },
                Kind::AlreadyDeclared {
                    name: name(),
                    first: kept,
                },
            ),
            (
                CheckErrorKind::DuplicateFunction {
                    name: name(),
                    first: other,
                },
                Kind::DuplicateFunction {
                    name: name(),
                    first: kept,
                },
            ),
            (
                CheckErrorKind::NameOfFunction {
                    name: name(),
                    defined: other,
                },
                Kind::NameOfFunction {
                    name: name(),
                    defined: kept,
                },
            ),
            (
                CheckErrorKind::CalledBeforeDefinition {
                    name: name(),
                    defined: other,
                },
                Kind::CalledBeforeDefinition {
                    name: name(),
                    defined: kept,
                },
            ),
        ] {
            assert_eq!(Kind::check(kind, &offsets), expected);
        }
    }

    // Each kind of the bridge is the kind of the same name in the core: a line of a conversion
    // that gave another one would give the learner the message of another error.

    fn place(start: (u32, u32), end: (u32, u32)) -> span::Span {
        span::Span::new(Position::new(start.0, start.1), Position::new(end.0, end.1))
    }

    /// The name of a variant, as `Debug` writes it first.
    fn variant(value: &impl Debug) -> String {
        format!("{value:?}")
            .chars()
            .take_while(char::is_ascii_alphanumeric)
            .collect()
    }

    /// Checks that `convert` gives each kind of the core the kind of the same name.
    fn assert_same_names<K: Debug>(
        kinds: impl IntoIterator<Item = K>,
        convert: impl Fn(K) -> Kind,
    ) {
        for kind in kinds {
            let expected = variant(&kind);
            assert_eq!(variant(&convert(kind)), expected);
        }
    }

    #[test]
    fn every_error_of_the_lexer_becomes_the_error_of_its_name() {
        let kinds = [
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
        assert_same_names(kinds, Kind::lex);
    }

    #[test]
    fn every_error_of_the_parser_becomes_the_error_of_its_name() {
        let kinds = [
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
        assert_same_names(kinds, |kind| Kind::parse(&kind));
    }

    #[test]
    fn every_error_of_the_checker_becomes_the_error_of_its_name() {
        let offsets = Offsets::new("let sides = 4\nsides = sides + 1\n");
        let other = place((1, 5), (1, 10));
        let name = || "sides".to_owned();
        let kinds = [
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
                declared: other,
            },
            CheckErrorKind::VariableAfterBlock {
                name: name(),
                declared: other,
            },
            CheckErrorKind::MainVariableInFunction { name: name() },
            CheckErrorKind::AlreadyDeclared {
                name: name(),
                first: other,
            },
            CheckErrorKind::DuplicateFunction {
                name: name(),
                first: other,
            },
            CheckErrorKind::NameOfFunction {
                name: name(),
                defined: other,
            },
            CheckErrorKind::CalledBeforeDefinition {
                name: name(),
                defined: other,
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
        assert_same_names(kinds, |kind| Kind::check(kind, &offsets));
    }
}
