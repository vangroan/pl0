#![allow(dead_code)]
use crate::ast::Program;
use crate::errors::{Error, Result};
use crate::lexer::Lexer;
use crate::tokens::{Keyword as KW, Token, TokenKind as TK};
use crate::{ast::*, error, Num};

macro_rules! trace {
    ($($arg:tt)*) => {
        if cfg!(feature = "trace_parser") {
            println!($($arg)*);
        }
    };
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    /// The next token, or a lexical error.
    token: Option<Result<Token>>,
    /// Indicates whether the parser has encountered an error.
    has_error: bool,
    errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            token: None,
            has_error: false,
            errors: vec![],
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        match self.token.take() {
            Some(maybe_token) => maybe_token,
            None => self.lexer.next_token(),
        }
    }

    fn peek(&mut self) -> Result<TK> {
        if self.token.is_none() {
            self.token = Some(self.lexer.next_token());
        }

        match self.token.as_ref().unwrap() {
            Ok(token) => Ok(token.kind),
            // Cloning the error greatly simplifies the parselets.
            Err(err) => Err(err.clone()),
        }
    }

    fn consume(&mut self, token_kind: TK) -> Result<Token> {
        let kind = self.peek()?;
        if kind == token_kind {
            self.next_token()
        } else {
            Err(error!("parser", "{:?} expected; found {:?}", token_kind, kind))
        }
    }
}

/// Parselet functions.
impl<'a> Parser<'a> {
    pub fn parse_program(&mut self) -> Result<Program> {
        trace!("parse_program");

        let block = self.parse_block()?;
        self.consume(TK::Dot)?;

        Ok(Program { block })
    }

    fn parse_block(&mut self) -> Result<Block> {
        trace!("parse_block");

        Ok(Block {
            consts: self.parse_consts()?,
            vars: self.parse_vars()?,
            procs: vec![],
            stmt: self.parse_stmt()?,
        })
    }

    fn parse_consts(&mut self) -> Result<Vec<Const>> {
        trace!("parse_consts");

        // `const` section is optional
        if self.peek()? == TK::Keyword(KW::Const) {
            self.next_token()?;
        } else {
            return Ok(vec![]);
        }

        // Must have at least one constant declaration.
        let mut consts = vec![self.parse_const_assign()?];

        loop {
            match self.peek()? {
                TK::Comma => {
                    self.next_token()?; // ,
                    consts.push(self.parse_const_assign()?);
                }
                TK::Semi => {
                    self.next_token()?;
                    break;
                }
                TK::Eof => {
                    return error!("parser", "unexpected end-of-file").into();
                }
                kind => {
                    return error!("parser", "expected comma or semicolon; found {kind:?}").into();
                }
            }
        }

        Ok(consts)
    }

    fn parse_const_assign(&mut self) -> Result<Const> {
        let ident = self.parse_ident()?;
        self.consume(TK::Eq)?;
        let value = self.parse_num()?;

        Ok(Const { ident, value })
    }

    fn parse_vars(&mut self) -> Result<Vec<Var>> {
        trace!("parse_vars");

        // `var` section is optional
        if self.peek()? == TK::Keyword(KW::Var) {
            self.next_token()?;
        } else {
            return Ok(vec![]);
        }

        // Must have at least one variable declaration.
        let mut vars = vec![Var {
            ident: self.parse_ident()?,
        }];

        loop {
            match self.peek()? {
                TK::Comma => {
                    self.next_token()?;
                    vars.push(Var {
                        ident: self.parse_ident()?,
                    });
                }
                TK::Semi => {
                    self.next_token()?;
                    break;
                }
                TK::Eof => {
                    return error!("parser", "unexpected end-of-file").into();
                }
                kind => return error!("parser", "expected comma or semicolon; found {kind:?}").into(),
            }
        }

        Ok(vars)
    }

    fn parse_stmts(&mut self) -> Result<Vec<Stmt>> {
        trace!("parse_stmts");

        // Must have at least one statement.
        let mut stmts = vec![self.parse_stmt()?];

        loop {
            match self.peek()? {
                TK::Keyword(KW::End) => {
                    break;
                }
                TK::Semi => {
                    self.next_token()?; // ;
                    stmts.push(self.parse_stmt()?);
                }
                TK::Eof => {
                    return error!("parser", "unexpected end-of-file").into();
                }
                kind => {
                    return error!("parser", "expected semicolon or 'end'; found {kind:?}").into();
                }
            }
        }

        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        trace!("parse_stmt");

        let kind = self.peek()?;
        match kind {
            TK::Ident => {
                let lhs = self.parse_ident()?;
                self.expect_op(TK::Assign)?;
                let rhs = self.parse_expr()?;

                Ok(Stmt::Assign(Box::new(AssignStmt { lhs, rhs })))
            }
            TK::Keyword(keyword) => match keyword {
                KW::Call => {
                    todo!("calll <ident>")
                }
                KW::Read => {
                    todo!("read <ident>")
                }
                KW::Write => Ok(self.parse_write().map(Stmt::Write)?),
                KW::Begin => Ok(self.parse_begin().map(Stmt::SubBlock)?),
                KW::If => {
                    todo!("if <condition> then <statement>")
                }
                KW::While => {
                    todo!("while <condition> then <statement>")
                }
                _ => error!("parser", "unexpected keyword: {kind:?}").into(),
            },
            TK::Eof => error!("parser", "unexpected end-of-file").into(),
            _ => error!("parser", "unexpected token: {kind:?}").into(),
        }
    }

    fn parse_write(&mut self) -> Result<WriteStmt> {
        trace!("parse_write");

        self.consume(TK::Keyword(KW::Write))?;
        let expr = self.parse_expr()?;

        Ok(WriteStmt { expr })
    }

    fn parse_begin(&mut self) -> Result<SubBlock> {
        trace!("parse_begin");

        self.consume(TK::Keyword(KW::Begin))?;
        let stmts = self.parse_stmts()?;
        self.consume(TK::Keyword(KW::End))?;

        Ok(SubBlock { stmts })
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

    fn expect_op(&mut self, token_kind: TK) -> Result<Token> {
        trace!("expect_op");

        self.consume(token_kind)
    }

    fn parse_ident(&mut self) -> Result<Ident> {
        trace!("parse_ident");

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
        trace!("parse_expr");

        match self.peek()? {
            TK::Num => {
                let token = self.next_token()?;
                let fragment = token.fragment(self.lexer.text());
                let num = fragment
                    .parse::<i32>()
                    .map_err(|e| error!("parser", "failed to parse number literal: {e}"))?;
                Ok(Expr::Num(num))
            }
            _ => todo!("expression parsing"),
        }
    }

    fn parse_num(&mut self) -> Result<Num> {
        let token = self.consume(TK::Num)?;
        let fragment = token.fragment(self.lexer.text());
        let num = fragment
            .parse::<i32>()
            .map_err(|e| error!("parser", "failed to parse number literal: {e}"))?;
        Ok(num)
    }
}
