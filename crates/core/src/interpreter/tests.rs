//! The tests of the interpreter: whole programs, with the lines highlighted step after step,
//! what they print and how they end, grouped by the rules of *Running a program*, *Values*,
//! *Expressions*, *Statements* and *Functions* in `specs.md`.

use std::iter;

use super::code::{Code, Instruction};
use super::compiler::compile;
use super::{
    Event, MAX_CALL_DEPTH, Machine, RuntimeError, RuntimeErrorKind as E, State, ValueType,
};
use crate::ast::{BinaryOperator, Program, Statement, StatementKind, UnaryOperator};
use crate::checker::check;
use crate::parser::{MAX_BLOCK_DEPTH, MAX_DEPTH, ParseErrorKind, SyntaxError, parse};
use crate::span::Span;
use crate::test_support::{Random, span};

/// The most steps a test runs: a program that never ends is still running after them.
const MAX_STEPS: usize = 1000;

/// What a program did, step after step.
#[derive(Debug)]
struct Run {
    /// The line highlighted before each step, in order.
    lines: Vec<u32>,
    /// The lines written by `print`, in order.
    printed: Vec<String>,
    /// The line of each step that ended a function, in order: the line of its call was
    /// highlighted again after it.
    returned_from: Vec<u32>,
    /// Where the program is after its last step.
    state: State,
    /// The machine after its last step, with its calls in progress.
    machine: Machine,
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
/// runs and the one of the call being run, and first that its instructions follow the rule
/// that ends a step: a mistake of the compiler is caught even by a test that looks at
/// something else.
#[track_caller]
fn run_program(program: &Program) -> Run {
    assert_every_step_ends(program);
    let mut steps = Vec::new();
    lines_run(&program.statements, &mut steps);
    let mut machine = Machine::new(program);
    let mut lines = Vec::new();
    let mut printed = Vec::new();
    let mut returned_from = Vec::new();
    for _ in 0..MAX_STEPS {
        let State::Running { line, after_call } = *machine.state() else {
            break;
        };
        assert!(steps.contains(&line), "{line:?} is no line of the program");
        assert_calls_in_progress(&machine, line);
        if after_call {
            // The step before ended a function: it started on the line highlighted before.
            returned_from.extend(lines.last());
        }
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
        returned_from,
        state,
        machine,
    }
}

/// Checks the calls in progress of `machine`, paused on `line`: the main program first, then
/// functions only, never more than the limit, the last one being on the line highlighted.
#[track_caller]
fn assert_calls_in_progress(machine: &Machine, line: Span) {
    let calls = machine.calls();
    let functions: Vec<bool> = calls.iter().map(|call| call.function.is_some()).collect();
    assert_eq!(functions.first(), Some(&false), "the main program is first");
    assert!(functions.iter().skip(1).all(|&function| function));
    assert!(calls.len() <= MAX_CALL_DEPTH + 1, "{} calls", calls.len());
    assert_eq!(calls.last().map(|call| call.line), Some(line));
}

/// Checks the instructions of `program`, whatever its values: from the start, from each line
/// and from each call a function comes back to, every way reaches a pause or the end without
/// running an instruction twice. So a step never runs more instructions than the code holds,
/// the count [`Machine`] stops at, even on the ways no test takes.
#[track_caller]
fn assert_every_step_ends(program: &Program) {
    let Ok(code) = compile(program) else {
        return;
    };
    let starts_a_step = |index: &usize| {
        let instruction = code.get(*index).map(|(instruction, _)| instruction);
        matches!(instruction, Some(Instruction::Line | Instruction::Call(_)))
    };
    let after = (0..code.len()).filter(starts_a_step).map(|index| index + 1);
    let mut known = vec![None; code.len()];
    let mut on_the_way = vec![false; code.len()];
    for start in iter::once(0).chain(after) {
        let longest = longest_step(&code, start, &mut known, &mut on_the_way);
        assert!(longest <= code.len(), "a step of {longest} instructions");
    }
}

/// The most instructions a step may run from the one at `index`, included, up to a pause or
/// the end.
fn longest_step(
    code: &Code,
    index: usize,
    known: &mut [Option<usize>],
    on_the_way: &mut [bool],
) -> usize {
    let Some((instruction, _)) = code.get(index) else {
        panic!("a step leaves the instructions at {index}");
    };
    if let Some(longest) = known[index] {
        return longest;
    }
    assert!(!on_the_way[index], "a way back to {index} meets no pause");
    on_the_way[index] = true;
    let after = ways_on(instruction, index + 1);
    let after = after
        .into_iter()
        .map(|way| longest_step(code, way, known, on_the_way));
    let longest = after.max().unwrap_or(0) + 1;
    on_the_way[index] = false;
    known[index] = Some(longest);
    longest
}

/// Where a step may go on after `instruction`, the one after it being `next`. Every
/// instruction is listed, so that a new one that jumps can't be forgotten.
fn ways_on(instruction: &Instruction, next: usize) -> Vec<usize> {
    match instruction {
        // A pause or the end: the step is over.
        Instruction::Line | Instruction::End | Instruction::Return { .. } => vec![],
        Instruction::Jump(target) => vec![*target],
        Instruction::JumpUnless(target)
        | Instruction::RepeatNext(target)
        | Instruction::ShortCircuit { end: target, .. } => vec![next, *target],
        // The first instruction of the function: a line, where the step ends.
        Instruction::Call(function) => vec![function.entry],
        Instruction::Push(_)
        | Instruction::Load(_)
        | Instruction::Store(_)
        | Instruction::Declare(_)
        | Instruction::Truncate(_)
        | Instruction::Pop
        | Instruction::Unary(_)
        | Instruction::Binary(_)
        | Instruction::RequireBoolean(_)
        | Instruction::RepeatStart
        | Instruction::CallBuiltin { .. }
        | Instruction::TakeResult { .. } => vec![next],
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

/// The calls in progress of `machine`, each as the variables panel writes it: its function,
/// its line, then its variables, the ones the last step changed after a `*`.
fn calls(machine: &Machine) -> Vec<String> {
    let written = |call: &super::CallInProgress| {
        let function = call.function.unwrap_or("main program");
        let variables = call.variables.iter().map(|variable| {
            let changed = if variable.changed { "*" } else { "" };
            format!(" {changed}{}={}", variable.name, variable.value)
        });
        let variables: String = variables.collect();
        format!("{function} · Ln {}:{variables}", call.line.start.line)
    };
    machine.calls().iter().map(written).collect()
}

/// A machine running `source`, paused when the line `line` is highlighted for the first
/// time.
#[track_caller]
fn paused_on(source: &str, line: u32) -> Machine {
    let program = parse(source).unwrap();
    assert_eq!(check(&program), [], "in {source:?}");
    let mut machine = Machine::new(&program);
    for _ in 0..MAX_STEPS {
        match machine.state() {
            State::Running { line: next, .. } if next.start.line == line => return machine,
            State::Running { .. } => machine.step(),
            state => panic!("{source:?} never reaches the line {line}: {state:?}"),
        };
    }
    panic!("{source:?} doesn't reach the line {line} in {MAX_STEPS} steps");
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
    let (line, after_call) = (span(3, 1, 14), false);
    assert_eq!(*machine.state(), State::Running { line, after_call });
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
    while let State::Running { line, .. } = *machine.state() {
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
fn a_break_in_a_while_leaves_the_turns_of_the_repeat_around_it() {
    let source = "\
repeat 2:
    while true:
        break
    print(\"outer\")
";
    let run = run(source);
    assert_eq!(run.lines, [1, 2, 3, 4, 1, 2, 3, 4, 1]);
    assert_eq!(run.printed, ["outer", "outer"]);
    assert_eq!(run.state, State::Ended);
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

#[test]
fn a_name_is_declared_again_once_its_block_has_ended() {
    let source = "\
let total = 0
if true:
    let side = 1
    total = total + side
    print(side)
if true:
    let other = 10
    let side = 2
    total = total + side
    print(side, other)
repeat 2:
    let side = total
    print(side)
let side = 7
print(side, total)
";
    assert_eq!(printed(source), ["1", "2 10", "3", "3", "7 3"]);
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

// Functions: the calls and the lines highlighted.

#[test]
fn a_call_enters_the_function_then_comes_back_to_its_line() {
    let source = "\
function walk():
    print(\"a\")
    print(\"b\")

print(\"before\")
walk()
print(\"after\")
";
    let run = run(source);
    assert_eq!(run.lines, [5, 6, 2, 3, 6, 7]);
    assert_eq!(run.printed, ["before", "a", "b", "after"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn the_line_of_a_call_is_resumed_when_the_function_ends() {
    let source = "\
function double(n):
    return n * 2

let total = double(3) + 1
print(total)
";
    let program = parse(source).unwrap();
    let mut machine = Machine::new(&program);
    let mut states = Vec::new();
    while let State::Running { line, after_call } = *machine.state() {
        states.push((line, after_call));
        machine.step();
    }
    let call = span(4, 1, 26);
    assert_eq!(
        states,
        [
            (call, false),
            (span(2, 5, 17), false),
            (call, true),
            (span(5, 1, 13), false)
        ]
    );
    assert_eq!(*machine.state(), State::Ended);
}

#[test]
fn a_line_is_resumed_after_each_of_its_calls() {
    let source = "\
function double(n):
    return n * 2

print(double(1) + double(double(2)))
";
    let run = run(source);
    assert_eq!(run.lines, [4, 2, 4, 2, 4, 2, 4]);
    assert_eq!(run.returned_from, [2, 2, 2]);
    assert_eq!(run.printed, ["10"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn the_condition_of_a_while_calls_its_function_at_each_turn() {
    let source = "\
function below(turns):
    return turns < 2

let turns = 0
while below(turns):
    turns = turns + 1
";
    let run = run(source);
    assert_eq!(run.lines, [4, 5, 2, 5, 6, 5, 2, 5, 6, 5, 2, 5]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn the_count_of_a_repeat_calls_its_function_once() {
    let source = "\
function count():
    print(\"counted\")
    return 2

repeat count():
    print(\"turn\")
";
    let run = run(source);
    assert_eq!(run.lines, [5, 2, 3, 5, 6, 5, 6, 5]);
    assert_eq!(run.printed, ["counted", "turn", "turn"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn a_function_calls_the_functions_defined_above_it() {
    let source = "\
function paint_side(length):
    print(\"side\", length)

function paint_square(side):
    repeat 2:
        paint_side(side)

paint_square(3)
";
    let run = run(source);
    assert_eq!(run.lines, [8, 5, 6, 2, 6, 5, 6, 2, 6, 5, 8]);
    assert_eq!(run.printed, ["side 3", "side 3"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn logic_that_stops_early_calls_no_function_on_its_right() {
    let source = "\
function loud():
    print(\"called\")
    return true

print(false and loud())
print(true or loud())
print(true and loud())
";
    let run = run(source);
    assert_eq!(run.lines, [5, 6, 7, 2, 3, 7]);
    assert_eq!(run.printed, ["false", "true", "called", "true"]);
}

// Parameters and variables of a function.

#[test]
fn the_parameters_take_the_values_of_the_call_in_order() {
    let source = "\
function show(first, second, third):
    print(first, second, third)

function note(value):
    print(\"note\", value)
    return value

show(1, \"a\", red)
show(note(2) + 1, note(true), note(\"b\"))
";
    assert_eq!(
        printed(source),
        ["1 a red", "note 2", "note true", "note b", "3 true b"]
    );
}

#[test]
fn a_parameter_is_changed_as_a_variable_of_its_function() {
    let source = "\
function count_down(length):
    while length > 0:
        print(length)
        length = length - 1

let length = 2
count_down(length)
print(length)
";
    assert_eq!(printed(source), ["2", "1", "2"]);
}

#[test]
fn a_function_and_the_main_program_have_distinct_variables() {
    // The function is defined under the variables of the main program, which it doesn't
    // see: its own start again from its parameters.
    let source = "\
let sides = 5
let turns = 1

function paint(length):
    let sides = length + 1
    sides = sides + 1
    print(sides, length)

paint(turns)
print(sides, turns)
";
    assert_eq!(printed(source), ["3 1", "5 1"]);
}

#[test]
fn the_variables_of_a_call_end_with_it() {
    let source = "\
function count(start):
    let turns = start
    turns = turns + 1
    print(turns)

count(1)
count(7)
count(1)
";
    assert_eq!(printed(source), ["2", "8", "2"]);
}

// `return`.

#[test]
fn a_return_gives_back_a_value_of_any_type() {
    let source = "\
function double(n):
    return n * 2

function is_big(n):
    return n > 9

function name():
    return \"side\"

function paint():
    return blue

print(double(4), is_big(4), name() + \"s\", paint() == blue)
";
    assert_eq!(printed(source), ["8 false sides true"]);
}

#[test]
fn a_return_leaves_the_function_at_once() {
    let source = "\
function check(n):
    if n > 2:
        print(\"big\")
        return
    print(\"small\")

check(3)
check(1)
";
    let run = run(source);
    assert_eq!(run.lines, [7, 2, 3, 4, 7, 8, 2, 5, 8]);
    assert_eq!(run.printed, ["big", "small"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn a_return_in_loops_leaves_them_all() {
    let source = "\
function find(limit):
    repeat 3:
        let turn = 0
        while true:
            repeat limit:
                turn = turn + 1
                if turn == 2:
                    return turn * 10
    return 0

repeat 2:
    print(find(5))
print(\"after\")
";
    let run = run(source);
    // The turns left of the loops of `find` end with it, those of the `repeat` of the main
    // program stay: it runs its two turns.
    assert_eq!(run.printed, ["20", "20", "after"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn the_value_of_a_call_alone_on_its_line_is_dropped() {
    let source = "\
function double(n):
    return n * 2

function count(n):
    n = n + 1

double(3)
print(double(1) + 1)
double(4)
let steps = count(1)
";
    let run = run(source);
    assert_eq!(run.printed, ["3"]);
    // The value `double(4)` gave back isn't the one `count(1)` lacks.
    let error = RuntimeError {
        kind: no_value("count"),
        span: span(10, 13, 21),
    };
    assert_eq!(run.state, State::Failed(error));
}

#[test]
fn a_value_dropped_is_not_the_result_of_a_built_in_function() {
    let source = "function double(n):\n    return n * 2\n\ndouble(4)\nlet line = print(\"a\")\n";
    let error = RuntimeError {
        kind: no_value("print"),
        span: span(5, 12, 22),
    };
    assert_eq!(run(source).state, State::Failed(error));
}

#[test]
fn the_result_of_a_function_that_gives_back_no_value_stops_the_program() {
    let source = "\
function walk():
    print(\"a\")

let steps = walk()
print(\"after\")
";
    let run = run(source);
    // The function runs to its end, the line of its call is highlighted again, and the rest
    // of that line stops the program.
    assert_eq!(run.lines, [4, 2, 4]);
    assert_eq!(run.printed, ["a"]);
    let error = RuntimeError {
        kind: no_value("walk"),
        span: span(4, 13, 19),
    };
    assert_eq!(run.state, State::Failed(error));
    assert_eq!(calls(&run.machine), ["main program · Ln 4:"]);
}

#[test]
fn a_return_without_value_gives_back_no_value() {
    let stop = "function stop():\n    return\n\n";
    assert_eq!(
        failure(&format!("{stop}print(stop())\n")).kind,
        no_value("stop")
    );
    assert_eq!(
        failure(&format!("{stop}if stop():\n    print(1)\n")).kind,
        no_value("stop")
    );
    // A function gives back a value on some of its ways only.
    let source = "\
function half(n):
    if n % 2 == 0:
        return n / 2

print(half(4))
print(half(3))
";
    let run = run(source);
    assert_eq!(run.printed, ["2"]);
    let error = RuntimeError {
        kind: no_value("half"),
        span: span(6, 7, 14),
    };
    assert_eq!(run.state, State::Failed(error));
}

// Recursion, and the limit of the calls in progress.

#[test]
fn a_function_calls_itself_and_each_return_is_a_step() {
    let source = "\
function factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print(factorial(3))
print(factorial(10))
";
    let run = run(source);
    let first: [u32; 10] = [6, 2, 4, 2, 4, 2, 3, 4, 4, 6];
    assert_eq!(run.lines[..first.len()], first);
    // The innermost call comes back first, from its `return 1`.
    assert_eq!(run.returned_from[..3], [3, 4, 4]);
    assert_eq!(run.printed, ["6", "3628800"]);
    assert_eq!(run.state, State::Ended);
}

#[test]
fn each_call_of_a_recursion_has_its_own_variables() {
    let source = "\
function count(n):
    let double = n * 2
    if n > 0:
        count(n - 1)
    print(n, double)

count(2)
";
    assert_eq!(printed(source), ["0 0", "1 2", "2 4"]);
}

/// A program whose function calls itself until `depth` calls are in progress.
fn dive(depth: usize) -> String {
    format!(
        "\
function dive(depth):
    if depth > 1:
        dive(depth - 1)

dive({depth})
print(\"back\")
"
    )
}

#[test]
fn calls_nest_up_to_the_limit() {
    let run = run(&dive(MAX_CALL_DEPTH));
    assert_eq!(run.printed, ["back"]);
    assert_eq!(run.returned_from.len(), MAX_CALL_DEPTH);
    assert_eq!(run.state, State::Ended);
    assert_eq!(calls(&run.machine), ["main program · Ln 6:"]);
}

#[test]
fn one_call_more_than_the_limit_stops_the_program() {
    let run = run(&dive(MAX_CALL_DEPTH + 1));
    assert_eq!(run.printed, [""; 0]);
    let function = "dive".to_owned();
    let error = RuntimeError {
        kind: E::TooManyCalls { function },
        span: span(3, 9, 24),
    };
    assert_eq!(run.state, State::Failed(error));
    // The calls in progress stay, the last one on the line of the call that is refused.
    let calls = calls(&run.machine);
    assert_eq!(calls.len(), MAX_CALL_DEPTH + 1);
    assert_eq!(calls[0], "main program · Ln 5:");
    assert_eq!(calls[1], "dive · Ln 3: depth=201");
    assert_eq!(calls[MAX_CALL_DEPTH], "dive · Ln 3: depth=2");
}

#[test]
fn a_recursion_that_never_ends_stops_at_the_limit() {
    let run = run("function fall():\n    fall()\n\nfall()\n");
    let mut lines = vec![4];
    lines.resize(1 + MAX_CALL_DEPTH, 2);
    assert_eq!(run.lines, lines);
    let function = "fall".to_owned();
    let error = RuntimeError {
        kind: E::TooManyCalls { function },
        span: span(2, 5, 11),
    };
    assert_eq!(run.state, State::Failed(error));
}

// The calls in progress and their variables.

#[test]
fn the_calls_in_progress_go_from_the_main_program_to_the_function_being_run() {
    let source = "\
function paint_side(length, color):
    let done = 0
    print(length, color)

function paint_square(side):
    let sides = 4
    paint_side(side, red)

let turns = 1
paint_square(3)
print(turns)
";
    let mut machine = paused_on(source, 9);
    assert_eq!(calls(&machine), ["main program · Ln 9:"]);
    machine.step();
    assert_eq!(calls(&machine), ["main program · Ln 10: *turns=1"]);
    let mut machine = paused_on(source, 3);
    assert_eq!(
        calls(&machine),
        [
            "main program · Ln 10: turns=1",
            "paint_square · Ln 7: side=3 sides=4",
            "paint_side · Ln 3: length=3 color=red *done=0"
        ]
    );
    // The end of `paint_side`, then the one of `paint_square`.
    machine.step();
    assert_eq!(
        calls(&machine),
        [
            "main program · Ln 10: turns=1",
            "paint_square · Ln 7: side=3 sides=4"
        ]
    );
    machine.step();
    assert_eq!(calls(&machine), ["main program · Ln 10: turns=1"]);
    machine.step();
    machine.step();
    assert_eq!(*machine.state(), State::Ended);
    assert_eq!(calls(&machine), ["main program · Ln 11: turns=1"]);
}

#[test]
fn the_variables_of_a_call_in_progress_are_the_ones_alive() {
    let source = "\
let sides = 0
repeat 2:
    let side = sides + 1
    if side > 0:
        let double = side * 2
        print(double)
    sides = side
print(sides)
";
    let machine = paused_on(source, 6);
    assert_eq!(
        calls(&machine),
        ["main program · Ln 6: sides=0 side=1 *double=2"]
    );
    let machine = paused_on(source, 7);
    assert_eq!(calls(&machine), ["main program · Ln 7: sides=0 side=1"]);
    let machine = paused_on(source, 8);
    assert_eq!(calls(&machine), ["main program · Ln 8: sides=2"]);
}

#[test]
fn the_variables_the_last_step_wrote_are_changed() {
    let source = "\
function double(n):
    return n * 2

let sides = 1
let turns = 0
sides = sides + 1
turns = turns
sides = double(sides)
print(sides)
";
    let mut machine = paused_on(source, 4);
    let mut seen = Vec::new();
    while matches!(machine.state(), State::Running { .. }) {
        machine.step();
        seen.push(calls(&machine).join(" | "));
    }
    assert_eq!(
        seen,
        [
            "main program · Ln 5: *sides=1",
            "main program · Ln 6: sides=1 *turns=0",
            "main program · Ln 7: *sides=2 turns=0",
            // Written with the value it had: changed all the same.
            "main program · Ln 8: sides=2 *turns=0",
            // A parameter is written when its call is entered.
            "main program · Ln 8: sides=2 turns=0 | double · Ln 2: *n=2",
            "main program · Ln 8: sides=2 turns=0",
            "main program · Ln 9: *sides=4 turns=0",
            "main program · Ln 9: sides=4 turns=0",
        ]
    );
}

#[test]
fn the_calls_in_progress_stay_as_they_were_at_an_error() {
    let source = "\
function share(total, parts):
    let rest = total % 3
    return total / parts

let sides = 4
print(share(sides, 0))
";
    let run = run(source);
    let error = RuntimeError {
        kind: E::DivisionByZero,
        span: span(3, 12, 25),
    };
    assert_eq!(run.state, State::Failed(error));
    assert_eq!(
        calls(&run.machine),
        [
            "main program · Ln 6: sides=4",
            "share · Ln 3: total=4 parts=0 rest=1"
        ]
    );
}

// The depth of a program.

/// Runs `source`, the deepest program of its kind: `deeper`, one level more, is refused by
/// the parser for its depth.
#[track_caller]
fn printed_at_the_limit(source: &str, deeper: &str) -> Vec<String> {
    let too_deep = matches!(
        parse(deeper),
        Err(SyntaxError::Parse(error)) if error.kind == ParseErrorKind::TooDeep
    );
    assert!(too_deep, "one level more isn't refused for its depth");
    printed(source)
}

/// `depth` blocks, one inside the other, around `line`.
fn nested_blocks(depth: usize, line: &str) -> String {
    let mut source = String::new();
    for level in 0..depth {
        source.push_str(&"    ".repeat(level));
        source.push_str(if level % 2 == 0 {
            "if true:\n"
        } else {
            "repeat 1:\n"
        });
    }
    source.push_str(&"    ".repeat(depth));
    source.push_str(line);
    source.push('\n');
    source
}

#[test]
fn the_deepest_expressions_run() {
    // In `print(…)`, the call and the value each take one level of the tree.
    let depth = usize::try_from(MAX_DEPTH).unwrap() - 2;
    let nested = |depth: usize| format!("print({}1{})", "(".repeat(depth), ")".repeat(depth));
    assert_eq!(
        printed_at_the_limit(&nested(depth), &nested(depth + 1)),
        ["1"]
    );
    let negated = |depth: usize| format!("print({}true)", "not ".repeat(depth));
    assert_eq!(
        printed_at_the_limit(&negated(depth), &negated(depth + 1)),
        ["true"]
    );
    // A chain of operators is as deep as it is long.
    let sum = |terms: usize| format!("print({})", vec!["1"; terms].join(" + "));
    assert_eq!(
        printed_at_the_limit(&sum(depth + 1), &sum(depth + 2)),
        [(depth + 1).to_string()]
    );
}

#[test]
fn the_deepest_blocks_run() {
    let blocks = usize::try_from(MAX_BLOCK_DEPTH).unwrap();
    let deep = |depth: usize| nested_blocks(depth, "print(\"deep\")");
    assert_eq!(
        printed_at_the_limit(&deep(blocks), &deep(blocks + 1)),
        ["deep"]
    );
    // The worst case: the deepest expression the parser takes inside the deepest blocks.
    let depth = usize::try_from(MAX_DEPTH).unwrap() - blocks - 2;
    let nested = |depth: usize| {
        let line = format!("print({}1{})", "(".repeat(depth), ")".repeat(depth));
        nested_blocks(blocks, &line)
    };
    assert_eq!(
        printed_at_the_limit(&nested(depth), &nested(depth + 1)),
        ["1"]
    );
}

// A program the checks before running refuse.

#[test]
fn a_program_that_is_not_checked_does_not_run() {
    for (source, place) in [
        ("print(1)\nsides = 3\n", span(2, 1, 6)),
        ("print(1)\nprint(sidse)\n", span(2, 7, 12)),
        ("print(1)\nbreak\n", span(2, 1, 6)),
        ("print(1)\nreturn 3\n", span(2, 1, 9)),
        (
            "function walk():\n    print(1)\n\nreturn 3\n",
            span(4, 1, 9),
        ),
        ("print(1)\nif true:\n    print(walk())\n", span(3, 11, 15)),
        // A function defined in a block, or in another function.
        (
            "print(1)\nif true:\n    function walk():\n        print(2)\n",
            span(3, 5, 21),
        ),
        (
            "function walk():\n    function turn():\n        print(1)\n",
            span(2, 5, 21),
        ),
        // A call above the definition, with a wrong number of values, of a second function
        // of the same name.
        ("walk()\nfunction walk():\n    print(1)\n", span(1, 1, 5)),
        ("function walk():\n    print(1)\n\nwalk(1)\n", span(4, 1, 5)),
        (
            "function walk():\n    print(1)\n\nfunction walk():\n    print(2)\n",
            span(4, 10, 14),
        ),
        // A variable of the main program used in a function.
        (
            "let sides = 1\nfunction walk():\n    print(sides)\n",
            span(3, 11, 16),
        ),
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

// Random programs.

/// The lines of the main program of a random program. An `else` only follows the block of
/// its `if`: they are drawn together, or almost no program would have one.
const MAIN_PIECES: [&str; 27] = [
    "let side = sides + 1",
    "let turns = 2",
    "sides = sides + 1",
    "sides = sides - 2",
    "turns = turns - 1",
    "print(sides)",
    "print(side, turns)",
    "print(10 / (sides - 1))",
    "if sides < 2:",
    "if sides > 0 and sides < 3:",
    "if side == 1 and turns > 0:",
    "if sides < 1:\n    sides = sides + 1\nelse if sides == 2:",
    "if sides == 1:\n    print(sides)\nelse if sides < 0 or sides > 4:\n    break\nelse:",
    "if sides < 2:\n    let side = 5\n    print(side)\nelse:",
    "while sides < 3:",
    "while sides < 2 or sides == 5:",
    "while turns > 0 or side < sides:",
    "repeat 2:",
    "repeat sides:",
    "repeat 3 - sides:",
    "break",
    "walk(sides)",
    "sides = walk(sides)",
    "print(walk(2) + walk(sides))",
    "if walk(sides) > 1:",
    "while walk(sides) < 2:",
    "repeat walk(1):",
];

/// The lines of the function `walk(n)` of a random program, whose block is drawn too: a
/// `return` goes back to a place only known while the program runs, from anywhere in the
/// loops of the function.
const FUNCTION_PIECES: [&str; 19] = [
    "let turn = n + 1",
    "n = n - 1",
    "turn = turn * 2",
    "print(n)",
    "return",
    "return n",
    "return n + 1",
    "return walk(n - 1) + 1",
    "n = walk(n - 2)",
    "walk(n - 1)",
    "if n > 2:",
    "if n < 1:\n    return 0\nelse:",
    "if n > 1 and walk(n - 1) > 0:",
    "while n > 0:",
    "repeat 2:",
    "repeat n:",
    "repeat 3:\n    n = n - 1\n    if n < 2:\n        return n",
    "while n > 0:\n    repeat n:\n        return walk(n - 1) + 1\n    n = n - 1",
    "break",
];

/// A random program: the function `walk(n)`, then a main program that may call it.
fn random_program(random: &mut Random) -> String {
    // Three times out of four, the function stops its recursion and ends on a value:
    // otherwise most programs would stop at their first call.
    let (stop, value) = if random.next().is_multiple_of(4) {
        ("", "")
    } else {
        ("if n < 0:\n        return 0\n    ", "return n\n")
    };
    let function = random.program(&FUNCTION_PIECES).replace('\n', "\n    ");
    let main_program = random.program(&MAIN_PIECES);
    format!("function walk(n):\n    {stop}{function}{value}\nlet sides = 0\n{main_program}")
}

/// Whether the line `line` of `source` is a `return` in the block of a `repeat`: the turns
/// left of that loop end with the function.
fn returns_from_a_repeat(source: &str, line: u32) -> bool {
    // The indentation of each `repeat` around the line reached.
    let mut repeats = Vec::new();
    for (number, text) in (1..).zip(source.lines()) {
        let statement = text.trim_start();
        if statement.is_empty() {
            continue;
        }
        let indentation = text.len() - statement.len();
        repeats.retain(|&around| around < indentation);
        if number == line {
            return statement.starts_with("return") && !repeats.is_empty();
        }
        if statement.starts_with("repeat") {
            repeats.push(indentation);
        }
    }
    false
}

#[test]
fn random_checked_programs_never_stop_on_a_mistake_of_the_core() {
    let mut random = Random(9);
    let (mut ended, mut failed, mut running) = (0, 0, 0);
    let (mut with_else, mut with_logic, mut with_break) = (0, 0, 0);
    let (mut with_return, mut with_repeat_left) = (0, 0);
    for _ in 0..20_000 {
        let source = random_program(&mut random);
        let Ok(program) = parse(&source) else {
            continue;
        };
        if !check(&program).is_empty() {
            continue;
        }
        with_else += usize::from(source.contains("else"));
        with_logic += usize::from(source.contains(" and ") || source.contains(" or "));
        with_break += usize::from(source.contains("break"));
        let run = run_program(&program);
        with_return += usize::from(!run.returned_from.is_empty());
        let mut returned_from = run.returned_from.iter();
        with_repeat_left +=
            usize::from(returned_from.any(|&line| returns_from_a_repeat(&source, line)));
        match run.state {
            State::Ended => ended += 1,
            State::Running { .. } => running += 1,
            State::Failed(error) => {
                assert_ne!(error.kind, E::Internal, "in {source:?}");
                failed += 1;
            }
        }
    }
    // Enough programs of each kind: the ones that end, that stop on an error of their own
    // (a count below zero, a division by zero, a value missing, too many calls), and that
    // never end.
    assert!(ended > 500, "only {ended} programs ended");
    assert!(failed > 80, "only {failed} programs stopped on an error");
    assert!(running > 80, "only {running} programs kept running");
    // And enough of each shape whose jumps the compiler may get wrong.
    assert!(with_else > 200, "only {with_else} programs had an else");
    assert!(
        with_logic > 200,
        "only {with_logic} programs had an and or an or"
    );
    assert!(with_break > 100, "only {with_break} programs had a break");
    assert!(
        with_return > 300,
        "only {with_return} programs came back from a function"
    );
    assert!(
        with_repeat_left > 100,
        "only {with_repeat_left} programs came back from a function with turns of a repeat left"
    );
}

// Until the next sub-step.

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
