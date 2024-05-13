//! PL/0 programming language.
mod ast;
mod bytecode;
mod codegen;
mod codegen_bytecode;
mod compiler;
#[cfg(test)]
mod compiler_tests;
mod env;
mod errors;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod limits;
mod parser;
#[cfg(test)]
mod parser_tests;
mod tokens;
mod vm;

pub use self::vm::Vm;

pub mod prelude {}

pub use self::errors::{Error, Result};

/// The number type.
///
/// PL/0 is a tiny language and only has the one value type.
pub type Num = i32;

/// A chunk holds an executable program.
pub struct Chunk {
    pub(crate) code: Vec<bytecode::Instr>,
}

pub fn compile(text: &str) -> Result<Chunk> {
    let lex = lexer::Lexer::new(text);
    let mut par = parser::Parser::new(lex);
    let program = par.parse_program()?;

    let mut gen = codegen_bytecode::BytecodeGen::new();
    let mut compiler = compiler::Compiler::new(&mut gen);
    compiler.compile(&program)?;
    drop(compiler);

    Ok(gen.make_chunk())
}

impl Chunk {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn dump(&self) {
        for (idx, instr) in self.code.iter().enumerate() {
            println!(" {idx:04} {instr:?}");
        }
    }
}
