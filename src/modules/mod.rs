pub mod evaluating;
pub mod parsing;

use either::Either;
use parsing::ParseError;

/// A single string multiplication command.
#[derive(Debug, Clone)]
pub struct StringMultCommand {
    /// The string to operate on.
    pub params: Vec<StrPiece>,
    /// The operations to perform.
    pub operations: Vec<StringMultOperation>,
}

/// Reverses the parameters.
pub(crate) fn rev_params(params: Vec<StrPiece>) -> Result<Vec<StrPiece>, ParseError> {
    let str = evaluating::to_string(params);
    let new_str = format!("\"{}\"", str.chars().rev().collect::<String>());
    parsing::parse_params(&new_str)
}

/// An operation to perform on a string.
#[derive(Debug, PartialEq, Clone)]
pub struct StringMultOperation {
    /// The type of operation to perform.
    pub operation_type: OperationType,
    /// The argument to the operation.
    pub argument: Either<isize, f64>,
}

#[derive(Debug, PartialEq, Clone)]
/// A piece of a string that is being operated on.
pub enum StrPiece {
    /// A number.
    Num(f64),
    /// Not a number.
    Str(String),
}

/// The type of operation to perform.
#[derive(Debug, PartialEq, Clone)]
pub enum OperationType {
    /// Multiply the number at the given index by the argument.
    Mult(Option<isize>),
    /// Multiply all numbers by the argument.
    MultAll,
    /// Duplicate the string times the argument.
    Duplicate,
}
