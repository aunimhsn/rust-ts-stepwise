//! The tests of the parser, grouped by the rules of the specifications.

use super::{ForeignWord, MAX_DEPTH, ParseError, ParseErrorKind as E, SyntaxError, parse};
use crate::ast::{
    BinaryOperator, Expression, ExpressionKind, Statement, StatementKind, UnaryOperator,
};
use crate::lexer::{Keyword, LexErrorKind};
use crate::span::{Position, Span};

/// The statements of `source`, which must have no error.
fn statements(source: &str) -> Vec<Statement> {
    parse(source).unwrap().statements
}

/// The statements of `source`, each written back with every operator between parentheses:
/// `1 + 2 * 3` gives `(1 + (2 * 3))`.
fn program(source: &str) -> Vec<String> {
    statements(source).iter().map(show_statement).collect()
}

/// The only statement of `source`, written back as by [`program`].
fn line(source: &str) -> String {
    let mut lines = program(source);
    assert_eq!(lines.len(), 1, "one statement expected in {source:?}");
    lines.pop().unwrap()
}

/// The expression alone on the only line of `source`.
fn expression(source: &str) -> Expression {
    let mut statements = statements(source);
    assert_eq!(statements.len(), 1, "one statement expected in {source:?}");
    match statements.pop().unwrap().kind {
        StatementKind::Expression(expression) => expression,
        other => panic!("an expression expected, found {other:?}"),
    }
}

/// The error of the parser for `source`, which must have one.
fn error(source: &str) -> ParseError {
    match parse(source) {
        Err(SyntaxError::Parse(error)) => error,
        other => panic!("an error of the parser expected, found {other:?}"),
    }
}

/// The kind of the error of the parser for `source`, which must have one.
fn error_kind(source: &str) -> E {
    error(source).kind
}

/// The span on `line` from the column `start` included to the column `end` excluded.
fn span(line: u32, start: u32, end: u32) -> Span {
    Span::new(Position::new(line, start), Position::new(line, end))
}

fn show_statement(statement: &Statement) -> String {
    match &statement.kind {
        StatementKind::Let { name, value } => format!("let {} = {}", name.text, show(value)),
        StatementKind::Assign { name, value } => format!("{} = {}", name.text, show(value)),
        StatementKind::Break => "break".to_owned(),
        StatementKind::Return { value: None } => "return".to_owned(),
        StatementKind::Return { value: Some(value) } => format!("return {}", show(value)),
        StatementKind::Expression(expression) => show(expression),
    }
}

fn show(expression: &Expression) -> String {
    match &expression.kind {
        ExpressionKind::Number(number) => number.to_string(),
        ExpressionKind::Boolean(boolean) => boolean.to_string(),
        ExpressionKind::Text(text) => format!("{text:?}"),
        ExpressionKind::Name(name) => name.clone(),
        ExpressionKind::Call {
            function,
            arguments,
        } => {
            let arguments: Vec<String> = arguments.iter().map(show).collect();
            format!("{}({})", function.text, arguments.join(", "))
        }
        ExpressionKind::Unary { operator, operand } => {
            let operator = match operator {
                UnaryOperator::Negate => "-",
                UnaryOperator::Not => "not ",
            };
            format!("({operator}{})", show(operand))
        }
        ExpressionKind::Binary {
            operator,
            left,
            right,
        } => {
            let operator = match operator {
                BinaryOperator::Add => "+",
                BinaryOperator::Subtract => "-",
                BinaryOperator::Multiply => "*",
                BinaryOperator::Divide => "/",
                BinaryOperator::Remainder => "%",
                BinaryOperator::Equal => "==",
                BinaryOperator::NotEqual => "!=",
                BinaryOperator::Less => "<",
                BinaryOperator::LessEqual => "<=",
                BinaryOperator::Greater => ">",
                BinaryOperator::GreaterEqual => ">=",
                BinaryOperator::And => "and",
                BinaryOperator::Or => "or",
            };
            format!("({} {operator} {})", show(left), show(right))
        }
    }
}

mod programs {
    use super::*;

    #[test]
    fn an_empty_source_is_an_empty_program() {
        assert_eq!(program(""), Vec::<String>::new());
        assert_eq!(program("\n\n"), Vec::<String>::new());
    }

