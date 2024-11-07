mod modules;

pub use modules::*;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./gramm.pest"]
///A simple grammar for parsing string multiplication commands.
pub struct StringMultGrammar;
