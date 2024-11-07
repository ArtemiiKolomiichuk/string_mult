use pest::Parser;
use thiserror::Error;

use crate::{Rule, StringMultGrammar};

use super::*;

#[derive(Error, Debug)]
///An error that can occur during evaluation.
pub enum EvalError {
    #[error("commands list not found")]
    NoCommandsList,
    #[error("no command found")]
    NoCommand,
    #[error("unexpected rule {0}")]
    UnexpectedRule(String),
    #[error("index '{0}' out of range '0..{1}'")]
    IndexOutOfRange(usize, usize),
    #[error("unexpected evaluation error")]
    Unknown,
}

///Evaluates a list of commands
pub fn evaluate_list(input: &str) -> anyhow::Result<Vec<Result<String, anyhow::Error>>> {
    let mut results = Vec::new();
    let data = StringMultGrammar::parse(Rule::commands_list, input);
    if data.is_err() {
        return Err(anyhow::anyhow!(EvalError::NoCommandsList));
    }
    let inner = data?.next().ok_or(EvalError::NoCommandsList)?.into_inner();
    for part in inner {
        results.push(evaluate(part.as_str()));
    }
    Ok(results)
}

///Evaluates a single string multiplication command, returning a new String without quote marks.
pub fn evaluate(input: &str) -> anyhow::Result<String> {
    let data = StringMultGrammar::parse(Rule::command, input);
    if data.is_err() {
        return Err(anyhow::anyhow!(EvalError::NoCommand));
    }
    let inner = data?.next().ok_or(EvalError::NoCommand)?.into_inner();

    let mut accum;

    let mut parts: Vec<StrPiece> = Vec::new();
    let mut operation = OperationType::Mult(None);
    for part in inner {
        match part.as_rule() {
            Rule::str_param => {
                parts = Vec::new();
                for inner_part in part.into_inner() {
                    match inner_part.as_rule() {
                        Rule::num => parts.push(StrPiece::Num(inner_part.as_str().parse::<f64>()?)),
                        Rule::inner_str_text => parts.push(StrPiece::Str(inner_part.as_str())),
                        r => {
                            return Err(anyhow::anyhow!(EvalError::UnexpectedRule(format!(
                                "{:?}",
                                r
                            ))))
                        }
                    }
                }
            }

            Rule::mult => {
                let mut inner_parts = part.into_inner();
                let index = match inner_parts.next() {
                    Some(inner_part) => inner_part.as_str().parse::<isize>()?,
                    None => 0,
                };
                operation = OperationType::Mult(Some(index));
            }
            Rule::multAll => operation = OperationType::MultAll,
            Rule::duplicate => operation = OperationType::Duplicate,

            Rule::int => {
                let int = part.as_str().parse::<isize>()?;
                match operation {
                    OperationType::Duplicate => {
                        if int == 0 {
                            return Ok("".to_string());
                        }

                        if int < 0 {
                            let str = to_string(parts).chars().rev().collect::<String>();
                            accum = String::new();
                            accum.push('\"');
                            for _ in 0..(-int) {
                                accum.push_str(&str);
                            }
                            accum.push('\"');

                            let data = StringMultGrammar::parse(Rule::str_param, accum.as_str())?
                                .next()
                                .ok_or(EvalError::Unknown)?;
                            parts = Vec::new();
                            for inner_part in data.into_inner() {
                                match inner_part.as_rule() {
                                    Rule::num => parts
                                        .push(StrPiece::Num(inner_part.as_str().parse::<f64>()?)),
                                    Rule::inner_str_text => {
                                        parts.push(StrPiece::Str(inner_part.as_str()))
                                    }
                                    r => {
                                        return Err(anyhow::anyhow!(EvalError::UnexpectedRule(
                                            format!("{:?}", r)
                                        )))
                                    }
                                }
                            }
                            continue;
                        }

                        let mut new_parts = Vec::new();
                        for _ in 0..(int - 1) {
                            for part in &parts {
                                match part {
                                    StrPiece::Num(n) => new_parts.push(StrPiece::Num(*n)),
                                    StrPiece::Str(text) => new_parts.push(StrPiece::Str(text)),
                                }
                            }
                        }
                        parts.extend(new_parts);
                    }
                    _ => continue,
                };
            }
            Rule::num => {
                let num = part.as_str().parse::<f64>()?;
                match operation {
                    OperationType::Mult(ref mut index) => {
                        let index = match index {
                            Some(index) => {
                                if *index < 0 {
                                    (parts
                                        .iter()
                                        .filter(|p| matches!(p, StrPiece::Num(_)))
                                        .count() as isize
                                        + *index) as usize
                                } else {
                                    *index as usize
                                }
                            }
                            None => 0,
                        };
                        let mut i = 0;
                        for part in parts.iter_mut() {
                            match part {
                                StrPiece::Num(n) => {
                                    if i == index {
                                        *part = StrPiece::Num(num * *n);
                                        i = usize::MAX;
                                        break;
                                    }
                                    i += 1;
                                }
                                _ => continue,
                            }
                        }
                        if i != usize::MAX {
                            return Err(anyhow::anyhow!(EvalError::IndexOutOfRange(
                                index,
                                parts
                                    .iter()
                                    .filter(|p| matches!(p, StrPiece::Num(_)))
                                    .count()
                            )));
                        }
                    }

                    OperationType::MultAll => {
                        for part in &mut parts {
                            match part {
                                StrPiece::Num(n) => *n *= num,
                                _ => continue,
                            }
                        }
                    }
                    _ => continue,
                };
            }

            r => {
                return Err(anyhow::anyhow!(EvalError::UnexpectedRule(format!(
                    "{:?}",
                    r
                ))))
            }
        }
    }
    Ok(to_string(parts))
}

fn to_string(parts: Vec<StrPiece>) -> String {
    parts
        .iter()
        .map(|p| match p {
            StrPiece::Num(n) => {
                let rounded = format!("{:.8}", n);
                let trimmed = rounded.trim_end_matches('0').trim_end_matches('.');
                trimmed.to_string()
            }
            StrPiece::Str(text) => text.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}
