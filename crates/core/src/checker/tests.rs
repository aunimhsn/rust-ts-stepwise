//! The tests of the checker, grouped by the rules of the specifications.

use super::{CheckError, CheckErrorKind as E, ReservedWord, check, is_snake_case};
use crate::builtin::Builtin;
use crate::color::Color;
use crate::parser::parse;
use crate::span::Span;
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

fn unknown(name: &str) -> E {
    let name = name.to_owned();
    E::UnknownName { name }
}

fn undeclared(name: &str) -> E {
    let name = name.to_owned();
    E::UndeclaredVariableChanged { name }
}

fn before_let(name: &str, declared: Span) -> E {
    let name = name.to_owned();
    E::VariableBeforeLet { name, declared }
}

fn after_block(name: &str, declared: Span) -> E {
    let name = name.to_owned();
    E::VariableAfterBlock { name, declared }
}

fn main_variable(name: &str) -> E {
    let name = name.to_owned();
    E::MainVariableInFunction { name }
}

fn already_declared(name: &str, first: Span) -> E {
    let name = name.to_owned();
    E::AlreadyDeclared { name, first }
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
print(total)
let total = 0
let total = 1
if true:
    let step = 1
step = 2
function walk():
    total = sidse
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
                CheckError {
                    kind: before_let("total", span(8, 5, 10)),
                    span: span(7, 7, 12),
                },
                CheckError {
                    kind: already_declared("total", span(8, 5, 10)),
                    span: span(9, 5, 10),
                },
                CheckError {
                    kind: after_block("step", span(11, 9, 13)),
                    span: span(12, 1, 5),
                },
                CheckError {
                    kind: main_variable("total"),
                    span: span(14, 5, 10),
                },
                CheckError {
                    kind: unknown("sidse"),
                    span: span(14, 13, 18),
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
    fn a_reserved_name_written_twice_keeps_its_one_error_each_time() {
        let source = "function paint(red, red):\n    forward()\n";
        assert_eq!(kinds(source), [reserved("red"), reserved("red")]);
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

mod variables {
    use super::*;

    #[test]
    fn a_variable_is_read_and_changed_after_its_let() {
        assert_valid("let sides = 0\nsides = sides + 1\nprint(sides)\n");
    }

    #[test]
    fn a_variable_is_seen_in_the_blocks_under_its_let() {
        assert_valid(
            "\
let sides = 0
while sides < 4:
    if sides == 2:
        sides = sides + 2
    else:
        repeat sides:
            print(sides)
",
        );
    }

    #[test]
    fn a_color_is_a_value() {
        assert_valid("let paint = red\nif here_is(blue):\n    start_painting(paint)\n");
    }

    #[test]
    fn the_names_of_every_expression_are_checked() {
        let source = "\
if a:
    forward()
else if b:
    forward()
while not c:
    forward()
repeat d:
    forward()
print(e, -f)
function walk():
    return g + 1
h
";
        let names = ["a", "b", "c", "d", "e", "f", "g", "h"];
        assert_eq!(kinds(source), names.map(unknown));
    }

    #[test]
    fn a_name_that_is_refused_is_still_declared() {
        assert_eq!(kinds("let Sides = 0\nprint(Sides)\n"), [E::NotSnakeCase]);
    }
}

mod unknown_names {
    use super::*;

    #[test]
    fn a_name_read_that_is_no_variable_is_an_error() {
        assert_eq!(
            error("let sides = 0\nsides = sidse + 1\n"),
            CheckError {
                kind: unknown("sidse"),
                span: span(2, 9, 14),
            }
        );
    }

    #[test]
    fn a_name_changed_that_is_no_variable_is_an_error_of_its_own() {
        assert_eq!(
            error("let sides = 0\nsidse = sides + 1\n"),
            CheckError {
                kind: undeclared("sidse"),
                span: span(2, 1, 6),
            }
        );
    }

    #[test]
    fn every_use_of_a_name_that_is_no_variable_has_its_error() {
        // Changing a name declares nothing: each line where the name doesn't exist says so.
        let source = "\
count = 0
while count < 3:
    count = count + 1
print(count)
";
        assert_eq!(
            kinds(source),
            [
                undeclared("count"),
                unknown("count"),
                undeclared("count"),
                unknown("count"),
                unknown("count")
            ]
        );
    }

    #[test]
    fn the_errors_of_a_line_are_in_its_order() {
        assert_eq!(
            errors("return sidse\n"),
            [
                CheckError {
                    kind: E::ReturnOutsideFunction,
                    span: span(1, 1, 13),
                },
                CheckError {
                    kind: unknown("sidse"),
                    span: span(1, 8, 13),
                },
            ]
        );
        assert_eq!(
            kinds("sidse = sidsa\n"),
            [undeclared("sidse"), unknown("sidsa")]
        );
    }

    #[test]
    fn both_sides_of_an_operator_are_checked() {
        assert_eq!(
            errors("print(1 + sidse)\n"),
            [CheckError {
                kind: unknown("sidse"),
                span: span(1, 11, 16),
            }]
        );
        assert_eq!(
            kinds("print(sidse * 2 - sidsa)\n"),
            [unknown("sidse"), unknown("sidsa")]
        );
    }

    #[test]
    fn the_value_given_to_a_name_that_is_refused_is_still_checked() {
        assert_eq!(
            kinds("let red = sidse\n"),
            [reserved("red"), unknown("sidse")]
        );
        assert_eq!(
            kinds("forward = sidse\n"),
            [reserved("forward"), unknown("sidse")]
        );
        assert_eq!(
            kinds("let Sides = sidse\n"),
            [E::NotSnakeCase, unknown("sidse")]
        );
    }

    #[test]
    fn a_condition_is_checked_before_its_block() {
        let source = "\
if sidse:
    let Sides = 0
else if sidsa:
    forward()
";
        assert_eq!(
            kinds(source),
            [unknown("sidse"), E::NotSnakeCase, unknown("sidsa")]
        );
    }

    #[test]
    fn the_values_of_a_call_are_checked_but_not_its_function() {
        // Whether the function exists is checked with the functions.
        assert_eq!(kinds("paint_side(sidse)\n"), [unknown("sidse")]);
    }

    #[test]
    fn a_built_in_function_without_parentheses_is_left_to_the_check_of_the_functions() {
        assert_valid("let go = forward\n");
    }
}

mod variables_before_their_let {
    use super::*;

    #[test]
    fn a_variable_read_above_its_let_is_an_error_that_gives_its_let() {
        assert_eq!(
            error("print(sides)\nlet sides = 0\n"),
            CheckError {
                kind: before_let("sides", span(2, 5, 10)),
                span: span(1, 7, 12),
            }
        );
    }

    #[test]
    fn a_variable_changed_above_its_let_has_the_same_error() {
        assert_eq!(
            error("sides = 1\nlet sides = 0\n"),
            CheckError {
                kind: before_let("sides", span(2, 5, 10)),
                span: span(1, 1, 6),
            }
        );
    }

    #[test]
    fn the_value_of_a_let_is_read_before_the_variable_exists() {
        assert_eq!(
            error("let sides = sides + 1\n"),
            CheckError {
                kind: before_let("sides", span(1, 5, 10)),
                span: span(1, 13, 18),
            }
        );
    }

    #[test]
    fn the_let_may_be_in_a_block_around() {
        let source = "\
while true:
    if true:
        print(sides)
    let sides = 0
";
        assert_eq!(
            error(source),
            CheckError {
                kind: before_let("sides", span(4, 9, 14)),
                span: span(3, 15, 20),
            }
        );
    }

    #[test]
    fn the_let_of_the_innermost_block_comes_first() {
        let source = "\
while true:
    if true:
        print(sides)
        let sides = 1
    let sides = 2
";
        assert_eq!(kinds(source), [before_let("sides", span(4, 13, 18))]);
    }

    #[test]
    fn a_let_in_a_block_further_down_is_another_variable() {
        let source = "print(sides)\nif true:\n    let sides = 0\n";
        assert_eq!(kinds(source), [unknown("sides")]);
    }

    #[test]
    fn a_function_has_the_same_rule() {
        let source = "function count():\n    print(total)\n    let total = 0\n";
        assert_eq!(kinds(source), [before_let("total", span(3, 9, 14))]);
    }
}

mod variables_after_their_block {
    use super::*;

    #[test]
    fn a_variable_read_after_its_block_is_an_error_that_gives_its_let() {
        assert_eq!(
            error("if true:\n    let sides = 0\nprint(sides)\n"),
            CheckError {
                kind: after_block("sides", span(2, 9, 14)),
                span: span(3, 7, 12),
            }
        );
    }

    #[test]
    fn a_variable_changed_after_its_block_has_the_same_error() {
        assert_eq!(
            error("while true:\n    let sides = 0\nsides = 1\n"),
            CheckError {
                kind: after_block("sides", span(2, 9, 14)),
                span: span(3, 1, 6),
            }
        );
    }

    #[test]
    fn the_let_given_is_the_closest_above() {
        let source = "\
if true:
    let sides = 0
else:
    let sides = 1
print(sides)
";
        assert_eq!(kinds(source), [after_block("sides", span(4, 9, 14))]);
    }

    #[test]
    fn a_variable_of_a_branch_is_not_seen_by_the_branches_after_it() {
        let source = "\
if true:
    let side = 1
else if side == 1:
    forward()
else:
    side = 2
";
        let declared = span(2, 9, 13);
        assert_eq!(
            errors(source),
            [
                CheckError {
                    kind: after_block("side", declared),
                    span: span(3, 9, 13),
                },
                CheckError {
                    kind: after_block("side", declared),
                    span: span(6, 5, 9),
                },
            ]
        );
    }

    #[test]
    fn a_name_is_free_again_after_a_function() {
        assert_valid("function count():\n    let total = 0\nlet total = 1\nprint(total)\n");
    }

    #[test]
    fn a_variable_declared_further_down_comes_before_one_of_a_function() {
        let source = "function count():\n    let total = 0\nprint(total)\nlet total = 1\n";
        assert_eq!(kinds(source), [before_let("total", span(4, 5, 10))]);
    }

    #[test]
    fn a_name_is_free_again_after_its_block() {
        assert_valid(
            "\
if true:
    let sides = 0
else:
    let sides = 1
let sides = 2
print(sides)
",
        );
    }

    #[test]
    fn a_variable_of_a_loop_is_declared_once() {
        assert_valid("repeat 3:\n    let step = 1\n    print(step)\n");
    }

    #[test]
    fn a_variable_declared_further_down_comes_before_one_whose_block_ended() {
        let source = "\
if true:
    let sides = 1
print(sides)
let sides = 2
";
        assert_eq!(kinds(source), [before_let("sides", span(4, 5, 10))]);
    }

    #[test]
    fn the_body_of_a_function_is_a_block() {
        let source = "function count():\n    let total = 0\nprint(total)\n";
        assert_eq!(kinds(source), [after_block("total", span(2, 9, 14))]);
    }

    #[test]
    fn a_parameter_is_unknown_in_the_main_program() {
        let source = "function walk(length):\n    forward()\nprint(length)\n";
        assert_eq!(kinds(source), [unknown("length")]);
    }
}

mod variables_of_the_main_program {
    use super::*;

    #[test]
    fn a_function_does_not_see_them() {
        assert_eq!(
            error("let sides = 0\nfunction walk():\n    print(sides)\n"),
            CheckError {
                kind: main_variable("sides"),
                span: span(3, 11, 16),
            }
        );
    }

    #[test]
    fn wherever_their_let_is() {
        let source = "\
function walk():
    sides = sides + 1
while true:
    let sides = 0
";
        assert_eq!(
            kinds(source),
            [main_variable("sides"), main_variable("sides")]
        );
    }

    #[test]
    fn in_every_branch_of_an_if() {
        let source = "\
function walk():
    print(first, second, third)
if true:
    let first = 1
else if true:
    let second = 2
else:
    let third = 3
";
        let names = ["first", "second", "third"];
        assert_eq!(kinds(source), names.map(main_variable));
    }

    #[test]
    fn even_when_their_block_has_ended() {
        let source = "\
if true:
    let sides = 0
function walk():
    print(sides)
";
        assert_eq!(kinds(source), [main_variable("sides")]);
    }

    #[test]
    fn a_variable_of_the_function_whose_block_ended_comes_first() {
        let source = "\
let total = 0
function count():
    if true:
        let total = 1
    print(total)
";
        assert_eq!(kinds(source), [after_block("total", span(4, 13, 18))]);
    }

    #[test]
    fn a_parameter_written_twice_is_one_variable() {
        let source = "function walk(side, side):\n    print(side)\n";
        assert_eq!(kinds(source), [duplicate("side")]);
    }

    #[test]
    fn a_parameter_with_a_reserved_name_is_no_variable() {
        // The name read in the body is the color.
        let source = "function walk(red):\n    print(red)\n";
        assert_eq!(kinds(source), [reserved("red")]);
    }

    #[test]
    fn a_function_in_a_function_does_not_know_the_variables_around_it() {
        // Accepted: they are unknown names rather than variables the function doesn't see,
        // since the function is refused where it is, and its errors go away with it (see
        // the backlog).
        let source = "\
function walk(length):
    let step = 0
    function turn():
        print(length, step)
";
        assert_eq!(
            kinds(source),
            [E::NestedFunction, unknown("length"), unknown("step")]
        );
    }

    #[test]
    fn a_function_may_declare_a_variable_of_the_same_name() {
        assert_valid(
            "\
let sides = 0
function walk():
    let sides = 1
    print(sides)
print(sides)
",
        );
    }

    #[test]
    fn the_variables_of_the_function_come_first() {
        let source = "\
let total = 0
function count():
    print(total)
    let total = 1
";
        assert_eq!(kinds(source), [before_let("total", span(4, 9, 14))]);
    }

    #[test]
    fn a_parameter_is_a_variable_of_its_function() {
        assert_valid(
            "\
function walk(length):
    length = length - 1
    if length > 0:
        return length
",
        );
    }

    #[test]
    fn a_function_does_not_see_the_variables_of_another() {
        let source = "\
function count():
    let total = 0
function walk(length):
    print(total, length)
function turn():
    print(length)
";
        assert_eq!(kinds(source), [unknown("total"), unknown("length")]);
    }

    #[test]
    fn a_function_in_a_block_does_not_see_the_block() {
        let source = "\
if true:
    let sides = 0
    function walk():
        print(sides)
    print(sides)
";
        assert_eq!(kinds(source), [E::NestedFunction, main_variable("sides")]);
    }
}

mod variables_declared_twice {
    use super::*;

    #[test]
    fn a_second_let_in_the_block_is_an_error_that_gives_the_first() {
        assert_eq!(
            error("let sides = 0\nlet sides = 1\n"),
            CheckError {
                kind: already_declared("sides", span(1, 5, 10)),
                span: span(2, 5, 10),
            }
        );
    }

    #[test]
    fn a_let_of_a_variable_of_a_block_around_is_an_error() {
        let source = "let sides = 0\nwhile true:\n    if true:\n        let sides = 1\n";
        assert_eq!(kinds(source), [already_declared("sides", span(1, 5, 10))]);
    }

    #[test]
    fn every_let_after_the_first_gives_the_first() {
        let source = "let sides = 0\nlet sides = 1\nlet sides = 2\n";
        let first = span(1, 5, 10);
        assert_eq!(
            kinds(source),
            [
                already_declared("sides", first),
                already_declared("sides", first)
            ]
        );
    }

    #[test]
    fn a_let_of_a_parameter_is_an_error() {
        let source = "\
function walk(length):
    let length = 1
    if true:
        let length = 2
";
        let first = span(1, 15, 21);
        assert_eq!(
            kinds(source),
            [
                already_declared("length", first),
                already_declared("length", first)
            ]
        );
    }

    #[test]
    fn the_first_of_a_parameter_written_twice_is_the_first_one() {
        let source = "function walk(side, side):\n    let side = 1\n";
        assert_eq!(
            kinds(source),
            [duplicate("side"), already_declared("side", span(1, 15, 19))]
        );
    }

    #[test]
    fn the_error_of_the_second_let_comes_before_those_of_its_value() {
        assert_eq!(
            errors("let sides = 0\nlet sides = sidse\n"),
            [
                CheckError {
                    kind: already_declared("sides", span(1, 5, 10)),
                    span: span(2, 5, 10),
                },
                CheckError {
                    kind: unknown("sidse"),
                    span: span(2, 13, 18),
                },
            ]
        );
    }

    #[test]
    fn the_value_of_the_second_let_reads_the_first_variable() {
        let source = "let sides = 0\nif true:\n    let sides = sides + 1\n";
        assert_eq!(kinds(source), [already_declared("sides", span(1, 5, 10))]);
    }

    #[test]
    fn a_name_that_is_not_in_snake_case_is_declared_once_too() {
        assert_eq!(
            kinds("let Sides = 0\nlet Sides = 1\n"),
            [
                E::NotSnakeCase,
                E::NotSnakeCase,
                already_declared("Sides", span(1, 5, 10))
            ]
        );
    }

    #[test]
    fn a_reserved_name_keeps_its_one_error() {
        assert_eq!(
            kinds("let red = 1\nlet red = 2\nforward = 3\nprint(red)\n"),
            [reserved("red"), reserved("red"), reserved("forward")]
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
    fn random_programs_never_panic_and_give_their_errors_in_the_order_of_the_source() {
        let lines = [
            "break",
            "return",
            "return 3",
            "return sidse",
            "let Sides = 0",
            "let red = 1",
            "forward = 2",
            "sides = sides + 1",
            "let sides = 0",
            "let sides = sides + 1",
            "let sides = sidse",
            "print(sides, total)",
            "let total = 0",
            "total = step",
            "let step = total",
            "forward()",
            "if true:",
            "if total == 0:",
            "else:",
            "else if sides == step:",
            "while sides < 3:",
            "repeat step:",
            "function walk(side, side):",
            "function run(total):",
            "function Run():",
        ];
        let mut random = Random(7);
        let mut with_several_errors = 0;
        for _ in 0..2000 {
            // The indentation follows the blocks, so that most programs have a right syntax.
            let mut source = String::new();
            let mut level = 0;
            let mut opens = false;
            for _ in 0..=(random.next() % 12) {
                if opens {
                    level += 1;
                } else {
                    level -= random.next() % (level + 1);
                }
                let line = random.pick(&lines);
                source.push_str(&" ".repeat(level * 4));
                source.push_str(line);
                source.push('\n');
                opens = line.ends_with(':');
            }
            if let Ok(program) = parse(&source) {
                let errors = check(&program);
                assert!(
                    errors.is_sorted_by_key(|error| error.span.start),
                    "errors out of order in {source:?}"
                );
                if errors.len() >= 2 {
                    with_several_errors += 1;
                }
            }
        }
        assert!(
            with_several_errors > 200,
            "only {with_several_errors} programs had several errors"
        );
    }
}
