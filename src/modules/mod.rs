pub mod evaluating;
pub mod parsing;

use either::Either;

/// A single string multiplication command.
#[derive(Debug)]
pub struct StringMultCommand {
    /// The string to operate on.
    pub params: Vec<StrPiece>,
    /// The operations to perform.
    pub operations: Vec<StringMultOperation>,
}

/// An operation to perform on a string.
#[derive(Debug, PartialEq)]
pub struct StringMultOperation {
    /// The type of operation to perform.
    pub operation_type: OperationType,
    /// The argument to the operation.
    pub argument: Either<isize, f64>,
}

#[derive(Debug, PartialEq)]
/// A piece of a string that is being operated on.
pub enum StrPiece {
    /// A number.
    Num(f64),
    /// Not a number.
    Str(String),
}

/// The type of operation to perform.
#[derive(Debug, PartialEq)]
pub enum OperationType {
    /// Multiply the number at the given index by the argument.
    Mult(Option<isize>),
    /// Multiply all numbers by the argument.
    MultAll,
    /// Duplicate the string times the argument.
    Duplicate,
}
