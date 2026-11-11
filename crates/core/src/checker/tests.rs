//! The tests of the checker, grouped by the rules of the specifications.

use super::{CheckError, CheckErrorKind as E, ReservedWord, check, is_snake_case};
use crate::builtin::Builtin;
use crate::color::Color;
use crate::parser::parse;
use crate::test_support::span;

/// The errors of the checker for `source`, whose syntax must be right.
#[track_caller]
fn errors(source: &str) -> Vec<CheckError> {
    check(&parse(source).unwrap())
}

/// The kinds of the errors of the checker for `source`, whose syntax must be right.
#[track_caller]
fn kinds(source: &str) -> Vec<E> {
    errors(source).into_iter().map(|error| error.kind).collect()
}

/// The only error of the checker for `source`.
#[track_caller]
fn error(source: &str) -> CheckError {
    let mut errors = errors(source);
    assert_eq!(errors.len(), 1, "one error expected in {source:?}");
    errors.pop().unwrap()
}

/// Asserts that the checker finds no error in `source`.
#[track_caller]
fn assert_valid(source: &str) {
    assert_eq!(errors(source), [], "in {source:?}");
}

fn reserved(word: &str) -> E {
    let word = ReservedWord::from_word(word).unwrap();
    E::ReservedName { word }
}

fn duplicate(name: &str) -> E {
    let name = name.to_owned();
    E::DuplicateParameter { name }
}

mod programs {
    use super::*;

    #[test]
    fn an_empty_program_has_no_error() {
        assert_valid("");
        assert_valid("# Nothing yet\n");
    }

    #[test]
    fn the_program_of_the_specifications_has_no_error() {
        assert_valid(
            "\
# Paint a square of side 3
function paint_side(length):
    start_painting(red)
    repeat length:
        forward()
    stop_painting()

let sides = 0
while sides < 4:
    paint_side(3)
    turn_right()
    sides = sides + 1

if front_is_clear():
    forward()
else if left_is_clear():
    turn_left()
else:
    print(\"I am stuck\")
",
        );
    }

    #[test]
    fn every_error_is_found_in_the_order_of_the_source() {
        let source = "\
break
let Sides = 0
if true:
    function red(side, side):
        break
return
";
        assert_eq!(
            errors(source),
            [
                CheckError {
                    kind: E::BreakOutsideLoop,
                    span: span(1, 1, 6),
                },
                CheckError {
                    kind: E::NotSnakeCase,
                    span: span(2, 5, 10),
                },
                CheckError {
                    kind: E::NestedFunction,
                    span: span(4, 5, 30),
                },
                CheckError {
                    kind: reserved("red"),
                    span: span(4, 14, 17),
                },
                CheckError {
                    kind: duplicate("side"),
                    span: span(4, 24, 28),
                },
                CheckError {
                    kind: E::BreakOutsideLoop,
                    span: span(5, 9, 14),
                },
                CheckError {
                    kind: E::ReturnOutsideFunction,
                    span: span(6, 1, 7),
                },
            ]
        );
    }
}

mod breaks {
    use super::*;

    #[test]
    fn a_break_leaves_a_while_or_a_repeat() {
        assert_valid("while true:\n    break\n");
        assert_valid("repeat 3:\n    break\n");
    }

    #[test]
    fn a_break_in_an_if_of_a_loop_leaves_the_loop() {
        assert_valid(
            "\
while true:
    if front_is_clear():
        forward()
    else if left_is_clear():
        break
    else:
        break
",
        );
    }

    #[test]
    fn a_break_in_a_loop_of_a_function_leaves_the_loop() {
        assert_valid("function walk():\n    repeat 3:\n        break\n");
    }

    #[test]
    fn a_break_at_the_top_level_is_an_error() {
        assert_eq!(
            error("forward()\nbreak\n"),
            CheckError {
                kind: E::BreakOutsideLoop,
                span: span(2, 1, 6),
            }
        );
    }

    #[test]
    fn a_break_in_an_if_alone_is_an_error() {
        let source = "if true:\n    break\nelse:\n    break\n";
        assert_eq!(kinds(source), [E::BreakOutsideLoop, E::BreakOutsideLoop]);
    }

