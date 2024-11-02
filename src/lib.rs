use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./gramm.pest"]
pub struct Grammar;

pub fn evaluate(input: &str) -> anyhow::Result<String> {
    let data = Grammar::parse(Rule::command, input)?
        .next()
        .ok_or_else(|| anyhow::anyhow!("no command found"))?;
    let inner = data.into_inner();
    if inner.len() < 2 {
        println!("{:#?}", inner);
        return Err(anyhow::anyhow!("not enough logical parts"));
    }

    let mut accum;

    let mut parts: Vec<StrPiece> = Vec::new();
    let mut operation = Operation::Mult(None);
    for part in inner {
        match part.as_rule() {
            Rule::str_param => {
                parts = Vec::new();
                for inner_part in part.into_inner() {
                    match inner_part.as_rule() {
                        Rule::num => parts.push(StrPiece::Num(inner_part.as_str().parse::<f64>()?)),
                        Rule::inner_str_text => parts.push(StrPiece::Str(inner_part.as_str())),
                        r => return Err(anyhow::anyhow!("unexpected rule {:?}", r)),
                    }
                }
            }

            Rule::mult => {
                let mut inner_parts = part.into_inner();
                let index = match inner_parts.next() {
                    Some(inner_part) => inner_part.as_str().parse::<isize>()?,
                    None => 0,
                };
                operation = Operation::Mult(Some(index));
            }
            Rule::multAll => operation = Operation::MultAll,
            Rule::duplicate => operation = Operation::Duplicate,

            Rule::int => {
                let int = part.as_str().parse::<isize>()?;
                match operation {
                    Operation::Duplicate => {
                        let mut new_parts = Vec::new();
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
                            let data = Grammar::parse(Rule::str_param, accum.as_str())?
                                .next()
                                .ok_or_else(|| anyhow::anyhow!("no params found"))?;
                            parts = Vec::new();
                            for inner_part in data.into_inner() {
                                match inner_part.as_rule() {
                                    Rule::num => parts
                                        .push(StrPiece::Num(inner_part.as_str().parse::<f64>()?)),
                                    Rule::inner_str_text => {
                                        parts.push(StrPiece::Str(inner_part.as_str()))
                                    }
                                    r => return Err(anyhow::anyhow!("unexpected rule {:?}", r)),
                                }
                            }
                            continue;
                        }

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
                    Operation::Mult(ref mut index) => {
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
                                        break;
                                    }
                                    i += 1;
                                }
                                _ => continue,
                            }
                        }
                    }

                    Operation::MultAll => {
                        for part in &mut parts {
                            match part {
                                StrPiece::Num(n) => *n *= num,
                                StrPiece::Str(_) => continue,
                            }
                        }
                    }
                    _ => continue,
                };
            }
            _ => continue,
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

#[derive(Debug)]
enum StrPiece<'a> {
    Num(f64),
    Str(&'a str),
}

enum Operation {
    Mult(Option<isize>),
    MultAll,
    Duplicate,
}
