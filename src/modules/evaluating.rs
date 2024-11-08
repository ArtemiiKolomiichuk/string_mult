//! Provides evaluating functionality for `StringMultCommand`

use parsing::{parse_command, ParseError};
use pest::Parser;
use thiserror::Error;
use Either::{Left, Right};

use crate::{Rule, StringMultGrammar};

use super::*;

#[derive(Error, Debug)]
///An error that can occur during evaluation.
pub enum EvalError {
    #[error("commands list not found")]
    /// No commands list was found
    NoCommandsList,
    #[error("no command found")]
    /// No command was found
    NoCommand,
    #[error("parsing error")]
    /// Parsing error occured 
    ParseError(#[from] ParseError),

    #[error("index '{0}' out of range '0..{1}'")]
    /// Index for multiplication was out of range
    IndexOutOfRange(usize, usize),
    #[error("duplicating by float is undefined")]
    /// Float argument was provided for multiplication
    DuplicatingByFloat,
    #[error("unexpected evaluation error")]
    /// Unknown unexpected error
    Unknown,
}

///Evaluates a list of commands
pub fn evaluate_list(input: &str) -> Result<Vec<Result<String, EvalError>>, EvalError> {
    let mut results = Vec::new();
    let data = StringMultGrammar::parse(Rule::commands_list, input);
    if data.is_err() {
        return Err(EvalError::NoCommandsList);
    }
    let inner = data
        .unwrap()
        .next()
        .ok_or(EvalError::NoCommandsList)?
        .into_inner();
    for part in inner {
        results.push(evaluate(part.as_str()));
    }
    Ok(results)
}

/// Evaluates a single string multiplication command, returning a new String without quote marks.
pub fn evaluate(input: &str) -> Result<String, EvalError> {
    let comm = parse_command(input);
    match comm {
        Ok(c) => evaluate_command(c),
        Err(e) => Err(EvalError::ParseError(e)),
    }
}

/// Evaluates a single `StringMultCommand`, returning a new String without quote marks.
pub fn evaluate_command(input: StringMultCommand) -> Result<String, EvalError> {
    let mut command = input.clone();

    for operation in command.operations {
        match operation.operation_type {
            OperationType::Mult(index) => {
                let index = match index {
                    Some(index) => {
                        if index < 0 {
                            (command
                                .params
                                .iter()
                                .filter(|p| matches!(p, ParamsPiece::Num(_)))
                                .count() as isize
                                + index) as usize
                        } else {
                            index as usize
                        }
                    }
                    None => 0,
                };
                let mut i = 0;
                let argument = match operation.argument {
                    Left(arg) => arg as f64,
                    Right(arg) => arg,
                };
                for part in command.params.iter_mut() {
                    match part {
                        ParamsPiece::Num(n) => {
                            if i == index {
                                *part = ParamsPiece::Num(*n * argument);
                                i = usize::MAX;
                                break;
                            }
                            i += 1;
                        }
                        _ => continue,
                    }
                }
                if i != usize::MAX {
                    return Err(EvalError::IndexOutOfRange(
                        index,
                        command
                            .params
                            .iter()
                            .filter(|p| matches!(p, ParamsPiece::Num(_)))
                            .count(),
                    ));
                }
            }
            OperationType::MultAll => {
                let argument = match operation.argument {
                    Left(arg) => arg as f64,
                    Right(arg) => arg,
                };
                for part in &mut command.params {
                    match part {
                        ParamsPiece::Num(n) => *n *= argument,
                        _ => continue,
                    }
                }
            }
            OperationType::Duplicate => {
                let mut argument = match operation.argument {
                    Left(arg) => arg,
                    Right(_) => return Err(EvalError::DuplicatingByFloat),
                };
                if argument == 0 {
                    return Ok("".to_string());
                }
                if argument < 0 {
                    command.params = match rev_params(command.params) {
                        Ok(p) => p,
                        Err(e) => return Err(EvalError::ParseError(e)),
                    };
                    argument = -argument;
                }
                let mut new_parts = Vec::new();
                for _ in 0..(argument - 1) {
                    for param in &command.params {
                        match param {
                            ParamsPiece::Num(n) => new_parts.push(ParamsPiece::Num(*n)),
                            ParamsPiece::Str(text) => new_parts.push(ParamsPiece::Str(text.to_string())),
                        }
                    }
                }
                command.params.extend(new_parts);
            }
        };
    }
    Ok(to_string(command.params))
}

pub(crate) fn to_string(parts: Vec<ParamsPiece>) -> String {
    parts
        .iter()
        .map(|p| match p {
            ParamsPiece::Num(n) => {
                let rounded = format!("{:.8}", n);
                let trimmed = rounded.trim_end_matches('0').trim_end_matches('.');
                trimmed.to_string()
            }
            ParamsPiece::Str(text) => text.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}