    #[test]
    fn a_break_after_a_loop_is_an_error() {
        assert_eq!(
            error("while true:\n    forward()\nbreak\n").span,
            span(3, 1, 6)
        );
    }

    #[test]
    fn a_break_in_a_function_without_loop_is_an_error() {
        assert_eq!(
            error("function walk():\n    break\n"),
            CheckError {
                kind: E::BreakOutsideLoop,
                span: span(2, 5, 10),
            }
        );
    }

    #[test]
    fn a_function_in_a_loop_takes_no_loop_from_around_it() {
        let source = "while true:\n    function walk():\n        break\n";
        assert_eq!(kinds(source), [E::NestedFunction, E::BreakOutsideLoop]);
    }
}

mod returns {
    use super::*;

    #[test]
    fn a_return_leaves_a_function_with_a_value_or_without() {
        assert_valid("function stop():\n    return\n");
        assert_valid("function double(number):\n    return number * 2\n");
    }

    #[test]
    fn a_return_in_the_blocks_of_a_function_leaves_the_function() {
        assert_valid(
            "\
function walk():
    while true:
        if front_is_clear():
            forward()
        else:
            return
",
        );
    }

    #[test]
    fn a_return_at_the_top_level_is_an_error() {
        assert_eq!(
            error("return\n"),
            CheckError {
                kind: E::ReturnOutsideFunction,
                span: span(1, 1, 7),
            }
        );
    }

    #[test]
    fn the_error_of_a_return_covers_its_value() {
        assert_eq!(
            error("return 3 + 4  # Too early\n"),
            CheckError {
                kind: E::ReturnOutsideFunction,
                span: span(1, 1, 13),
            }
        );
    }

    #[test]
    fn a_return_in_a_block_of_the_main_program_is_an_error() {
        let source = "while true:\n    if true:\n        return\n";
        assert_eq!(error(source).span, span(3, 9, 15));
    }

    #[test]
    fn a_return_after_a_function_is_an_error() {
        let source = "function stop():\n    return\nreturn\n";
        assert_eq!(error(source).span, span(3, 1, 7));
    }
}

mod nested_functions {
    use super::*;

    #[test]
    fn functions_are_defined_at_the_top_level_wherever_among_the_statements() {
        assert_valid("forward()\nfunction walk():\n    forward()\nturn_left()\n");
    }

    #[test]
    fn a_function_in_a_block_is_an_error_on_its_line() {
        assert_eq!(
            error("if true:\n    function walk(steps):  # Here\n        forward()\n"),
            CheckError {
                kind: E::NestedFunction,
                span: span(2, 5, 26),
            }
        );
    }

    #[test]
    fn a_function_in_any_block_is_an_error() {
        for opening in ["if true:", "while true:", "repeat 3:"] {
            let source = format!("{opening}\n    function walk():\n        forward()\n");
            assert_eq!(kinds(&source), [E::NestedFunction], "{opening}");
        }
        let source = "\
if true:
    forward()
else if false:
    function walk():
        forward()
else:
    function run():
        forward()
";
        assert_eq!(kinds(source), [E::NestedFunction, E::NestedFunction]);
    }

    #[test]
    fn a_function_in_a_function_is_an_error() {
        let source = "function walk():\n    function run():\n        forward()\n";
        assert_eq!(
            error(source),
            CheckError {
                kind: E::NestedFunction,
                span: span(2, 5, 20),
            }
        );
    }

    #[test]
    fn the_body_of_a_nested_function_is_checked_as_a_function() {
        let source = "\
if true:
    function walk(side, side):
        return
";
        assert_eq!(kinds(source), [E::NestedFunction, duplicate("side")]);
    }
}

mod snake_case {
    use super::*;

    #[test]
    fn the_names_of_the_specifications_are_in_snake_case() {
        for name in ["sides", "paint_side", "count_2", "a", "a1", "a_", "a__b"] {
            assert!(is_snake_case(name), "{name}");
        }
    }

    #[test]
    fn the_other_names_are_not() {
        for name in [
            "paintSide",
            "Sides",
            "SIDES",
            "côté",
            "_sides",
            "2sides",
            "paint-side",
            "paint side",
            "сторона",
            "辺",
            "",
        ] {
            assert!(!is_snake_case(name), "{name}");
        }
    }