    #[test]
    fn a_source_of_comments_is_an_empty_program() {
        assert_eq!(program("# nothing\n    # here\n"), Vec::<String>::new());
    }

    #[test]
    fn statements_come_in_the_order_of_the_source() {
        assert_eq!(
            program("let sides = 0\nforward()\n\n# turn\nsides = sides + 1\n"),
            ["let sides = 0", "forward()", "sides = (sides + 1)"]
        );
    }

    #[test]
    fn the_last_line_needs_no_line_break() {
        assert_eq!(
            program("forward()\nturn_left()"),
            ["forward()", "turn_left()"]
        );
    }

    #[test]
    fn a_statement_spans_its_line_without_the_comment() {
        let statements = statements("forward()\nlet sides = 10  # count\n");
        assert_eq!(statements[0].span, span(1, 1, 10));
        assert_eq!(statements[1].span, span(2, 1, 15));
    }

    #[test]
    fn an_error_of_the_lexer_comes_through() {
        let Err(SyntaxError::Lex(error)) = parse("let a = 'x'") else {
            panic!("an error of the lexer expected");
        };
        assert_eq!(error.kind, LexErrorKind::SingleQuote);
        assert_eq!(
            SyntaxError::Lex(error).span(),
            span(1, 9, 10),
            "the span of a syntax error is the one of its error"
        );
    }

    #[test]
    fn the_span_of_a_syntax_error_is_the_one_of_the_parser() {
        let error = parse("let").unwrap_err();
        assert_eq!(error.span(), span(1, 4, 4));
    }

    #[test]
    fn only_the_first_error_is_given() {
        assert_eq!(error("let = 3\nlet a =\n").span, span(1, 5, 6));
    }

    #[test]
    fn a_line_indented_under_a_statement_is_an_error() {
        let error = error("forward()\n    forward()\n");
        assert_eq!(error.kind, E::UnexpectedIndent);
        assert_eq!(error.span, span(2, 1, 5));
    }

    #[test]
    fn two_statements_on_a_line_are_an_error() {
        let error = error("forward() forward()");
        assert_eq!(error.kind, E::UnexpectedToken);
        assert_eq!(error.span, span(1, 11, 18));
    }

    /// Until step 6b, which parses the blocks.
    #[test]
    fn a_line_that_opens_a_block_is_an_error_for_now() {
        for source in [
            "if true:",
            "else:",
            "while true:",
            "repeat 3:",
            "function f():",
        ] {
            assert_eq!(error_kind(source), E::UnexpectedToken, "{source}");
        }
    }
}

mod values {
    use super::*;

    #[test]
    fn a_number() {
        assert_eq!(expression("42").kind, ExpressionKind::Number(42));
        assert_eq!(
            expression("9223372036854775807").kind,
            ExpressionKind::Number(i64::MAX)
        );
    }

    #[test]
    fn a_negative_number_is_a_minus_in_front_of_a_number() {
        assert_eq!(line("-7"), "(-7)");
    }

    #[test]
    fn booleans() {
        assert_eq!(expression("true").kind, ExpressionKind::Boolean(true));
        assert_eq!(expression("false").kind, ExpressionKind::Boolean(false));
    }

    #[test]
    fn a_text_keeps_its_content() {
        assert_eq!(
            expression(r#""I am \"stuck\"""#).kind,
            ExpressionKind::Text("I am \"stuck\"".to_owned())
        );
    }

    #[test]
    fn a_name_alone() {
        let expression = expression("red");
        assert_eq!(expression.kind, ExpressionKind::Name("red".to_owned()));
        assert_eq!(expression.span, span(1, 1, 4));
    }

    #[test]
    fn a_name_in_any_alphabet_is_left_to_the_checker() {
        assert_eq!(line("let côté = paintSide"), "let côté = paintSide");
    }
}

mod calls {
    use super::*;

    #[test]
    fn a_call_without_values() {
        let expression = expression("forward()");
        let ExpressionKind::Call {
            function,
            arguments,
        } = expression.kind
        else {
            panic!("a call expected");
        };
        assert_eq!(function.text, "forward");
        assert_eq!(function.span, span(1, 1, 8));
        assert_eq!(arguments.len(), 0);
        assert_eq!(expression.span, span(1, 1, 10));
    }

