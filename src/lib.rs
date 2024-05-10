//! PL/0 programming language.
mod ast;
mod errors;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod parser;
#[cfg(test)]
mod parser_tests;
mod tokens;

pub mod prelude {}

pub use self::errors::{Error, Result};

/// The number type.
///
/// PL/0 is a tiny language and only has the one value type.
pub type Num = i32;

pub fn compile(text: &str) -> Result<()> {
    let lex = lexer::Lexer::new(text);
    let mut par = parser::Parser::new(lex);
    let _ = par.parse_program()?;

    Ok(())
}
