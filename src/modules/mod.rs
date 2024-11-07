pub mod evaluating;
pub mod parsing;

use either::Either;

pub struct StringMultCommand<'a> {
    pub params: Vec<StrPiece<'a>>,
    pub operations: Vec<StringMultOperation>,
}

pub struct StringMultOperation {
    pub operation_type: OperationType,
    pub argument: Either<isize, f64>,
}

#[derive(Debug)]
pub enum StrPiece<'a> {
    Num(f64),
    Str(&'a str),
}

pub enum OperationType {
    Mult(Option<isize>),
    MultAll,
    Duplicate,
}
