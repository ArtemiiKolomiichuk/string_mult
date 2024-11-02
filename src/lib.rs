use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./gramm.pest"]
pub struct Grammar;

