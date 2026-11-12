//! The tests of the interpreter: whole programs, with the lines highlighted step after step,
//! what they print and how they end, grouped by the rules of *Running a program*, *Values*,
//! *Expressions* and *Statements* in `specs.md`.

use super::{Event, Machine, RuntimeError, RuntimeErrorKind as E, State, ValueType};
use crate::ast::{BinaryOperator, Program, Statement, StatementKind, UnaryOperator};
use crate::checker::check;
use crate::parser::parse;
use crate::span::Span;
use crate::test_support::span;

/// The most steps a test runs: a program that never ends is still running after them.
const MAX_STEPS: usize = 1000;

/// What a program did, step after step.
#[derive(Debug)]
struct Run {
    /// The line highlighted before each step, in order.
    lines: Vec<u32>,
    /// The lines written by `print`, in order.
    printed: Vec<String>,
    /// Where the program is after its last step.
    state: State,
}

/// Runs `source`, which must pass the checks before running, for at most [`MAX_STEPS`].
#[track_caller]
fn run(source: &str) -> Run {
    let program = parse(source).unwrap();
    assert_eq!(check(&program), [], "in {source:?}");
    run_program(&program)
}

/// Runs `source` without the checks before running, which would refuse it.
#[track_caller]
fn run_unchecked(source: &str) -> Run {
    let program = parse(source).unwrap();
    assert_ne!(check(&program), [], "in {source:?}");
    run_program(&program)
}

/// Runs `program`, and checks at each step that the line highlighted is one the program
/// runs: a wrong place is caught even by a test that looks at something else.
#[track_caller]
fn run_program(program: &Program) -> Run {
    let mut steps = Vec::new();
    lines_run(&program.statements, &mut steps);
    let mut machine = Machine::new(program);
    let mut lines = Vec::new();
    let mut printed = Vec::new();
    for _ in 0..MAX_STEPS {
        let State::Running { line } = *machine.state() else {
            break;
        };
        assert!(steps.contains(&line), "{line:?} is no line of the program");
        lines.push(line.start.line);
        match machine.step() {
            Some(Event::Printed(text)) => printed.push(text),
            None => {}
        }
    }
    let state = machine.state().clone();
    Run {
        lines,
        printed,
        state,
    }
}

/// The lines a step may run: every statement but the definition of a function, and each
/// `else if`.
fn lines_run(statements: &[Statement], lines: &mut Vec<Span>) {
    for statement in statements {
        match &statement.kind {
            StatementKind::Function { .. } => {}
            StatementKind::If { branches, .. } => {
                lines.extend(branches.iter().map(|branch| branch.span));
            }
            _ => lines.push(statement.span),
        }
        for block in statement.blocks() {
            lines_run(block, lines);
        }
    }
}

/// The lines `source` prints, the program running to its end.
#[track_caller]
fn printed(source: &str) -> Vec<String> {
    let run = run(source);
    assert_eq!(run.state, State::Ended, "in {source:?}");
    run.printed
}

/// What `print` writes for `expression`.
#[track_caller]
fn value(expression: &str) -> String {
    let mut printed = printed(&format!("print({expression})"));
    assert_eq!(printed.len(), 1, "one line expected for {expression:?}");
    printed.pop().unwrap()
}

/// The error that stops `source`.
#[track_caller]
fn failure(source: &str) -> RuntimeError {
    match run(source).state {
        State::Failed(error) => error,
        state => panic!("{source:?} should stop on an error: {state:?}"),
    }
}

/// The kind of the error that stops `print(expression)`.
#[track_caller]
fn refused(expression: &str) -> E {
    failure(&format!("print({expression})")).kind
}

fn operands(operator: BinaryOperator, left: ValueType, right: ValueType) -> E {
    E::BinaryOperands {
        operator,
        left,
        right,
    }
}

fn no_value(function: &str) -> E {
    let function = function.to_owned();
    E::NoValue { function }
}

// Steps: one line run, the next one highlighted.

#[test]
fn a_program_without_statement_ends_before_any_step() {
    for source in ["", "\n\n", "# Nothing to do\n", "    # indented\n\n"] {
        let run = run(source);
        assert_eq!(run.lines, [], "in {source:?}");
        assert_eq!(run.state, State::Ended, "in {source:?}");
    }
}

