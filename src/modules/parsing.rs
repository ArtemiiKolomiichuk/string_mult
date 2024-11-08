use super::*;
use crate::{Rule, StringMultGrammar};
use pest::Parser;
use thiserror::Error;

use super::StringMultCommand;

#[derive(Error, Debug)]
/// An error that can occur during parsing.
pub enum ParseError {
    #[error("commands list not found")]
    NoCommandsList,
    #[error("no command found in '{0}'")]
    WrongCommand(String),
    #[error("unexpected rule {0}")]
    UnexpectedRule(String),
    #[error("operation argument precedes operation")]
    ArgumentWithoutOperation,
    #[error("failed to parse float: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("failed to parse int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("unexpected parsing error")]
    Unknown,
}


/// Parses a list of commands.
pub fn parse_list(input: &str) -> Result<Vec<Result<StringMultCommand, ParseError>>, ParseError> {
    let mut results = Vec::new();

    let data = StringMultGrammar::parse(Rule::commands_list, input);
    if data.is_err() {
        return Err(ParseError::NoCommandsList);
    }

    let inner = data.unwrap().next().ok_or(ParseError::NoCommandsList)?.into_inner();
    for part in inner {
        if part.as_rule() == Rule::wrong_command {
            results.push(Err(ParseError::WrongCommand(part.as_str().to_string())));
        }
        else{
            results.push(parse_command(part.as_str()));
        }
    }

    Ok(results)
}

/// Parses a string into a `StringMultCommand`.
pub fn parse_command(input: &str) -> Result<StringMultCommand, ParseError> {
    let data = StringMultGrammar::parse(Rule::command, input);
    if data.is_err() {
        return Err(ParseError::WrongCommand(input.to_string()));
    }
    let inner = data
        .unwrap()
        .next()
        .ok_or(ParseError::WrongCommand(input.to_string()))?
        .into_inner();

    let mut pieces: Vec<StrPiece> = Vec::new();

    let mut operations: Vec<StringMultOperation> = Vec::new();
    let mut operation: Option<OperationType> = None;

    for part in inner {
        match part.as_rule() {
            Rule::str_param => {
                pieces = Vec::new();
                for inner_part in part.into_inner() {
                    match inner_part.as_rule() {
                        Rule::num => {
                            pieces.push(StrPiece::Num(inner_part.as_str().parse::<f64>()?))
                        }
                        Rule::inner_str_text => pieces.push(StrPiece::Str(inner_part.as_str().to_string())),
                        r => return Err(ParseError::UnexpectedRule(format!("{:?}", r))),
                    }
                }
            }

            Rule::mult => {
                let mut inner_parts = part.into_inner();
                let index = match inner_parts.next() {
                    Some(inner_part) => inner_part.as_str().parse::<isize>()?,
                    None => 0,
                };
                operation = Some(OperationType::Mult(Some(index)));
            }
            Rule::multAll => operation = Some(OperationType::MultAll),
            Rule::duplicate => operation = Some(OperationType::Duplicate),

            Rule::int => {
                let int = part.as_str().parse::<isize>()?;
                if let Some(op) = operation {
                    operations.push(StringMultOperation {
                        operation_type: op,
                        argument: Either::Left(int),
                    });
                    operation = None;
                } else {
                    return Err(ParseError::ArgumentWithoutOperation);
                }
            }
            Rule::num => {
                let num = part.as_str().parse::<f64>()?;
                if let Some(op) = operation {
                    operations.push(StringMultOperation {
                        operation_type: op,
                        argument: Either::Right(num),
                    });
                    operation = None;
                } else {
                    return Err(ParseError::ArgumentWithoutOperation);
                }
            }

            r => return Err(ParseError::UnexpectedRule(format!("{:?}", r))),
        }
    }
    Ok(StringMultCommand {
        params: pieces,
        operations,
    })
}