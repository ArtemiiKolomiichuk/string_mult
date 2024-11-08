//! Provides a parser for simple commands that allow multiplying strings and evaluation functions to retrieve results.

mod modules;

pub use modules::*;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./gramm.pest"]
///A simple grammar for parsing string multiplication commands.
pub struct StringMultGrammar;
