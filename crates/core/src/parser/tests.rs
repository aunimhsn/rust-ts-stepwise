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
        StatementKind::If {
            branches,
            otherwise,
        } => {
            let mut shown: Vec<String> = branches
                .iter()
                .map(|branch| {
                    format!(
                        "if {} {}",
                        show(&branch.condition),
                        show_block(&branch.body)
                    )
                })
                .collect();
            shown.extend(otherwise.as_deref().map(show_block));
            shown.join(" else ")
        }
        StatementKind::While { condition, body } => {
            format!("while {} {}", show(condition), show_block(body))
        }
        StatementKind::Repeat { count, body } => {
            format!("repeat {} {}", show(count), show_block(body))
        }
    }
}

/// A block between braces, its statements separated by `;`.
fn show_block(body: &[Statement]) -> String {
    let lines: Vec<String> = body.iter().map(show_statement).collect();
    format!("{{ {} }}", lines.join("; "))
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

    /// Until step 6c, which parses the functions.
    #[test]
    fn a_function_is_an_error_for_now() {
        assert_eq!(error_kind("function f():"), E::UnexpectedToken);
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
        assert_eq!(error.kind, E::TrailingComma);
        assert_eq!(error.span, span(1, 13, 14), "the comma");
        assert_eq!(error_kind("print(1, 2, )"), E::TrailingComma);
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
        assert_eq!(error_kind("print(1,"), E::UnclosedParen);
        assert_eq!(error_kind("print(1 +"), E::ExpectedValue);
    }

    #[test]
    fn a_call_cut_at_its_parenthesis_is_not_closed() {
        let error = error("forward(");
        assert_eq!(error.kind, E::UnclosedParen, "rather than a value missing");
        assert_eq!(error.span, span(1, 8, 9));
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
    fn not_starts_an_expression_or_follows_and_or_or() {
        assert_eq!(line("print(not a, -b)"), "print((not a), (-b))");
        assert_eq!(line("let a = not b"), "let a = (not b)");
        assert_eq!(
            program("return not done\ndone = not done"),
            ["return (not done)", "done = (not done)"]
        );
        assert_eq!(line("a or not b and not c"), "(a or ((not b) and (not c)))");
        assert_eq!(line("a == (not b)"), "(a == (not b))");
        assert_eq!(line("2 * (not a)"), "(2 * (not a))");
    }

    #[test]
    fn not_after_a_stronger_operator_is_an_error() {
        let error = error("let same = a == not b");
        assert_eq!(error.kind, E::NotAfterOperator);
        assert_eq!(error.span, span(1, 17, 20));
        for source in [
            "2 * not a + 1",
            "a < not b < c",
            "1 + not a",
            "a - not b",
            "a % not b",
            // Even with no value after it: the place of the `not` is the first thing wrong.
            "a == not",
            "-not a",
            "not -not a",
            "a and b != not c",
        ] {
            assert_eq!(error_kind(source), E::NotAfterOperator, "{source}");
        }
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
        assert_eq!(error_kind("print(1,\n2)"), E::UnclosedParen);
        assert_eq!(error_kind("let a = (\n1 + 2)"), E::UnclosedParen);
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
    fn a_keyword_is_not_assigned() {
        let error = error("repeat = 3");
        assert_eq!(
            error.kind,
            E::KeywordAsName {
                keyword: Keyword::Repeat
            }
        );
        assert_eq!(error.span, span(1, 1, 7));
        for keyword in Keyword::ALL {
            if keyword != Keyword::Let {
                assert_eq!(
                    error_kind(&format!("{} = 3", keyword.as_str())),
                    E::KeywordAsName { keyword }
                );
            }
        }
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

mod blocks {
    use super::*;

    #[test]
    fn an_if() {
        assert_eq!(line("if a:\n    b()\n"), "if a { b() }");
    }

    #[test]
    fn an_if_with_its_else() {
        let source = "if a:\n    b()\nelse:\n    c()\n";
        assert_eq!(line(source), "if a { b() } else { c() }");
    }

    #[test]
    fn an_if_with_several_else_if() {
        let source = "if a:\n    b()\nelse if c:\n    d()\nelse if e:\n    f()\n";
        assert_eq!(
            line(source),
            "if a { b() } else if c { d() } else if e { f() }"
        );
        let source = format!("{source}else:\n    g()\n");
        assert_eq!(
            line(&source),
            "if a { b() } else if c { d() } else if e { f() } else { g() }"
        );
    }

    #[test]
    fn the_else_if_are_branches_of_one_statement() {
        let source = "if a:\n    b()\nelse if c:\n    d()\nelse:\n    e()\n";
        let mut statements = statements(source);
        assert_eq!(statements.len(), 1);
        let StatementKind::If {
            branches,
            otherwise,
        } = statements.pop().unwrap().kind
        else {
            panic!("an if expected");
        };
        assert_eq!(branches.len(), 2);
        assert_eq!(otherwise.map(|block| block.len()), Some(1));
    }

    #[test]
    fn a_while() {
        let source = "while sides < 4:\n    forward()\n";
        assert_eq!(line(source), "while (sides < 4) { forward() }");
    }

    #[test]
    fn a_repeat() {
        let source = "repeat length + 1:\n    forward()\n";
        assert_eq!(line(source), "repeat (length + 1) { forward() }");
    }

    #[test]
    fn a_block_holds_every_line_under_it() {
        assert_eq!(
            program("if a:\n    b()\n    c()\nd()\n"),
            ["if a { b(); c() }", "d()"]
        );
    }

    #[test]
    fn a_block_holds_any_statement() {
        let source = "while true:\n    let a = 1\n    a = 2\n    break\n    return a\n";
        assert_eq!(
            line(source),
            "while true { let a = 1; a = 2; break; return a }"
        );
    }

    #[test]
    fn blocks_nest() {
        let source = "while a:\n    if b:\n        c()\n    d()\ne()\n";
        assert_eq!(program(source), ["while a { if b { c() }; d() }", "e()"]);
    }

    #[test]
    fn the_end_of_the_source_closes_every_block() {
        let source = "while a:\n    repeat 3:\n        b()";
        assert_eq!(line(source), "while a { repeat 3 { b() } }");
    }

    #[test]
    fn a_line_closes_several_blocks() {
        let source = "while a:\n    repeat 3:\n        b()\nc()\n";
        assert_eq!(program(source), ["while a { repeat 3 { b() } }", "c()"]);
    }

    #[test]
    fn an_else_belongs_to_the_if_at_its_indentation() {
        let outer = "if a:\n    if b:\n        c()\nelse:\n    d()\n";
        assert_eq!(line(outer), "if a { if b { c() } } else { d() }");
        let inner = "if a:\n    if b:\n        c()\n    else:\n        d()\n";
        assert_eq!(line(inner), "if a { if b { c() } else { d() } }");
    }

    #[test]
    fn comments_and_empty_lines_change_nothing() {
        let source =
            "if a:  # one\n    # two\n\n    b()\n# three\n\nelse:\n        # four\n    c()\n";
        assert_eq!(line(source), "if a { b() } else { c() }");
    }

    #[test]
    fn spaces_around_the_words_of_a_line_change_nothing() {
        let source = "if  a :\n    b()\nelse   if c  :\n    d()\nelse :\n    e()\n";
        assert_eq!(line(source), "if a { b() } else if c { d() } else { e() }");
    }

    #[test]
    fn parentheses_around_a_condition_are_never_required() {
        assert_eq!(line("if (a):\n    b()\n"), "if a { b() }");
        assert_eq!(
            line("while (a) and (b):\n    c()\n"),
            "while (a and b) { c() }"
        );
    }

    #[test]
    fn a_long_chain_of_else_if_is_one_statement() {
        let source = format!("if a:\n    b()\n{}", "else if a:\n    b()\n".repeat(1_000));
        assert_eq!(statements(&source).len(), 1);
    }

    #[test]
    fn a_statement_that_opens_a_block_spans_its_line() {
        let statements = statements("if a < 3:  # one\n    b()\nc()\n");
        assert_eq!(statements.len(), 2);
        let first = statements.into_iter().next().unwrap();
        assert_eq!(first.span, span(1, 1, 10));
        let StatementKind::If { branches, .. } = first.kind else {
            panic!("an if expected");
        };
        assert_eq!(branches[0].span, span(1, 1, 10));
        assert_eq!(branches[0].body[0].span, span(2, 5, 8));
    }

    #[test]
    fn each_branch_spans_its_line() {
        let source = "if a:\n    b()\nelse if c:\n    d()\nelse:\n    e()\n";
        let StatementKind::If { branches, .. } = statements(source).pop().unwrap().kind else {
            panic!("an if expected");
        };
        assert_eq!(branches[0].span, span(1, 1, 6));
        assert_eq!(branches[1].span, span(3, 1, 11));
    }

    #[test]
    fn a_nested_statement_spans_its_line_without_the_indentation() {
        let source = "while a:\n    repeat 3:\n        b()\n";
        let StatementKind::While { body, .. } = statements(source).pop().unwrap().kind else {
            panic!("a while expected");
        };
        assert_eq!(body[0].span, span(2, 5, 14));
    }
}

mod block_errors {
    use super::*;

    #[test]
    fn a_missing_colon_is_an_error_at_the_end_of_the_line() {
        for (source, line, column) in [
            ("if a\n    b()\n", 1, 5),
            ("if a", 1, 5),
            ("if a  # one\n    b()\n", 1, 5),
            ("while a\nb()\n", 1, 8),
            ("repeat 3\n    b()\n", 1, 9),
            ("if a:\n    b()\nelse\n    c()\n", 3, 5),
            ("if a:\n    b()\nelse if c\n    d()\n", 3, 10),
            ("while a:\n    if b\n        c()\n", 2, 9),
        ] {
            let error = error(source);
            assert_eq!(error.kind, E::ExpectedColon, "{source}");
            assert_eq!(error.span, span(line, column, column), "{source}");
        }
    }

    #[test]
    fn a_missing_colon_is_an_error_at_what_stands_there() {
        let error = error("if a b():\n    c()\n");
        assert_eq!(error.kind, E::ExpectedColon);
        assert_eq!(error.span, span(1, 6, 7));
        let error = error_of_else("else c:\n    d()\n");
        assert_eq!(error.kind, E::ExpectedColon);
        assert_eq!(error.span, span(3, 6, 7));
    }

    /// The error of an `if` with its block followed by `rest`, from line 3.
    fn error_of_else(rest: &str) -> ParseError {
        error(&format!("if a:\n    b()\n{rest}"))
    }

    #[test]
    fn a_parenthesis_too_many_before_the_colon_is_its_own_error() {
        assert_eq!(error_kind("if a):\n    b()\n"), E::UnmatchedParen);
    }

    #[test]
    fn something_after_the_colon_is_an_error_up_to_the_end_of_the_line() {
        for (source, line, start, end) in [
            ("if a: b()\n", 1, 7, 10),
            ("while a: b() # one\n", 1, 10, 13),
            ("repeat 3: forward()", 1, 11, 20),
            ("if a:\n    b()\nelse: c()\n", 3, 7, 10),
            ("if a: if b: c()\n", 1, 7, 16),
            ("if a::\n    b()\n", 1, 6, 7),
            ("if a: let\n", 1, 7, 10),
        ] {
            let error = error(source);
            assert_eq!(error.kind, E::AfterColon, "{source}");
            assert_eq!(error.span, span(line, start, end), "{source}");
        }
    }

    #[test]
    fn a_word_of_another_language_after_the_colon_is_its_own_error() {
        let error = error("if a: pass\n");
        assert_eq!(
            error.kind,
            E::ForeignWord {
                word: ForeignWord::Pass
            }
        );
        assert_eq!(error.span, span(1, 7, 11));
    }

    #[test]
    fn a_word_of_another_language_further_after_the_colon_is_part_of_what_follows() {
        let error = error("if a: b() pass\n");
        assert_eq!(error.kind, E::AfterColon);
        assert_eq!(error.span, span(1, 7, 15));
    }

    #[test]
    fn a_block_without_a_line_is_an_error_at_the_line_that_opens_it() {
        for (source, line, start, end) in [
            ("if a:\nb()\n", 1, 1, 6),
            ("if a:", 1, 1, 6),
            ("if a:\n", 1, 1, 6),
            ("while a:\n\n\nb()\n", 1, 1, 9),
            ("repeat 3:\nb()\n", 1, 1, 10),
            ("while a:\n    if b:\n    c()\n", 2, 5, 10),
            ("while a:\n    c()\n    if b:\nd()\n", 3, 5, 10),
            ("while a:\n    if b:\n", 2, 5, 10),
            ("if a:\n    b()\nelse:\nc()\n", 3, 1, 6),
            ("if a:\n    b()\nelse if c:\nd()\n", 3, 1, 11),
        ] {
            let error = error(source);
            assert_eq!(error.kind, E::EmptyBlock, "{source}");
            assert_eq!(error.span, span(line, start, end), "{source}");
        }
    }

    #[test]
    fn a_block_of_comments_is_an_error_at_its_first_comment() {
        for (source, line, start, end) in [
            ("if a:\n    # todo\nb()\n", 2, 5, 11),
            ("if a:\n    # todo", 2, 5, 11),
            ("if a:\n    # todo\n", 2, 5, 11),
            ("if a:\n\n    # one\n    # two\nb()\n", 3, 5, 10),
            ("while a:\n    if b:\n        # todo\n    c()\n", 3, 9, 15),
            ("while a:\n    if b:\n        # todo\nc()\n", 3, 9, 15),
            ("if a:\n    b()\nelse:\n    # todo\nc()\n", 4, 5, 11),
        ] {
            let error = error(source);
            assert_eq!(error.kind, E::CommentOnlyBlock, "{source}");
            assert_eq!(error.span, span(line, start, end), "{source}");
        }
    }

    #[test]
    fn a_comment_indented_further_than_the_opening_line_is_in_the_block() {
        // Whatever its indentation, as long as it is further.
        for source in [
            "if a:\n  # todo\nb()\n",
            "if a:\n            # todo\nb()\n",
            "if a:\n\t# todo\nb()\n",
        ] {
            assert_eq!(error_kind(source), E::CommentOnlyBlock, "{source}");
        }
    }

    #[test]
    fn a_comment_at_the_level_of_the_opening_line_is_not_in_the_block() {
        for source in [
            "if a:\n# next\nb()\n",
            "if a:\n\n# next\nb()\n",
            "if a:\n# next",
            "while a:\n    if b:\n    # todo\n    c()\n",
            "while a:\n    if b:\n# todo\n    c()\n",
        ] {
            assert_eq!(error_kind(source), E::EmptyBlock, "{source}");
        }
    }

    #[test]
    fn a_comment_in_the_block_wins_over_one_at_the_level_of_the_opening_line() {
        let error = error("if a:\n# next\n    # todo\n\n# next\nb()\n");
        assert_eq!(error.kind, E::CommentOnlyBlock);
        assert_eq!(error.span, span(3, 5, 11));
    }

    #[test]
    fn only_the_comments_under_the_colon_and_above_the_next_line_count() {
        for source in [
            "if a: # todo\nb()\n",
            "    # above\nif a:\nb()\n",
            "if a:\nb()\n    # below\n",
            "if c:\n    # in another block\n    d()\nif a:\nb()\n",
        ] {
            assert_eq!(error_kind(source), E::EmptyBlock, "{source}");
        }
    }

    #[test]
    fn a_line_indented_under_a_statement_of_a_block_is_an_error() {
        let error = error("if a:\n    b()\n        c()\n");
        assert_eq!(error.kind, E::UnexpectedIndent);
        assert_eq!(error.span, span(3, 1, 9));
    }

    #[test]
    fn an_else_without_its_if_is_an_error() {
        for (source, line, column) in [
            ("else:\n    a()\n", 1, 1),
            ("else if a:\n    b()\n", 1, 1),
            ("else", 1, 1),
            ("while a:\n    b()\nelse:\n    c()\n", 3, 1),
            ("if a:\n    b()\nc()\nelse:\n    d()\n", 4, 1),
            // Indented further than its `if`: it follows `b()`.
            ("if a:\n    b()\n    else:\n        c()\n", 3, 5),
            // Indented less than its `if`: it follows the block of the `while`.
            ("while a:\n    if b:\n        c()\nelse:\n    d()\n", 4, 1),
        ] {
            let error = error(source);
            assert_eq!(error.kind, E::ElseWithoutIf, "{source}");
            assert_eq!(error.span, span(line, column, column + 4), "{source}");
        }
    }

    #[test]
    fn nothing_follows_the_else_of_an_if() {
        for rest in ["else if d:\n    e()\n", "else:\n    e()\n", "else"] {
            let error = error(&format!("if a:\n    b()\nelse:\n    c()\n{rest}"));
            assert_eq!(error.kind, E::ElseAfterElse, "{rest}");
            assert_eq!(error.span, span(5, 1, 5), "{rest}");
        }
    }

    #[test]
    fn a_missing_condition_is_a_missing_value() {
        for (source, column) in [("if:", 3), ("while :", 7), ("repeat:", 7), ("if", 3)] {
            let error = error(source);
            assert_eq!(error.kind, E::ExpectedValue, "{source}");
            assert_eq!(error.span.start, Position::new(1, column), "{source}");
        }
        let error = error_of_else("else if:\n    c()\n");
        assert_eq!(error.kind, E::ExpectedValue);
    }

    #[test]
    fn the_errors_of_an_expression_come_first() {
        assert_eq!(error_kind("if x = 3:\n    a()\n"), E::AssignInExpression);
        assert_eq!(
            error_kind("while 1 < x < 5:\n    a()\n"),
            E::ChainedComparison
        );
        let error = error_of_else("else if x = 3:\n    a()\n");
        assert_eq!(error.kind, E::AssignInExpression);
    }

    #[test]
    fn a_parenthesis_not_closed_before_the_colon_is_an_error_at_the_parenthesis() {
        for (source, column) in [
            ("if front_is_clear(:\n    a()\n", 18),
            ("repeat (3:\n    a()\n", 8),
            ("while (a:\n    b()\n", 7),
            ("if a(b:\n    c()\n", 5),
            ("if a(b, :\n    c()\n", 5),
            ("if (a + (b):\n    c()\n", 4),
        ] {
            let error = error(source);
            assert_eq!(error.kind, E::UnclosedParen, "{source}");
            assert_eq!(error.span, span(1, column, column + 1), "{source}");
        }
    }

    #[test]
    fn a_colon_after_a_statement_that_opens_no_block_is_an_error() {
        for (source, column) in [("forward():", 10), ("break:", 6), ("let a = 3:", 10)] {
            let error = error(source);
            assert_eq!(error.kind, E::UnexpectedToken, "{source}");
            assert_eq!(error.span, span(1, column, column + 1), "{source}");
        }
    }

    #[test]
    fn a_keyword_that_opens_a_block_is_not_assigned() {
        for (source, keyword) in [
            ("if = 3", Keyword::If),
            ("else = 3", Keyword::Else),
            ("while = 3", Keyword::While),
            ("repeat = 3", Keyword::Repeat),
        ] {
            assert_eq!(error_kind(source), E::KeywordAsName { keyword }, "{source}");
        }
    }

    #[test]
    fn elif_is_a_word_of_another_language() {
        let error = error_of_else("elif c:\n    d()\n");
        assert_eq!(
            error.kind,
            E::ForeignWord {
                word: ForeignWord::Elif
            }
        );
        assert_eq!(error.span, span(3, 1, 5));
    }

    #[test]
    fn an_error_inside_a_block_comes_through() {
        let error = error("while a:\n    if b:\n        let = 3\n");
        assert_eq!(error.kind, E::ExpectedName);
        assert_eq!(error.span, span(3, 13, 14));
    }

    #[test]
    fn an_error_above_a_block_comes_first() {
        assert_eq!(error_kind("let = 3\nif a:\nb()\n"), E::ExpectedName);
    }
}

mod depth {
    use super::*;

    const MAX: usize = MAX_DEPTH as usize;

    /// The number of levels of the tree of `expression`, 1 for a value alone.
    fn height(expression: &Expression) -> usize {
        1 + match &expression.kind {
            ExpressionKind::Call { arguments, .. } => {
                arguments.iter().map(height).max().unwrap_or(0)
            }
            ExpressionKind::Unary { operand, .. } => height(operand),
            ExpressionKind::Binary { left, right, .. } => height(left).max(height(right)),
            _ => 0,
        }
    }

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
        assert_eq!(height(&expression(&chain(MAX))), MAX);
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
        let error = error(&parentheses(MAX));
        assert_eq!(error.kind, E::TooDeep);
        let column = u32::try_from(MAX).unwrap() + 1;
        assert_eq!(
            error.span,
            span(1, column, column + 1),
            "what the parenthesis too many holds"
        );
    }

    #[test]
    fn the_tree_is_never_deeper_than_the_limit() {
        // Parentheses leave no node: the tree is less deep than the levels counted.
        assert_eq!(height(&expression(&parentheses(MAX - 1))), 1);
        let source = format!("{}true", "not ".repeat(MAX - 1));
        assert_eq!(height(&expression(&source)), MAX);
        let source = format!("{}1{}", "f(".repeat(MAX - 1), ")".repeat(MAX - 1));
        assert_eq!(height(&expression(&source)), MAX);
    }

    #[test]
    fn a_call_is_as_deep_as_its_deepest_value() {
        let call = |count: usize| format!("f(1, {}, 1) + 1", chain(count));
        assert_eq!(height(&expression(&call(MAX - 2))), MAX);
        assert_eq!(error_kind(&call(MAX - 1)), E::TooDeep);
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

    /// `count` blocks one inside the other, the innermost holding `body`.
    fn blocks(count: usize, body: &str) -> String {
        let mut source = String::new();
        for level in 0..count {
            source.push_str(&" ".repeat(level * 4));
            source.push_str("if a:\n");
        }
        source.push_str(&" ".repeat(count * 4));
        source.push_str(body);
        source
    }

    /// The number of levels of the tree of `statement`: its blocks, then its deepest
    /// expression.
    pub(super) fn statement_height(statement: &Statement) -> usize {
        let block = |body: &[Statement]| 1 + body.iter().map(statement_height).max().unwrap_or(0);
        match &statement.kind {
            StatementKind::Let { value, .. }
            | StatementKind::Assign { value, .. }
            | StatementKind::Return { value: Some(value) }
            | StatementKind::Expression(value) => height(value),
            StatementKind::Break | StatementKind::Return { value: None } => 0,
            StatementKind::If {
                branches,
                otherwise,
            } => branches
                .iter()
                .map(|branch| height(&branch.condition).max(block(&branch.body)))
                .chain(otherwise.as_deref().map(block))
                .max()
                .unwrap_or(0),
            StatementKind::While { condition, body } => height(condition).max(block(body)),
            StatementKind::Repeat { count, body } => height(count).max(block(body)),
        }
    }

    #[test]
    fn blocks_as_deep_as_the_limit() {
        let mut statements = statements(&blocks(MAX - 1, "b()"));
        assert_eq!(statement_height(&statements.pop().unwrap()), MAX);
        let error = error(&blocks(MAX, "b()"));
        assert_eq!(error.kind, E::TooDeep);
        let column = u32::try_from((MAX - 1) * 4 + 1).unwrap();
        assert_eq!(
            error.span,
            span(MAX_DEPTH, column, column + 5),
            "the line that opens the block too many"
        );
    }

    #[test]
    fn a_block_too_deep_is_an_error_whatever_it_holds() {
        assert_eq!(error_kind(&blocks(MAX, "break")), E::TooDeep);
    }

    #[test]
    fn the_blocks_and_the_expressions_add_up() {
        for count in [1, 2, 50, MAX - 1] {
            let fits = blocks(count, &chain(MAX - count));
            let mut statements = statements(&fits);
            assert_eq!(statement_height(&statements.pop().unwrap()), MAX, "{count}");
            let too_deep = blocks(count, &chain(MAX - count + 1));
            assert_eq!(error_kind(&too_deep), E::TooDeep, "{count}");
        }
    }

    #[test]
    fn a_condition_is_as_deep_as_the_line_of_its_block() {
        let inside = " ".repeat(44);
        let outside = " ".repeat(40);
        for line in [
            "if {}:\n",
            "while {}:\n",
            "repeat {}:\n",
            "if a:\n{inside}b()\n{outside}else if {}:\n",
        ] {
            let nested = |condition: &str| {
                let line = line
                    .replace("{}", condition)
                    .replace("{inside}", &inside)
                    .replace("{outside}", &outside);
                blocks(10, &format!("{line}{inside}b()"))
            };
            assert!(parse(&nested(&chain(MAX - 10))).is_ok(), "{line}");
            assert_eq!(error_kind(&nested(&chain(MAX - 9))), E::TooDeep, "{line}");
        }
    }

    #[test]
    fn every_kind_of_block_counts() {
        let nested = |count: usize| {
            let mut source = String::new();
            for level in 0..count {
                source.push_str(&" ".repeat(level * 4));
                source.push_str(match level % 3 {
                    0 => "if a:\n",
                    1 => "while a:\n",
                    _ => "repeat 3:\n",
                });
            }
            source + &" ".repeat(count * 4) + "b()"
        };
        assert!(parse(&nested(MAX - 1)).is_ok());
        assert_eq!(error_kind(&nested(MAX)), E::TooDeep);
    }

    #[test]
    fn the_blocks_of_an_else_count_as_those_of_an_if() {
        // Each `else:` holds the next `if`: one level per `else`.
        let nested = |count: usize| {
            let mut source = String::new();
            for level in 0..count {
                let indent = " ".repeat(level * 4);
                source += &[
                    format!("{indent}if a:\n"),
                    format!("{indent}    b()\n"),
                    format!("{indent}else:\n"),
                ]
                .concat();
            }
            source + &" ".repeat(count * 4) + "b()"
        };
        assert!(parse(&nested(MAX - 1)).is_ok());
        assert_eq!(error_kind(&nested(MAX)), E::TooDeep);
    }

    #[test]
    fn the_depth_starts_again_after_a_block() {
        let source = format!("{}\n{}\n", blocks(MAX - 1, "b()"), chain(MAX));
        assert_eq!(statements(&source).len(), 2);
    }

    #[test]
    fn a_huge_nesting_of_blocks_is_an_error_and_never_an_overflow() {
        assert_eq!(error_kind(&blocks(1_000, "b()")), E::TooDeep);
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

mod robustness {
    use super::depth::statement_height;
    use super::*;

    const MAX: usize = MAX_DEPTH as usize;

    /// A small generator of numbers, the same at every run, without a dependency.
    struct Random(u64);

    impl Random {
        fn next(&mut self) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            usize::try_from(self.0 >> 33).unwrap()
        }

        fn pick<'a>(&mut self, pieces: &[&'a str]) -> &'a str {
            pieces[self.next() % pieces.len()]
        }
    }

    /// Checks that the tree of `source`, when it has one, is never deeper than the limit, and
    /// returns whether it has one. A panic or a loop without end fails the test by itself.
    fn bounded(source: &str) -> bool {
        let Ok(program) = parse(source) else {
            return false;
        };
        for statement in &program.statements {
            assert!(statement_height(statement) <= MAX, "{source}");
        }
        true
    }

    #[test]
    fn any_pieces_give_a_tree_or_an_error() {
        const PIECES: [&str; 32] = [
            "if a:\n",
            "if ",
            "else:\n",
            "else if a:\n",
            "else",
            "while a:\n",
            "repeat 3:\n",
            "function",
            "    ",
            "        ",
            " ",
            "\n",
            "# c\n",
            "    # c\n",
            ":",
            "(",
            ")",
            ",",
            "=",
            "==",
            "+",
            "-",
            "not ",
            "and",
            "a",
            "3",
            "b()\n",
            "let a = 1\n",
            "break\n",
            "return",
            "pass",
            "elif",
        ];
        let mut random = Random(1);
        let mut read = 0;
        for _ in 0..30_000 {
            let length = random.next() % 12;
            let source: String = (0..length).map(|_| random.pick(&PIECES)).collect();
            read += usize::from(bounded(&source));
        }
        assert!(read > 1_000, "only {read} sources read");
    }

    #[test]
    fn any_lines_at_any_indentation_give_a_tree_or_an_error() {
        const LINES: [&str; 12] = [
            "if a:",
            "else if b:",
            "else:",
            "while a and not b:",
            "repeat 3:",
            "b()",
            "b()",
            "let c = (1 + 2) * 3",
            "break",
            "# c",
            "",
            "if a: b()",
        ];
        let mut random = Random(2);
        let mut read = 0;
        for _ in 0..30_000 {
            let mut source = String::new();
            let mut level = 0;
            for _ in 0..random.next() % 10 {
                let line = random.pick(&LINES);
                source.push_str(&" ".repeat(level * 4));
                source.push_str(line);
                source.push('\n');
                // Most often the level a right program takes, sometimes any other.
                level = match random.next() % 8 {
                    0 => random.next() % 4,
                    1 | 2 => level.saturating_sub(1),
                    _ if line.ends_with(':') => level + 1,
                    _ => level,
                };
            }
            read += usize::from(bounded(&source));
        }
        assert!(read > 1_000, "only {read} sources read");
    }

    #[test]
    fn deep_blocks_and_expressions_give_a_tree_or_an_error() {
        let mut random = Random(3);
        let mut read = 0;
        let mut refused = 0;
        for _ in 0..300 {
            let mut source = String::new();
            let blocks = random.next() % (MAX + 20);
            for level in 0..blocks {
                source.push_str(&" ".repeat(level * 4));
                source.push_str(random.pick(&["if a:\n", "while a:\n", "repeat 3:\n"]));
            }
            source.push_str(&" ".repeat(blocks * 4));
            let count = random.next() % (MAX + 20);
            let expression = match random.next() % 3 {
                0 => vec!["1"; count + 1].join(" + "),
                1 => format!("{}1{}", "(".repeat(count), ")".repeat(count)),
                _ => format!("{}true", "not ".repeat(count)),
            };
            source.push_str(&expression);
            if bounded(&source) {
                read += 1;
            } else {
                assert_eq!(error_kind(&source), E::TooDeep);
                refused += 1;
            }
        }
        assert!(read > 20 && refused > 20, "{read} read, {refused} refused");
    }
}