    #[test]
    fn a_call_with_values() {
        assert_eq!(
            line(r#"print("Sides:", sides + 1, red)"#),
            r#"print("Sides:", (sides + 1), red)"#
        );
    }

    #[test]
    fn calls_nest() {
        assert_eq!(line("print(double(double(2)))"), "print(double(double(2)))");
    }

    #[test]
    fn a_call_is_a_value_among_operators() {
        assert_eq!(
            line("not front_is(red) and size() * 2 > 3"),
            "((not front_is(red)) and ((size() * 2) > 3))"
        );
    }

    #[test]
    fn a_space_may_come_before_the_parenthesis() {
        assert_eq!(line("forward ()"), "forward()");
    }

    #[test]
    fn a_comma_after_the_last_value_is_an_error() {
        let error = error("paint_side(3,)");
        assert_eq!(error.kind, E::ExpectedValue);
        assert_eq!(error.span, span(1, 14, 15));
    }

    #[test]
    fn a_comma_without_a_value_is_an_error() {
        assert_eq!(error_kind("print(,)"), E::ExpectedValue);
        assert_eq!(error_kind("print(1,,2)"), E::ExpectedValue);
    }

    #[test]
    fn two_values_without_a_comma_are_an_error() {
        let error = error(r#"print("a" "b")"#);
        assert_eq!(error.kind, E::UnexpectedToken);
        assert_eq!(error.span, span(1, 11, 14));
    }

    #[test]
    fn a_call_not_closed_is_an_error_at_its_parenthesis() {
        let error = error("print(1, 2\nforward()");
        assert_eq!(error.kind, E::UnclosedParen);
        assert_eq!(error.span, span(1, 6, 7));
        assert_eq!(error_kind("forward("), E::ExpectedValue);
    }

    #[test]
    fn only_a_name_is_called() {
        for source in ["3()", "f()()", "(f)()", r#""f"()"#] {
            assert_eq!(error_kind(source), E::UnexpectedToken, "{source}");
        }
    }
}

mod priorities {
    use super::*;

    #[test]
    fn products_before_sums() {
        assert_eq!(line("1 + 2 * 3"), "(1 + (2 * 3))");
        assert_eq!(line("1 * 2 + 3"), "((1 * 2) + 3)");
        assert_eq!(line("8 - 6 / 2 % 5"), "(8 - ((6 / 2) % 5))");
    }

    #[test]
    fn operators_of_the_same_priority_group_from_the_left() {
        assert_eq!(line("10 - 4 - 3"), "((10 - 4) - 3)");
        assert_eq!(line("10 - 4 + 3"), "((10 - 4) + 3)");
        assert_eq!(line("24 / 4 / 2"), "((24 / 4) / 2)");
        assert_eq!(line("7 * 2 % 4 / 3"), "(((7 * 2) % 4) / 3)");
        assert_eq!(line("a and b and c"), "((a and b) and c)");
        assert_eq!(line("a or b or c"), "((a or b) or c)");
    }

    #[test]
    fn sums_before_comparisons() {
        assert_eq!(line("a + 1 < b * 2"), "((a + 1) < (b * 2))");
        assert_eq!(line("a == b - 1"), "(a == (b - 1))");
    }

    #[test]
    fn every_comparison() {
        for operator in ["==", "!=", "<", "<=", ">", ">="] {
            assert_eq!(
                line(&format!("a {operator} b")),
                format!("(a {operator} b)")
            );
        }
    }

    #[test]
    fn comparisons_before_and_before_or() {
        assert_eq!(
            line("a < b and c > d or e == f"),
            "(((a < b) and (c > d)) or (e == f))"
        );
        assert_eq!(line("a or b and c"), "(a or (b and c))");
        assert_eq!(line("a and b or c and d"), "((a and b) or (c and d))");
    }

    #[test]
    fn a_minus_in_front_takes_the_value_alone() {
        assert_eq!(line("-a * b"), "((-a) * b)");
        assert_eq!(line("a * -b"), "(a * (-b))");
        assert_eq!(line("a - -b"), "(a - (-b))");
        assert_eq!(line("-a + b"), "((-a) + b)");
        assert_eq!(line("-size() < -(a + b)"), "((-size()) < (-(a + b)))");
    }

    #[test]
    fn not_takes_the_comparison_that_follows() {
        assert_eq!(line("not sides == 4"), "(not (sides == 4))");
        assert_eq!(line("not a + 1 < b"), "(not ((a + 1) < b))");
    }

    #[test]
    fn not_comes_before_and_and_or() {
        assert_eq!(line("not a and b"), "((not a) and b)");
        assert_eq!(line("not a or b"), "((not a) or b)");
        assert_eq!(line("a and not b or c"), "((a and (not b)) or c)");
        assert_eq!(line("not a == b and not c"), "((not (a == b)) and (not c))");
    }

    #[test]
    fn minus_and_not_repeat() {
        assert_eq!(line("- -3"), "(-(-3))");
        assert_eq!(line("--3"), "(-(-3))");
        assert_eq!(line("not not done"), "(not (not done))");
    }

    #[test]
    fn not_is_a_value_wherever_one_is_expected() {
        assert_eq!(line("a == not b"), "(a == (not b))");
        assert_eq!(line("print(not a, -b)"), "print((not a), (-b))");
    }

    #[test]
    fn parentheses_group() {
        assert_eq!(line("(1 + 2) * 3"), "((1 + 2) * 3)");
        assert_eq!(line("10 - (4 - 3)"), "(10 - (4 - 3))");
        assert_eq!(line("not (a and b)"), "(not (a and b))");
        assert_eq!(line("(not a) == b"), "((not a) == b)");
        assert_eq!(line("((front_is_clear()))"), "front_is_clear()");
    }
}

mod spans {
    use super::*;

    #[test]
    fn an_operator_spans_its_two_values() {
        let expression = expression("sides + 10 * 2");
        assert_eq!(expression.span, span(1, 1, 15));
        let ExpressionKind::Binary { left, right, .. } = expression.kind else {
            panic!("an operator expected");
        };
        assert_eq!(left.span, span(1, 1, 6));
        assert_eq!(right.span, span(1, 9, 15));
    }

    #[test]
    fn an_operator_in_front_spans_itself_and_its_value() {
        assert_eq!(expression("not  done").span, span(1, 1, 10));
        assert_eq!(expression("- 3").span, span(1, 1, 4));
    }

    #[test]
    fn parentheses_widen_the_span_of_their_expression() {
        let expression = expression("( a + b ) * c");
        let ExpressionKind::Binary { left, .. } = expression.kind else {
            panic!("an operator expected");
        };
        assert_eq!(left.span, span(1, 1, 10));
    }

    #[test]
    fn columns_count_characters() {
        let expression = expression(r#"print("côté") + a"#);
        let ExpressionKind::Binary { right, .. } = expression.kind else {
            panic!("an operator expected");
        };
        assert_eq!(right.span, span(1, 17, 18));
    }
}

mod expression_errors {
    use super::*;

    #[test]
    fn a_value_missing_after_an_operator() {
        let error = error("let a = 1 +");
        assert_eq!(error.kind, E::ExpectedValue);
        assert_eq!(error.span, span(1, 12, 12), "at the end of the line");
        assert_eq!(error_kind("1 + * 2"), E::ExpectedValue);
        assert_eq!(error_kind("a and"), E::ExpectedValue);
        assert_eq!(error_kind("not"), E::ExpectedValue);
        assert_eq!(error_kind("-"), E::ExpectedValue);
    }

    #[test]
    fn a_value_missing_before_an_operator() {
        let error = error("* 2");
        assert_eq!(error.kind, E::ExpectedValue);
        assert_eq!(error.span, span(1, 1, 2));
        assert_eq!(error_kind("== 2"), E::ExpectedValue);
    }

    #[test]
    fn a_keyword_is_no_value() {
        assert_eq!(error_kind("let a = let"), E::ExpectedValue);
        assert_eq!(error_kind("print(while)"), E::ExpectedValue);
    }

    #[test]
    fn empty_parentheses_are_an_error() {
        let error = error("let a = ()");
        assert_eq!(error.kind, E::ExpectedValue);
        assert_eq!(error.span, span(1, 10, 11));
    }

    #[test]
    fn two_values_in_a_row_are_an_error() {
        let error = error("let a = 1 2");
        assert_eq!(error.kind, E::UnexpectedToken);
        assert_eq!(error.span, span(1, 11, 12));
        assert_eq!(error_kind("a b"), E::UnexpectedToken);
    }

    #[test]
    fn a_parenthesis_not_closed_is_an_error_at_the_parenthesis() {
        let error = error("let a = 2 * (1 + 2");
        assert_eq!(error.kind, E::UnclosedParen);
        assert_eq!(error.span, span(1, 13, 14));
    }

    #[test]
    fn the_innermost_parenthesis_not_closed_is_the_error() {
        assert_eq!(error("((1 + 2)").span, span(1, 1, 2));
        assert_eq!(error("(1 + (2").span, span(1, 6, 7));
    }

    #[test]
    fn a_line_never_goes_on_inside_parentheses() {
        assert_eq!(error_kind("print(1,\n2)"), E::ExpectedValue);
        assert_eq!(error_kind("let a = (1 +\n2)"), E::ExpectedValue);
    }

    #[test]
    fn a_closing_parenthesis_alone_is_an_error() {
        let error = error("forward())");
        assert_eq!(error.kind, E::UnmatchedParen);
        assert_eq!(error.span, span(1, 10, 11));
        assert_eq!(error_kind("let a = 1 + 2)"), E::UnmatchedParen);
    }

    #[test]
    fn something_else_than_a_closing_parenthesis_is_an_error() {
        let error = error("(1 + 2 let");
        assert_eq!(error.kind, E::UnexpectedToken);
        assert_eq!(error.span, span(1, 8, 11));
    }

    #[test]
    fn an_equal_sign_in_an_expression_is_an_error() {
        let error = error("let a = b = 3");
        assert_eq!(error.kind, E::AssignInExpression);
        assert_eq!(error.span, span(1, 11, 12));
        for source in [
            "a = b = 3",
            "print(x = 3)",
            "return x = 3",
            "x + 1 = 3",
            "let a = (b = 3)",
            "let a = not b = 3",
            "let a = b < c = d",
            "forward() = 3",
        ] {
            assert_eq!(error_kind(source), E::AssignInExpression, "{source}");
        }
    }

    #[test]
    fn comparisons_do_not_chain() {
        let error = error("let inside = 1 < x < 5");
        assert_eq!(error.kind, E::ChainedComparison);
        assert_eq!(error.span, span(1, 20, 21), "the second comparison");
        for source in [
            "a == b == c",
            "a < b == true",
            "a != b >= c",
            "a + 1 < b * 2 <= c",
            "not a < b < c",
            "x and a < b < c",
            "print(a > b > c)",
        ] {
            assert_eq!(error_kind(source), E::ChainedComparison, "{source}");
        }
    }

    #[test]
    fn comparisons_between_parentheses_or_joined_are_fine() {
        assert_eq!(line("(a < b) == c"), "((a < b) == c)");
        assert_eq!(line("a == (b < c)"), "(a == (b < c))");
        assert_eq!(line("1 < x and x < 5"), "((1 < x) and (x < 5))");
        assert_eq!(
            line("a < b or c < d and e < f"),
            "((a < b) or ((c < d) and (e < f)))"
        );
        assert_eq!(line("a == not b == c"), "(a == (not (b == c)))");
    }
}

mod statement_rules {
    use super::*;

    #[test]
    fn a_declaration() {
        let statement = statements("let sides = 4 * 2").pop().unwrap();
        let StatementKind::Let { name, value } = statement.kind else {
            panic!("a declaration expected");
        };
        assert_eq!(name.text, "sides");
        assert_eq!(name.span, span(1, 5, 10));
        assert_eq!(show(&value), "(4 * 2)");
    }

    #[test]
    fn a_declaration_without_a_name_is_an_error() {
        let error = error("let = 3");
        assert_eq!(error.kind, E::ExpectedName);
        assert_eq!(error.span, span(1, 5, 6));
        assert_eq!(error_kind("let 3 = 4"), E::ExpectedName);
        assert_eq!(error_kind("let"), E::ExpectedName);
        assert_eq!(error_kind(r#"let "a" = 4"#), E::ExpectedName);
    }

    #[test]
    fn a_keyword_is_no_name() {
        let error = error("let repeat = 3");
        assert_eq!(
            error.kind,
            E::KeywordAsName {
                keyword: Keyword::Repeat
            }
        );
        assert_eq!(error.span, span(1, 5, 11));
        for keyword in Keyword::ALL {
            assert_eq!(
                error_kind(&format!("let {} = 3", keyword.as_str())),
                E::KeywordAsName { keyword }
            );
        }
    }

    #[test]
    fn a_declaration_without_a_value_is_an_error() {
        let error = error("let sides");
        assert_eq!(error.kind, E::ExpectedEqual);
        assert_eq!(error.span, span(1, 10, 10));
        assert_eq!(error_kind("let sides 3"), E::ExpectedEqual);
        assert_eq!(error_kind("let sides == 3"), E::ExpectedEqual);
        assert_eq!(error_kind("let a, b = 3"), E::ExpectedEqual);
        assert_eq!(error_kind("let sides ="), E::ExpectedValue);
    }

    #[test]
    fn an_assignment() {
        let statement = statements("sides = sides + 1").pop().unwrap();
        let StatementKind::Assign { name, value } = statement.kind else {
            panic!("an assignment expected");
        };
        assert_eq!(name.text, "sides");
        assert_eq!(name.span, span(1, 1, 6));
        assert_eq!(show(&value), "(sides + 1)");
    }

    #[test]
    fn an_assignment_without_a_value_is_an_error() {
        assert_eq!(error_kind("sides ="), E::ExpectedValue);
        assert_eq!(error_kind("sides = = 3"), E::ExpectedValue);
    }

    #[test]
    fn only_a_name_is_assigned() {
        assert_eq!(error_kind("3 = sides"), E::AssignInExpression);
        assert_eq!(error_kind("= 3"), E::ExpectedValue);
    }

    #[test]
    fn a_comparison_alone_is_no_assignment() {
        assert_eq!(line("sides == 4"), "(sides == 4)");
    }

    #[test]
    fn a_break() {
        assert_eq!(statements("break")[0].kind, StatementKind::Break);
        assert_eq!(error_kind("break 3"), E::UnexpectedToken);
    }

    #[test]
    fn a_return_with_or_without_a_value() {
        assert_eq!(
            program("return\nreturn  # nothing\nreturn sides * 2"),
            ["return", "return", "return (sides * 2)"]
        );
        assert_eq!(error_kind("return 1 2"), E::UnexpectedToken);
        assert_eq!(error_kind("return )"), E::ExpectedValue);
    }

    #[test]
    fn any_expression_alone_is_left_to_the_checker() {
        assert_eq!(
            program("3 + 4\nsides\nforward\n\"text\"\ntrue"),
            ["(3 + 4)", "sides", "forward", "\"text\"", "true"]
        );
    }
}

mod foreign_words {
    use super::*;

    fn foreign(source: &str) -> ParseError {
        let error = error(source);
        assert!(
            matches!(error.kind, E::ForeignWord { .. }),
            "{source}: {error:?}"
        );
        error
    }

    #[test]
    fn every_word_is_an_error_wherever_it_is() {
        for word in ForeignWord::ALL {
            let text = word.as_str();
            for source in [
                text.to_owned(),
                format!("{text} x = 3"),
                format!("{text} = 3"),
                format!("{text}(3)"),
                format!("let {text} = 3"),
                format!("let a = {text}"),
                format!("let a = 1 + {text}"),
                format!("print(1, {text})"),
                format!("print(1 {text}"),
                format!("forward() {text}"),
                format!("return {text}"),
                format!("let a {text}"),
            ] {
                assert_eq!(foreign(&source).kind, E::ForeignWord { word }, "{source}");
            }
        }
    }

    #[test]
    fn the_error_is_at_the_word() {
        assert_eq!(foreign("elif x > 3:").span, span(1, 1, 5));
        assert_eq!(foreign("let a = null").span, span(1, 9, 13));
        assert_eq!(foreign("for i in range(3):").span, span(1, 1, 4));
    }

    #[test]
    fn an_error_above_the_word_comes_first() {
        assert_eq!(error_kind("let = 3\npass"), E::ExpectedName);
        assert_eq!(
            error_kind("let a = (1 + pass"),
            E::ForeignWord {
                word: ForeignWord::Pass
            }
        );
    }

    #[test]
    fn a_name_that_holds_a_word_is_a_name() {
        assert_eq!(
            program("let passing = format\nfor_each(nullable, variable)\nlet Pass = None"),
            [
                "let passing = format",
                "for_each(nullable, variable)",
                "let Pass = None"
            ]
        );
    }

    #[test]
    fn every_word_is_read_back_from_its_text() {
        for word in ForeignWord::ALL {
            assert_eq!(ForeignWord::from_word(word.as_str()), Some(word));
        }
        assert_eq!(ForeignWord::from_word("let"), None);
    }
}

mod depth {
    use super::*;

    const MAX: usize = MAX_DEPTH as usize;

    /// `count` values joined by `+`: a tree of `count` levels.
    fn chain(count: usize) -> String {
        vec!["1"; count].join(" + ")
    }

    /// A value inside `count` parentheses: `count` levels and the one of the value.
    fn parentheses(count: usize) -> String {
        format!("{}1{}", "(".repeat(count), ")".repeat(count))
    }

    #[test]
    fn a_chain_of_operators_as_deep_as_the_limit() {
        assert!(parse(&chain(MAX)).is_ok());
        assert_eq!(error_kind(&chain(MAX + 1)), E::TooDeep);
    }

    #[test]
    fn a_chain_too_deep_is_underlined_up_to_the_value_too_many() {
        let error = error(&chain(MAX + 1));
        assert_eq!(error.span.start, Position::new(1, 1));
        assert_eq!(error.span.end.column as usize, chain(MAX + 1).len() + 1);
    }

    #[test]
    fn parentheses_as_deep_as_the_limit() {
        assert!(parse(&parentheses(MAX - 1)).is_ok());
        assert_eq!(error_kind(&parentheses(MAX)), E::TooDeep);
    }

    #[test]
    fn operators_in_front_as_deep_as_the_limit() {
        assert!(parse(&format!("{}1", "-".repeat(MAX - 1))).is_ok());
        assert_eq!(error_kind(&format!("{}1", "-".repeat(MAX))), E::TooDeep);
        assert!(parse(&format!("{}true", "not ".repeat(MAX - 1))).is_ok());
        assert_eq!(
            error_kind(&format!("{}true", "not ".repeat(MAX))),
            E::TooDeep
        );
    }

    #[test]
    fn calls_as_deep_as_the_limit() {
        let calls = |count: usize| format!("{}1{}", "f(".repeat(count), ")".repeat(count));
        assert!(parse(&calls(MAX - 1)).is_ok());
        assert_eq!(error_kind(&calls(MAX)), E::TooDeep);
    }

    #[test]
    fn the_levels_add_up() {
        // A chain inside parentheses: one level more than the chain.
        assert!(parse(&format!("({})", chain(MAX - 1))).is_ok());
        assert_eq!(error_kind(&format!("({})", chain(MAX))), E::TooDeep);
        // A chain whose first value is deep: the chain is above it.
        let deep_first = |count: usize| format!("{} + 1 + 1", parentheses(count));
        assert!(parse(&deep_first(MAX - 3)).is_ok());
        assert_eq!(error_kind(&deep_first(MAX - 2)), E::TooDeep);
        // A chain whose last value is deep: only one operator is above it.
        let deep_last = |count: usize| format!("1 + 1 + {}", parentheses(count));
        assert!(parse(&deep_last(MAX - 2)).is_ok());
        assert_eq!(error_kind(&deep_last(MAX - 1)), E::TooDeep);
    }

    #[test]
    fn the_depth_starts_again_at_each_statement() {
        let source = format!("{}\n{}\n", chain(MAX), parentheses(MAX - 1));
        assert_eq!(statements(&source).len(), 2);
    }

    #[test]
    fn a_huge_nesting_is_an_error_and_never_an_overflow() {
        for source in [
            chain(100_000),
            "(".repeat(100_000),
            "-".repeat(100_000),
            "not ".repeat(100_000),
            "f(".repeat(100_000),
            "1 + (".repeat(100_000),
            "let a = ".to_owned() + &"(1 * ".repeat(100_000),
        ] {
            assert_eq!(error_kind(&source), E::TooDeep);
        }
    }
}