#[test]
fn each_statement_is_a_step() {
    let run = run("let sides = 4\nsides = sides + 1\nprint(sides)\n");
    assert_eq!(run.lines, [1, 2, 3]);
    assert_eq!(run.printed, ["5"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn a_program_starts_paused_before_its_first_line() {
    let program = parse("# Count the sides\n\nlet sides = 4\n").unwrap();
    let machine = Machine::new(&program);
    let line = span(3, 1, 14);
    assert_eq!(*machine.state(), State::Running { line });
}

#[test]
fn the_line_of_a_block_is_highlighted_up_to_its_colon_included() {
    let source = "\
if 1 > 2:  # never
    print(1)
else if 2 > 1:
    while false:
        print(2)
    repeat 0:
        print(3)
";
    let program = parse(source).unwrap();
    let mut machine = Machine::new(&program);
    let mut lines = Vec::new();
    while let State::Running { line } = *machine.state() {
        lines.push(line);
        machine.step();
    }
    assert_eq!(
        lines,
        [
            span(1, 1, 10),
            span(3, 1, 15),
            span(4, 5, 17),
            span(6, 5, 14)
        ]
    );
}

#[test]
fn a_step_after_the_end_does_nothing() {
    let program = parse("print(1)\n").unwrap();
    let mut machine = Machine::new(&program);
    assert_eq!(machine.step(), Some(Event::Printed("1".to_owned())));
    for _ in 0..3 {
        assert_eq!(*machine.state(), State::Ended);
        assert_eq!(machine.step(), None);
    }
}

#[test]
fn a_step_after_an_error_does_nothing() {
    let program = parse("print(1 / 0)\nprint(2)\n").unwrap();
    let mut machine = Machine::new(&program);
    assert_eq!(machine.step(), None);
    let failed = machine.state().clone();
    assert!(matches!(failed, State::Failed(_)));
    for _ in 0..3 {
        assert_eq!(machine.step(), None);
        assert_eq!(*machine.state(), failed);
    }
}

#[test]
fn an_error_stops_the_program_on_its_line() {
    let run = run("print(\"before\")\nprint(1 / 0)\nprint(\"after\")\n");
    assert_eq!(run.lines, [1, 2]);
    assert_eq!(run.printed, ["before"]);
    let kind = E::DivisionByZero;
    let error = RuntimeError {
        kind,
        span: span(2, 7, 12),
    };
    assert_eq!(run.state, State::Failed(error));
}

#[test]
fn the_definition_of_a_function_is_no_step() {
    let source = "\
function unused():
    print(\"never\")

print(\"main\")
function other():
    print(\"never\")
";
    let run = run(source);
    assert_eq!(run.lines, [4]);
    assert_eq!(run.printed, ["main"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn a_program_of_definitions_only_ends_before_any_step() {
    let run = run("function unused():\n    print(\"never\")\n");
    assert_eq!(run.lines, []);
    assert_eq!(run.state, State::Ended);
}

// `if`, `else if`, `else`.

#[test]
fn an_if_runs_its_block_when_its_condition_is_true() {
    let source =
        |condition: &str| format!("if {condition}:\n    print(\"yes\")\nprint(\"after\")\n");
    let taken = run(&source("1 < 2"));
    assert_eq!(taken.lines, [1, 2, 3]);
    assert_eq!(taken.printed, ["yes", "after"]);
    let skipped = run(&source("1 > 2"));
    assert_eq!(skipped.lines, [1, 3]);
    assert_eq!(skipped.printed, ["after"]);
}

#[test]
fn the_line_of_an_else_if_is_a_step_when_its_condition_is_evaluated() {
    let source = |sides: i64| {
        format!(
            "\
let sides = {sides}
if sides == 1:
    print(\"one\")
else if sides == 2:
    print(\"two\")
else if sides == 3:
    print(\"three\")
else:
    print(\"many\")
print(\"done\")
"
        )
    };
    let first = run(&source(1));
    assert_eq!(first.lines, [1, 2, 3, 10]);
    assert_eq!(first.printed, ["one", "done"]);
    let second = run(&source(2));
    assert_eq!(second.lines, [1, 2, 4, 5, 10]);
    assert_eq!(second.printed, ["two", "done"]);
    let third = run(&source(3));
    assert_eq!(third.lines, [1, 2, 4, 6, 7, 10]);
    assert_eq!(third.printed, ["three", "done"]);
}

#[test]
fn the_line_of_an_else_is_never_a_step() {
    let source = "\
if false:
    print(\"if\")
else if false:
    print(\"else if\")
else:
    print(\"else\")
";
    let run = run(source);
    assert_eq!(run.lines, [1, 3, 6]);
    assert_eq!(run.printed, ["else"]);
}

#[test]
fn an_if_without_true_condition_and_without_else_runs_nothing() {
    let run = run("if false:\n    print(1)\nelse if false:\n    print(2)\n");
    assert_eq!(run.lines, [1, 3]);
    assert_eq!(run.printed, [""; 0]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn ifs_nest() {
    let source = "\
if true:
    if false:
        print(\"no\")
    else:
        if true:
            print(\"deep\")
    print(\"back\")
";
    let run = run(source);
    assert_eq!(run.lines, [1, 2, 5, 6, 7]);
    assert_eq!(run.printed, ["deep", "back"]);
}

// `while`.

#[test]
fn a_while_checks_its_condition_again_before_each_turn() {
    let source = "\
let turns = 0
while turns < 2:
    turns = turns + 1
print(turns)
";
    let run = run(source);
    assert_eq!(run.lines, [1, 2, 3, 2, 3, 2, 4]);
    assert_eq!(run.printed, ["2"]);
}

#[test]
fn a_while_whose_condition_is_false_runs_no_turn() {
    let run = run("while false:\n    print(\"never\")\nprint(\"after\")\n");
    assert_eq!(run.lines, [1, 3]);
    assert_eq!(run.printed, ["after"]);
}

#[test]
fn a_loop_that_never_ends_keeps_running() {
    let run = run("while true:\n    print(\"again\")\n");
    assert_eq!(run.lines.len(), MAX_STEPS);
    assert!(run.lines.chunks(2).all(|turn| turn == [1, 2]));
    assert_eq!(run.printed.len(), MAX_STEPS / 2);
    assert!(matches!(run.state, State::Running { .. }));
}

// `repeat`.

#[test]
fn the_line_of_a_repeat_is_a_step_before_each_turn_and_a_last_time() {
    let run = run("repeat 3:\n    print(\"turn\")\nprint(\"done\")\n");
    assert_eq!(run.lines, [1, 2, 1, 2, 1, 2, 1, 3]);
    assert_eq!(run.printed, ["turn", "turn", "turn", "done"]);
}

#[test]
fn a_repeat_of_zero_runs_no_turn() {
    let run = run("repeat 0:\n    print(\"never\")\nprint(\"after\")\n");
    assert_eq!(run.lines, [1, 3]);
    assert_eq!(run.printed, ["after"]);
}

#[test]
fn the_count_of_a_repeat_is_evaluated_once() {
    let source = "\
let turns = 2
repeat turns:
    turns = turns + 5
    print(turns)
";
    assert_eq!(printed(source), ["7", "12"]);
    assert_eq!(printed("repeat 1 + 1:\n    print(\"turn\")\n").len(), 2);
}

#[test]
fn repeats_nest_each_with_its_own_count() {
    let run = run("repeat 2:\n    repeat 3:\n        print(\"turn\")\n");
    assert_eq!(
        run.lines,
        [1, 2, 3, 2, 3, 2, 3, 2, 1, 2, 3, 2, 3, 2, 3, 2, 1]
    );
    assert_eq!(run.printed.len(), 6);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn the_count_of_a_repeat_is_a_whole_number() {
    let text = failure("repeat \"twice\":\n    print(1)\n");
    let found = ValueType::Text;
    assert_eq!(text.kind, E::RepeatCountNotNumber { found });
    assert_eq!(text.span, span(1, 8, 15));
    let boolean = failure("repeat true:\n    print(1)\n");
    let found = ValueType::Boolean;
    assert_eq!(boolean.kind, E::RepeatCountNotNumber { found });
}

#[test]
fn the_count_of_a_repeat_is_not_below_zero() {
    let error = failure("let turns = 2\nrepeat turns - 5:\n    print(1)\n");
    assert_eq!(error.kind, E::NegativeRepeatCount { count: -3 });
    assert_eq!(error.span, span(2, 8, 17));
    assert_eq!(
        failure("repeat -1:\n    print(1)\n").kind,
        E::NegativeRepeatCount { count: -1 }
    );
}

// `break`.

#[test]
fn a_break_leaves_a_while() {
    let source = "\
let turns = 0
while true:
    turns = turns + 1
    if turns == 3:
        break
    print(\"again\")
print(turns)
";
    let run = run(source);
    assert_eq!(run.lines, [1, 2, 3, 4, 6, 2, 3, 4, 6, 2, 3, 4, 5, 7]);
    assert_eq!(run.printed, ["again", "again", "3"]);
}

#[test]
fn a_break_leaves_a_repeat_with_turns_left() {
    let source = "\
repeat 5:
    print(\"once\")
    break
print(\"after\")
";
    let run = run(source);
    assert_eq!(run.lines, [1, 2, 3, 4]);
    assert_eq!(run.printed, ["once", "after"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn a_break_leaves_the_innermost_loop_only() {
    let source = "\
repeat 2:
    repeat 10:
        break
    print(\"outer\")
print(\"done\")
";
    let run = run(source);
    assert_eq!(run.lines, [1, 2, 3, 4, 1, 2, 3, 4, 1, 5]);
    assert_eq!(run.printed, ["outer", "outer", "done"]);
    let source = "\
let turns = 0
while turns < 2:
    turns = turns + 1
    while true:
        break
    print(turns)
";
    assert_eq!(printed(source), ["1", "2"]);
}

#[test]
fn a_break_ends_the_variables_of_its_loop() {
    let source = "\
let total = 0
while true:
    let side = 5
    if side == 5:
        let twice = side * 2
        total = total + twice
        break
let after = total + 1
print(total, after)
";
    assert_eq!(printed(source), ["10 11"]);
    let source = "\
repeat 3:
    let side = 5
    repeat 3:
        let twice = side * 2
        break
    print(side)
    break
let after = 1
print(after)
";
    assert_eq!(printed(source), ["5", "1"]);
}

// Variables.

#[test]
fn a_variable_keeps_its_value_until_it_is_changed() {
    let source = "\
let sides = 4
let name = \"square\"
print(name, sides)
sides = sides - 1
name = \"triangle\"
print(name, sides)
";
    assert_eq!(printed(source), ["square 4", "triangle 3"]);
}

#[test]
fn a_variable_holds_a_value_of_any_type() {
    let source = "\
let value = 3
value = \"three\"
value = value == \"three\"
print(value)
value = red
print(value)
";
    assert_eq!(printed(source), ["true", "red"]);
}

#[test]
fn a_variable_of_a_block_is_declared_again_at_each_turn() {
    let source = "\
let total = 0
repeat 3:
    let one = 1
    total = total + one
    one = one + 10
print(total)
";
    assert_eq!(printed(source), ["3"]);
}

#[test]
fn the_variables_of_blocks_that_have_ended_leave_their_place() {
    let source = "\
let total = 3
if total == 3:
    let message = \"three\"
    print(message)
else:
    let other = \"not three\"
    print(other)
let last = total * 2
if last == 6:
    let first = 1
    let second = 2
    while first < 3:
        let third = first + second
        first = first + 1
        print(third)
print(total, last)
";
    assert_eq!(printed(source), ["three", "3", "4", "3 6"]);
}

// `print`, and the values as it writes them.

#[test]
fn print_writes_its_values_on_one_line_separated_by_spaces() {
    assert_eq!(value(""), "");
    assert_eq!(value("\"Sides:\", 4"), "Sides: 4");
    assert_eq!(value("1, true, \"a\", blue"), "1 true a blue");
    assert_eq!(value("\"\", \"\""), " ");
}

#[test]
fn print_writes_each_value_as_it_is_written() {
    assert_eq!(value("42"), "42");
    assert_eq!(value("-7"), "-7");
    assert_eq!(value("- -7"), "7");
    assert_eq!(value("9223372036854775807"), "9223372036854775807");
    assert_eq!(value("-9223372036854775807 - 1"), "-9223372036854775808");
    assert_eq!(value("true"), "true");
    assert_eq!(value("false"), "false");
    assert_eq!(value("red"), "red");
    assert_eq!(value("yellow"), "yellow");
    assert_eq!(value("blue"), "blue");
}

#[test]
fn print_writes_a_text_without_its_quotes_nor_its_escapes() {
    assert_eq!(value("\"I am stuck\""), "I am stuck");
    assert_eq!(value(r#""say \"hi\"""#), "say \"hi\"");
    assert_eq!(value(r#""a\\b""#), "a\\b");
    assert_eq!(value(r#""two\nlines""#), "two\nlines");
    assert_eq!(value("\"côté 🤖\""), "côté 🤖");
}

#[test]
fn the_result_of_print_is_no_value() {
    let run = run("let line = print(\"a\")\nprint(\"after\")\n");
    assert_eq!(run.lines, [1]);
    assert_eq!(run.printed, ["a"]);
    let error = RuntimeError {
        kind: no_value("print"),
        span: span(1, 12, 22),
    };
    assert_eq!(run.state, State::Failed(error));
    assert_eq!(refused("print()"), no_value("print"));
    assert_eq!(
        failure("if print(1):\n    print(2)\n").kind,
        no_value("print")
    );
    assert_eq!(refused("1 + print(1)"), no_value("print"));
}

// Arithmetic.

#[test]
fn arithmetic_on_whole_numbers() {
    for (expression, result) in [
        ("1 + 2", "3"),
        ("7 - 10", "-3"),
        ("6 * 7", "42"),
        ("-6 * 7", "-42"),
        ("7 / 2", "3"),
        ("7 % 2", "1"),
        ("8 / 2", "4"),
        ("8 % 2", "0"),
        ("0 / 5", "0"),
        ("2 + 3 * 4", "14"),
        ("(2 + 3) * 4", "20"),
        ("10 - 4 - 3", "3"),
        ("-3 + 5", "2"),
    ] {
        assert_eq!(value(expression), result, "{expression}");
    }
}

#[test]
fn the_division_rounds_down_and_the_remainder_follows_it() {
    for (expression, result) in [
        ("-7 / 2", "-4"),
        ("-7 % 2", "1"),
        ("7 / -2", "-4"),
        ("7 % -2", "-1"),
        ("-7 / -2", "3"),
        ("-7 % -2", "-1"),
        ("-8 / 2", "-4"),
        ("-8 % 2", "0"),
        ("-1 / 4", "-1"),
        ("(0 - 1) % 4", "3"),
    ] {
        assert_eq!(value(expression), result, "{expression}");
    }
}

#[test]
fn dividing_by_zero_stops_the_program() {
    assert_eq!(refused("1 / 0"), E::DivisionByZero);
    assert_eq!(refused("1 % 0"), E::DivisionByZero);
    assert_eq!(refused("0 / 0"), E::DivisionByZero);
    let error = failure("let sides = 0\nlet each = 10 / (sides * 2)\n");
    assert_eq!(error.kind, E::DivisionByZero);
    assert_eq!(error.span, span(2, 12, 28));
}

#[test]
fn a_result_beyond_the_range_stops_the_program() {
    for expression in [
        "9223372036854775807 + 1",
        "-9223372036854775807 - 2",
        "9223372036854775807 * 2",
        "-(-9223372036854775807 - 1)",
        "(-9223372036854775807 - 1) / -1",
    ] {
        assert_eq!(refused(expression), E::Overflow, "{expression}");
    }
    assert_eq!(value("(-9223372036854775807 - 1) % -1"), "0");
    assert_eq!(value("9223372036854775806 + 1"), "9223372036854775807");
    let error = failure("print(1 + 9223372036854775807 * 2)\n");
    assert_eq!(error.span, span(1, 11, 34));
    let error = failure("let smallest = -9223372036854775807 - 1\nprint(-smallest)\n");
    assert_eq!(error.span, span(2, 7, 16));
}

#[test]
fn arithmetic_refuses_what_is_no_whole_number() {
    use BinaryOperator as B;
    use ValueType as T;
    for (expression, expected) in [
        ("\"a\" * 2", operands(B::Multiply, T::Text, T::Number)),
        ("true + 1", operands(B::Add, T::Boolean, T::Number)),
        ("4 - \"a\"", operands(B::Subtract, T::Number, T::Text)),
        ("red / blue", operands(B::Divide, T::Color, T::Color)),
        (
            "true % false",
            operands(B::Remainder, T::Boolean, T::Boolean),
        ),
        ("\"ab\" - \"b\"", operands(B::Subtract, T::Text, T::Text)),
    ] {
        assert_eq!(refused(expression), expected, "{expression}");
    }
    let error = failure("print(1 + (true * 2))\n");
    assert_eq!(error.span, span(1, 11, 21));
}

#[test]
fn a_minus_in_front_of_what_is_no_whole_number_is_refused() {
    let operator = UnaryOperator::Negate;
    for (expression, found) in [
        ("-\"a\"", ValueType::Text),
        ("-true", ValueType::Boolean),
        ("-red", ValueType::Color),
    ] {
        assert_eq!(
            refused(expression),
            E::UnaryOperand { operator, found },
            "{expression}"
        );
    }
    assert_eq!(failure("print(1, -\"a\")\n").span, span(1, 10, 14));
}

// Texts.

#[test]
fn plus_joins_two_texts() {
    assert_eq!(value("\"ab\" + \"cd\""), "abcd");
    assert_eq!(value("\"\" + \"\""), "");
    assert_eq!(value("\"a\" + \"b\" + \"c\""), "abc");
    assert_eq!(value("\"é\" + \"🤖\""), "é🤖");
}

#[test]
fn plus_refuses_a_text_and_a_number() {
    use BinaryOperator::Add;
    use ValueType as T;
    let error = failure("let sides = 4\nprint(\"Sides: \" + sides)\n");
    assert_eq!(error.kind, operands(Add, T::Text, T::Number));
    assert_eq!(error.span, span(2, 7, 24));
    assert_eq!(refused("4 + \"a\""), operands(Add, T::Number, T::Text));
    assert_eq!(refused("\"a\" + true"), operands(Add, T::Text, T::Boolean));
    assert_eq!(refused("red + red"), operands(Add, T::Color, T::Color));
}

#[test]
fn a_text_holds_at_most_ten_thousand_characters() {
    // Ten characters, then ten times as many at each turn: 10 000 after three turns.
    let ten_thousand = "\
let line = \"0123456789\"
repeat 3:
    line = line + line + line + line + line + line + line + line + line + line
";
    assert_eq!(
        printed(&format!("{ten_thousand}print(line == line)\n")),
        ["true"]
    );
    let error = failure(&format!("{ten_thousand}let longer = line + \"x\"\n"));
    assert_eq!(error.kind, E::TextTooLong);
    assert_eq!(error.span, span(4, 14, 24));
}

#[test]
fn a_loop_that_doubles_a_text_stops_on_its_length() {
    let run = run("let line = \"ab\"\nwhile true:\n    line = line + line\n");
    assert!(run.lines.len() < 40, "{:?}", run.lines);
    let error = RuntimeError {
        kind: E::TextTooLong,
        span: span(3, 12, 23),
    };
    assert_eq!(run.state, State::Failed(error));
}

// Comparisons.

#[test]
fn two_values_of_the_same_type_are_equal_or_not() {
    for (expression, result) in [
        ("1 == 1", "true"),
        ("1 == 2", "false"),
        ("1 != 2", "true"),
        ("1 != 1", "false"),
        ("\"a\" == \"a\"", "true"),
        ("\"a\" == \"A\"", "false"),
        ("\"a\" != \"b\"", "true"),
        ("\"a\" + \"b\" == \"ab\"", "true"),
        ("true == true", "true"),
        ("true == false", "false"),
        ("true != false", "true"),
        ("red == red", "true"),
        ("red == blue", "false"),
        ("red != yellow", "true"),
    ] {
        assert_eq!(value(expression), result, "{expression}");
    }
}

#[test]
fn two_values_of_different_types_are_not_compared() {
    use BinaryOperator as B;
    use ValueType as T;
    for (expression, expected) in [
        ("1 == \"1\"", operands(B::Equal, T::Number, T::Text)),
        ("true == 1", operands(B::Equal, T::Boolean, T::Number)),
        ("red != 1", operands(B::NotEqual, T::Color, T::Number)),
        ("\"red\" != red", operands(B::NotEqual, T::Text, T::Color)),
    ] {
        assert_eq!(refused(expression), expected, "{expression}");
    }
    assert_eq!(failure("print(1 == \"1\")\n").span, span(1, 7, 15));
}

#[test]
fn whole_numbers_are_ordered() {
    for (expression, result) in [
        ("1 < 2", "true"),
        ("2 < 2", "false"),
        ("2 <= 2", "true"),
        ("3 <= 2", "false"),
        ("3 > 2", "true"),
        ("2 > 2", "false"),
        ("2 >= 2", "true"),
        ("1 >= 2", "false"),
        ("-3 < 2", "true"),
        ("1 + 1 < 3 * 1", "true"),
    ] {
        assert_eq!(value(expression), result, "{expression}");
    }
}

#[test]
fn an_order_refuses_what_is_no_whole_number() {
    use BinaryOperator as B;
    use ValueType as T;
    for (expression, expected) in [
        ("\"a\" < \"b\"", operands(B::Less, T::Text, T::Text)),
        (
            "true <= false",
            operands(B::LessEqual, T::Boolean, T::Boolean),
        ),
        ("red > blue", operands(B::Greater, T::Color, T::Color)),
        ("1 >= \"1\"", operands(B::GreaterEqual, T::Number, T::Text)),
    ] {
        assert_eq!(refused(expression), expected, "{expression}");
    }
}

// Logic.

#[test]
fn logic_on_booleans() {
    for (expression, result) in [
        ("true and true", "true"),
        ("true and false", "false"),
        ("false and true", "false"),
        ("false and false", "false"),
        ("true or true", "true"),
        ("true or false", "true"),
        ("false or true", "true"),
        ("false or false", "false"),
        ("not true", "false"),
        ("not false", "true"),
        ("not not true", "true"),
        ("not 1 == 2", "true"),
        ("false or true and false", "false"),
        ("true or false and false", "true"),
        ("1 < 2 and 2 < 3", "true"),
    ] {
        assert_eq!(value(expression), result, "{expression}");
    }
}

#[test]
fn logic_stops_as_soon_as_the_result_is_known() {
    assert_eq!(value("false and 1 / 0 == 0"), "false");
    assert_eq!(value("true or 1 / 0 == 0"), "true");
    assert_eq!(refused("true and 1 / 0 == 0"), E::DivisionByZero);
    assert_eq!(refused("false or 1 / 0 == 0"), E::DivisionByZero);
    // The right side isn't evaluated: what it would give isn't known.
    assert_eq!(value("false and 3"), "false");
    assert_eq!(value("true or \"a\""), "true");
    assert_eq!(value("false and print(1)"), "false");
}

#[test]
fn logic_refuses_what_is_no_boolean() {
    use BinaryOperator::{And, Or};
    let refused_side = |operator, found| E::LogicOperand { operator, found };
    for (expression, expected) in [
        ("1 and true", refused_side(And, ValueType::Number)),
        ("true and 3", refused_side(And, ValueType::Number)),
        ("\"a\" or true", refused_side(Or, ValueType::Text)),
        ("false or red", refused_side(Or, ValueType::Color)),
        ("0 or 1", refused_side(Or, ValueType::Number)),
    ] {
        assert_eq!(refused(expression), expected, "{expression}");
    }
    // The span is the side refused.
    assert_eq!(failure("print(1 + 1 and true)\n").span, span(1, 7, 12));
    assert_eq!(failure("print(true and \"yes\")\n").span, span(1, 16, 21));
    assert_eq!(failure("print(false or (1 + 1))\n").span, span(1, 16, 23));
}

#[test]
fn not_refuses_what_is_no_boolean() {
    let operator = UnaryOperator::Not;
    for (expression, found) in [
        ("not 3", ValueType::Number),
        ("not \"a\"", ValueType::Text),
        ("not red", ValueType::Color),
    ] {
        assert_eq!(
            refused(expression),
            E::UnaryOperand { operator, found },
            "{expression}"
        );
    }
    assert_eq!(failure("print(not 3)\n").span, span(1, 7, 12));
}

// Conditions.

#[test]
fn a_condition_is_a_boolean() {
    let condition = |found| E::ConditionNotBoolean { found };
    let error = failure("if 3:\n    print(1)\n");
    assert_eq!(error.kind, condition(ValueType::Number));
    assert_eq!(error.span, span(1, 4, 5));
    let error = failure("if false:\n    print(1)\nelse if \"yes\":\n    print(2)\n");
    assert_eq!(error.kind, condition(ValueType::Text));
    assert_eq!(error.span, span(3, 9, 14));
    let error = failure("let paint = red\nwhile paint:\n    print(1)\n");
    assert_eq!(error.kind, condition(ValueType::Color));
    assert_eq!(error.span, span(2, 7, 12));
    let error = failure("if (1 + 1):\n    print(1)\n");
    assert_eq!(error.kind, condition(ValueType::Number));
    assert_eq!(error.span, span(1, 4, 11));
}

#[test]
fn a_while_stops_when_its_condition_is_no_boolean_anymore() {
    let source = "\
let going = true
while going:
    print(\"turn\")
    going = 0
";
    let run = run(source);
    assert_eq!(run.lines, [1, 2, 3, 4, 2]);
    let found = ValueType::Number;
    let error = RuntimeError {
        kind: E::ConditionNotBoolean { found },
        span: span(2, 7, 12),
    };
    assert_eq!(run.state, State::Failed(error));
}

// The depth of a program.

#[test]
fn the_deepest_expressions_and_blocks_run() {
    let sum = vec!["1"; 150].join(" + ");
    assert_eq!(value(&sum), "150");
    let nested = format!("{}1{}", "(".repeat(150), ")".repeat(150));
    assert_eq!(value(&nested), "1");
    let negated = format!("{}true", "not ".repeat(150));
    assert_eq!(value(&negated), "true");
    let mut blocks = String::new();
    for depth in 0..40 {
        blocks.push_str(&"    ".repeat(depth));
        blocks.push_str(if depth % 2 == 0 {
            "if true:\n"
        } else {
            "repeat 1:\n"
        });
    }
    blocks.push_str(&"    ".repeat(40));
    blocks.push_str("print(\"deep\")\n");
    assert_eq!(printed(&blocks), ["deep"]);
}

// A program the checks before running refuse.

#[test]
fn a_program_that_is_not_checked_does_not_run() {
    for (source, place) in [
        ("print(1)\nsides = 3\n", span(2, 1, 6)),
        ("print(1)\nprint(sidse)\n", span(2, 7, 12)),
        ("print(1)\nbreak\n", span(2, 1, 6)),
        ("print(1)\nreturn 3\n", span(2, 1, 9)),
        ("print(1)\nif true:\n    print(walk())\n", span(3, 11, 15)),
    ] {
        let run = run_unchecked(source);
        assert_eq!(run.lines, [], "in {source:?}");
        assert_eq!(run.printed, [""; 0], "in {source:?}");
        let kind = E::Internal;
        let error = RuntimeError { kind, span: place };
        assert_eq!(run.state, State::Failed(error), "in {source:?}");
    }
}

#[test]
fn an_expression_alone_on_its_line_leaves_no_value() {
    // Refused by the checks: it runs all the same, without leaving its value behind.
    let run = run_unchecked("repeat 2:\n    1 + 1\nprint(\"after\")\n");
    assert_eq!(run.lines, [1, 2, 1, 2, 1, 3]);
    assert_eq!(run.printed, ["after"]);
    assert_eq!(run.state, State::Ended);
}

// Until the next sub-steps.

#[test]
fn until_step_9b_a_function_of_the_program_is_not_called() {
    let source = "function walk():\n    print(\"a\")\n\nwalk()\n";
    let program = parse(source).unwrap();
    assert_eq!(check(&program), []);
    let machine = Machine::new(&program);
    let error = RuntimeError {
        kind: E::Internal,
        span: span(4, 1, 5),
    };
    assert_eq!(*machine.state(), State::Failed(error));
}

#[test]
fn until_step_9c_the_robot_does_not_act() {
    let run = run("print(\"before\")\nforward()\n");
    assert_eq!(run.lines, [1, 2]);
    let error = RuntimeError {
        kind: E::Internal,
        span: span(2, 1, 10),
    };
    assert_eq!(run.state, State::Failed(error));
}
