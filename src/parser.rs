#![allow(dead_code)]

use crate::ast::Program;
use crate::errors::Result;
use crate::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        todo!()
    }
}
