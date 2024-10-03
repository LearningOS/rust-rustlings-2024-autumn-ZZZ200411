// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl From<CreationError> for ParsePosNonzeroError {
    fn from(e: CreationError) -> Self {
        ParsePosNonzeroError::Creation(e)
    }
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    // 新增 new 方法，用于构造 PositiveNonzeroInteger。
    fn new(value: i64) -> Result<Self, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::ParseInt)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }
   
    #[test]
    fn test_negative() {
        let result = parse_pos_nonzero("-555");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParsePosNonzeroError::Creation(CreationError::Negative)
        );
    }

    #[test]
    fn test_zero() {
        let result = parse_pos_nonzero("0");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParsePosNonzeroError::Creation(CreationError::Zero)
        );
    }

    #[test]
    fn test_positive() {
        let result = parse_pos_nonzero("42");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PositiveNonzeroInteger(42));
    }
}