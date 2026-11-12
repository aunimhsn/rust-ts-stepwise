//! What the operators of the language compute (see *Expressions* in `specs.md`), and the
//! values they refuse. Nothing here knows the machine: an operator takes values and gives one.

use std::rc::Rc;

use super::error::RuntimeErrorKind;
use super::value::{MAX_TEXT_LENGTH, Value};
use crate::ast::{BinaryOperator, UnaryOperator};

/// The result of `operator` in front of `operand`.
pub(super) fn unary(operator: UnaryOperator, operand: &Value) -> Result<Value, RuntimeErrorKind> {
    match (operator, operand) {
        (UnaryOperator::Negate, Value::Number(number)) => number
            .checked_neg()
            .map(Value::Number)
            .ok_or(RuntimeErrorKind::Overflow),
        (UnaryOperator::Not, Value::Boolean(boolean)) => Ok(Value::Boolean(!boolean)),
        _ => Err(RuntimeErrorKind::UnaryOperand {
            operator,
            found: operand.value_type(),
        }),
    }
}

/// The result of `operator` between `left` and `right`, both already evaluated. `and` and
/// `or` aren't computed here: the machine runs them itself, since their right side is only
/// evaluated when the left one doesn't decide.
pub(super) fn binary(
    operator: BinaryOperator,
    left: &Value,
    right: &Value,
) -> Result<Value, RuntimeErrorKind> {
    use BinaryOperator as Operator;
    let numbers = match (left, right) {
        (&Value::Number(left), &Value::Number(right)) => Some((left, right)),
        _ => None,
    };
    let same_type = left.value_type() == right.value_type();
    let number = |result: Option<i64>| result.map(Value::Number).ok_or(RuntimeErrorKind::Overflow);
    // `None` when the operator doesn't take values of these types.
    let result = match operator {
        Operator::Add => match (left, right) {
            (Value::Text(left), Value::Text(right)) => Some(join(left, right)),
            _ => numbers.map(|(left, right)| number(left.checked_add(right))),
        },
        Operator::Subtract => numbers.map(|(left, right)| number(left.checked_sub(right))),
        Operator::Multiply => numbers.map(|(left, right)| number(left.checked_mul(right))),
        Operator::Divide => numbers.map(|(left, right)| divide(left, right).map(Value::Number)),
        Operator::Remainder => {
            numbers.map(|(left, right)| remainder(left, right).map(Value::Number))
        }
        Operator::Equal => same_type.then(|| Ok(Value::Boolean(left == right))),
        Operator::NotEqual => same_type.then(|| Ok(Value::Boolean(left != right))),
        Operator::Less => numbers.map(|(left, right)| Ok(Value::Boolean(left < right))),
        Operator::LessEqual => numbers.map(|(left, right)| Ok(Value::Boolean(left <= right))),
        Operator::Greater => numbers.map(|(left, right)| Ok(Value::Boolean(left > right))),
        Operator::GreaterEqual => numbers.map(|(left, right)| Ok(Value::Boolean(left >= right))),
        Operator::And | Operator::Or => Some(Err(RuntimeErrorKind::Internal)),
    };
    result.unwrap_or_else(|| {
        Err(RuntimeErrorKind::BinaryOperands {
            operator,
            left: left.value_type(),
            right: right.value_type(),
        })
    })
}

/// The whole division, rounded down as in Python: `-7 / 2` is `-4`.
fn divide(left: i64, right: i64) -> Result<i64, RuntimeErrorKind> {
    if right == 0 {
        return Err(RuntimeErrorKind::DivisionByZero);
    }
    // Only the smallest number divided by -1 goes beyond the range.
    let quotient = left.checked_div(right).ok_or(RuntimeErrorKind::Overflow)?;
    // Rust rounds toward zero: a result below zero that isn't whole is one too high.
    let whole = left.checked_rem(right) == Some(0);
    if !whole && (left < 0) != (right < 0) {
        quotient.checked_sub(1).ok_or(RuntimeErrorKind::Overflow)
    } else {
        Ok(quotient)
    }
}