    #[test]
    fn the_name_of_a_variable_is_in_snake_case() {
        assert_eq!(
            error("let paintSide = 3\n"),
            CheckError {
                kind: E::NotSnakeCase,
                span: span(1, 5, 14),
            }
        );
    }

    #[test]
    fn the_name_of_a_function_is_in_snake_case() {
        assert_eq!(
            error("function PaintSide():\n    forward()\n"),
            CheckError {
                kind: E::NotSnakeCase,
                span: span(1, 10, 19),
            }
        );
    }

    #[test]
    fn the_name_of_a_parameter_is_in_snake_case() {
        assert_eq!(
            error("function paint_side(length, côté):\n    forward()\n"),
            CheckError {
                kind: E::NotSnakeCase,
                span: span(1, 29, 33),
            }
        );
    }

    #[test]
    fn a_name_starting_with_an_underscore_is_not_in_snake_case() {
        assert_eq!(kinds("let _sides = 3\n"), [E::NotSnakeCase]);
    }

    #[test]
    fn the_form_of_a_name_is_checked_where_it_is_declared_only() {
        assert!(!kinds("Sides = 3\n").contains(&E::NotSnakeCase));
        assert!(!kinds("print(paintSide)\n").contains(&E::NotSnakeCase));
    }

    #[test]
    fn each_name_of_a_line_has_its_error() {
        let source = "function paintSide(sideLength, Color):\n    forward()\n";
        let spans: Vec<_> = errors(source).iter().map(|error| error.span).collect();
        assert_eq!(spans, [span(1, 10, 19), span(1, 20, 30), span(1, 32, 37)]);
    }
}

mod reserved_names {
    use super::*;

    #[test]
    fn a_reserved_word_is_a_built_in_function_or_a_color() {
        assert_eq!(
            ReservedWord::from_word("forward"),
            Some(ReservedWord::Builtin(Builtin::Forward))
        );
        assert_eq!(
            ReservedWord::from_word("blue"),
            Some(ReservedWord::Color(Color::Blue))
        );
        assert_eq!(ReservedWord::from_word("sides"), None);
        assert_eq!(ReservedWord::from_word("Forward"), None);
    }

    #[test]
    fn every_reserved_word_reads_back_from_its_word() {
        let builtins = Builtin::ALL.into_iter().map(ReservedWord::Builtin);
        let colors = Color::ALL.into_iter().map(ReservedWord::Color);
        for word in builtins.chain(colors) {
            assert_eq!(ReservedWord::from_word(word.as_str()), Some(word));
        }
    }

    #[test]
    fn no_variable_takes_the_name_of_a_built_in_function() {
        for builtin in Builtin::ALL {
            let source = format!("let {} = 3\n", builtin.as_str());
            let word = ReservedWord::Builtin(builtin);
            assert_eq!(kinds(&source), [E::ReservedName { word }], "{builtin:?}");
        }
    }

    #[test]
    fn no_variable_takes_the_name_of_a_color() {
        for color in Color::ALL {
            let source = format!("let {} = 3\n", color.as_str());
            let word = ReservedWord::Color(color);
            assert_eq!(kinds(&source), [E::ReservedName { word }], "{color:?}");
        }
    }

    #[test]
    fn the_error_of_a_reserved_name_is_on_the_name() {
        assert_eq!(
            error("let red = 3\n"),
            CheckError {
                kind: reserved("red"),
                span: span(1, 5, 8),
            }
        );
    }

    #[test]
    fn no_function_takes_a_reserved_name() {
        assert_eq!(
            error("function print():\n    forward()\n"),
            CheckError {
                kind: reserved("print"),
                span: span(1, 10, 15),
            }
        );
        assert_eq!(
            kinds("function yellow():\n    forward()\n"),
            [reserved("yellow")]
        );
    }

    #[test]
    fn no_parameter_takes_a_reserved_name() {
        let source = "function paint(blue, drop):\n    forward()\n";
        assert_eq!(kinds(source), [reserved("blue"), reserved("drop")]);
        let source = "function paint(length, drop):\n    forward()\n";
        assert_eq!(
            error(source),
            CheckError {
                kind: reserved("drop"),
                span: span(1, 24, 28),
            }
        );
    }

