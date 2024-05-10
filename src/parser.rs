#![allow(dead_code)]
use crate::ast::Program;
use crate::errors::{Error, Result};
use crate::lexer::Lexer;
use crate::tokens::{Keyword as KW, Token, TokenKind as TK};
use crate::{ast::*, error};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    // TODO: Should this never be None?
    token: Option<Result<Token>>,
    /// Indicates whether the parser has encountered an error.
    has_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            token: None,
            has_error: false,
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        todo!()
    }

    fn peek(&self) -> Option<&Token> {
        todo!()
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        Ok(Program {
            block: self.parse_block()?,
        })
    }

    fn parse_block(&mut self) -> Result<Block> {
        Ok(Block {
            consts: vec![],
            vars: vec![],
            procs: vec![],
            stmts: self.parse_stmts()?,
        })
    }

    fn parse_stmts(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = vec![];

        while let Some(kind) = self.peek().map(|t| t.kind) {
            match kind {
                TK::Ident => {
                    let lhs = self.parse_ident()?;
                    self.expect_op(TK::Assign)?;
                    let rhs = self.parse_expr()?;

                    stmts.push(Stmt::Assign(Box::new(AssignStmt { lhs, rhs })));
                }
                TK::Keyword(keyword) => match keyword {
                    KW::Call => {
                        todo!("calll <ident>")
                    }
                    KW::Read => {
                        todo!("read <ident>")
                    }
                    KW::Write => {
                        todo!("write <expression>")
                    }
                    KW::Begin => {
                        todo!("sub-block")
                    }
                    KW::If => {
                        todo!("if <condition> then <statement>")
                    }
                    KW::While => {
                        todo!("while <condition> then <statement>")
                    }
                    _ => return error!("parser", "unexpected keyword: {kind:?}").into(),
                },
                _ => return error!("parser", "unexpected token: {kind:?}").into(),
            }
        }

        Ok(stmts)
    }

    /// Create a special error production.
    ///
    /// This marks the parser as having failed.
    /// The lexer will advance to the next statement, and parsing
    /// will continue, but the result will be an incorrect program.
    ///
    /// # Errors
    ///
    /// The errors production itself can encounter more errors.
    fn error_stmt(&mut self, err: Error) -> Result<ErrStmt> {
        self.has_error = true;

        while let Ok(token) = self.next_token() {
            if matches!(token.kind, TK::Semi | TK::Keyword(KW::End)) {
                break;
            }
        }

        Ok(ErrStmt { err })
    }

    fn expect_op(&mut self, token_kind: TK) -> Result<()> {
        match self.peek() {
            None => error!("parser", "unexpected end-of-file; expected {:?}", token_kind).into(),
            Some(token) => {
                if token.kind == token_kind {
                    Ok(())
                } else {
                    error!("parser", "unexpected token {:?}; expected {:?}", token.kind, token_kind).into()
                }
            }
        }
    }

    fn parse_ident(&mut self) -> Result<Ident> {
        let token = self.next_token()?;
        match token.kind {
            TK::Ident => {
                let fragment = token.fragment(self.lexer.text());
                Ok(Ident {
                    name: fragment.to_string(),
                })
            }
            _ => error!("parser", "identifier expected; found: {:?}", token.kind).into(),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        todo!()
    }
}