/// The remainder of the whole division, which follows its rounding: it has the sign of
/// `right`, as in Python, so `-7 % 2` is `1`.
fn remainder(left: i64, right: i64) -> Result<i64, RuntimeErrorKind> {
    if right == 0 {
        return Err(RuntimeErrorKind::DivisionByZero);
    }
    // `checked_rem` refuses the smallest number by -1, whose remainder is 0, as for any
    // number by -1.
    let remainder = left.checked_rem(right).unwrap_or(0);
    // Rust gives the remainder the sign of `left`.
    if remainder != 0 && (remainder < 0) != (right < 0) {
        remainder
            .checked_add(right)
            .ok_or(RuntimeErrorKind::Overflow)
    } else {
        Ok(remainder)
    }
}

/// Two texts joined by `+`.
fn join(left: &str, right: &str) -> Result<Value, RuntimeErrorKind> {
    // A text has at most as many characters as bytes: most texts are accepted without
    // counting their characters.
    let bytes = left.len().saturating_add(right.len());
    if bytes > MAX_TEXT_LENGTH {
        let characters = left.chars().count().saturating_add(right.chars().count());
        if characters > MAX_TEXT_LENGTH {
            return Err(RuntimeErrorKind::TextTooLong);
        }
    }
    Ok(Value::Text(Rc::from([left, right].concat())))
}

#[cfg(test)]
mod tests {
    use super::{binary, divide, remainder};
    use crate::ast::BinaryOperator;
    use crate::interpreter::{MAX_TEXT_LENGTH, RuntimeErrorKind, Value};

    #[test]
    fn the_division_and_its_remainder_give_back_the_number_divided() {
        for left in -20..=20_i64 {
            for right in (-7..=7_i64).filter(|&right| right != 0) {
                let quotient = divide(left, right).unwrap();
                let remainder = remainder(left, right).unwrap();
                assert_eq!(quotient * right + remainder, left, "{left} by {right}");
                // The remainder is between zero and the divisor, on its side of zero.
                assert!(remainder.abs() < right.abs(), "{left} by {right}");
                assert!(
                    remainder == 0 || (remainder < 0) == (right < 0),
                    "{left} by {right}"
                );
            }
        }
    }

    #[test]
    fn the_division_at_the_ends_of_the_range() {
        assert_eq!(divide(i64::MIN, -1), Err(RuntimeErrorKind::Overflow));
        assert_eq!(remainder(i64::MIN, -1), Ok(0));
        assert_eq!(divide(i64::MIN, 1), Ok(i64::MIN));
        assert_eq!(divide(i64::MIN, 2), Ok(i64::MIN / 2));
        assert_eq!(divide(i64::MAX, -2), Ok(i64::MIN / 2));
        assert_eq!(remainder(i64::MAX, -2), Ok(-1));
        assert_eq!(remainder(i64::MIN, i64::MAX), Ok(i64::MAX - 1));
        assert_eq!(divide(i64::MIN, i64::MAX), Ok(-2));
    }

    #[test]
    fn the_length_of_a_text_counts_its_characters_not_its_bytes() {
        // Two bytes for each character.
        let text = |characters: usize| Value::Text("é".repeat(characters).into());
        let joined = binary(BinaryOperator::Add, &text(MAX_TEXT_LENGTH - 1), &text(1));
        assert_eq!(joined, Ok(text(MAX_TEXT_LENGTH)));
        let too_long = binary(BinaryOperator::Add, &text(MAX_TEXT_LENGTH), &text(1));
        assert_eq!(too_long, Err(RuntimeErrorKind::TextTooLong));
    }

    #[test]
    fn and_and_or_are_left_to_the_machine() {
        for operator in [BinaryOperator::And, BinaryOperator::Or] {
            let both = binary(operator, &Value::Boolean(true), &Value::Boolean(true));
            assert_eq!(both, Err(RuntimeErrorKind::Internal));
        }
    }
}