    #[test]
    fn a_reserved_name_is_given_no_value() {
        assert_eq!(
            error("forward = 3\n"),
            CheckError {
                kind: reserved("forward"),
                span: span(1, 1, 8),
            }
        );
        assert_eq!(kinds("red = blue\n"), [reserved("red")]);
    }

    #[test]
    fn a_name_that_holds_a_reserved_word_is_free() {
        assert_valid("let reds = 3\nlet forward_2 = 4\nlet my_print = 5\n");
    }
}

mod duplicate_parameters {
    use super::*;

    #[test]
    fn parameters_of_different_names_are_right() {
        assert_valid("function paint(length, width, color_name):\n    forward()\n");
    }

    #[test]
    fn a_parameter_named_as_one_before_it_is_an_error() {
        assert_eq!(
            error("function paint(side, side):\n    forward()\n"),
            CheckError {
                kind: duplicate("side"),
                span: span(1, 22, 26),
            }
        );
    }

    #[test]
    fn each_parameter_after_the_first_of_its_name_has_its_error() {
        let source = "function paint(side, length, side, side):\n    forward()\n";
        let spans: Vec<_> = errors(source).iter().map(|error| error.span).collect();
        assert_eq!(spans, [span(1, 30, 34), span(1, 36, 40)]);
    }

    #[test]
    fn two_functions_may_name_their_parameters_alike() {
        assert_valid(
            "\
function paint(side):
    forward()
function erase(side):
    forward()
",
        );
    }

    #[test]
    fn a_duplicate_that_is_not_in_snake_case_has_both_errors() {
        let source = "function paint(Side, Side):\n    forward()\n";
        assert_eq!(
            kinds(source),
            [E::NotSnakeCase, E::NotSnakeCase, duplicate("Side")]
        );
    }
}

mod robustness {
    use super::*;
    use crate::parser::{MAX_BLOCK_DEPTH, MAX_DEPTH};
    use crate::test_support::Random;

    #[test]
    fn the_deepest_blocks_are_checked() {
        let depth = usize::try_from(MAX_BLOCK_DEPTH).unwrap();
        let indent = |level: usize| " ".repeat(level * 4);
        let mut lines: Vec<String> = (0..depth)
            .map(|level| format!("{}if true:", indent(level)))
            .collect();
        lines.push(format!("{}break", indent(depth)));
        let source = lines.join("\n");
        assert_eq!(kinds(&source), [E::BreakOutsideLoop]);
    }

    #[test]
    fn a_program_with_the_deepest_expression_is_checked() {
        let depth = usize::try_from(MAX_DEPTH).unwrap() - 2;
        let source = format!("let Deep = {}1{}\n", "(".repeat(depth), ")".repeat(depth));
        assert_eq!(kinds(&source), [E::NotSnakeCase]);
    }

    #[test]
    fn many_parameters_of_the_same_name_each_have_their_error() {
        let parameters = vec!["side"; 50_000].join(", ");
        let source = format!("function paint({parameters}):\n    forward()\n");
        assert_eq!(errors(&source).len(), 49_999);
    }

    #[test]
    fn random_programs_never_panic() {
        let lines = [
            "break",
            "return",
            "return 3",
            "let Sides = 0",
            "let red = 1",
            "forward = 2",
            "sides = sides + 1",
            "forward()",
            "if true:",
            "else:",
            "else if false:",
            "while true:",
            "repeat 3:",
            "function walk(side, side):",
            "function Run():",
        ];
        let mut random = Random(7);
        let mut checked = 0;
        for _ in 0..2000 {
            let mut source = String::new();
            for _ in 0..(random.next() % 12) {
                source.push_str(&" ".repeat((random.next() % 4) * 4));
                source.push_str(random.pick(&lines));
                source.push('\n');
            }
            if let Ok(program) = parse(&source) {
                let _ = check(&program);
                checked += 1;
            }
        }
        assert!(checked > 100, "only {checked} programs had a right syntax");
    }
}
