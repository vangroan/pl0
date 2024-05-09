//! PL/0 programming language.
mod ast;
mod errors;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod parser;
mod tokens;

pub mod prelude {}

pub use self::errors::{Error, Result};

pub fn compile(text: &str) -> Result<()> {
    let lex = lexer::Lexer::new(text);
    let mut par = parser::Parser::new(lex);
    let _ = par.parse_program()?;

    Ok(())
}
