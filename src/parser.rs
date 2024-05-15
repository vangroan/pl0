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
            procs: self.parse_procedures()?,
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

    fn parse_procedures(&mut self) -> Result<Vec<Proc>> {
        let mut procs = vec![];

        // Zero or more procedures.
        while let TK::Keyword(KW::Procedure) = self.peek()? {
            procs.push(self.parse_procedure()?);
        }

        Ok(procs)
    }

    fn parse_procedure(&mut self) -> Result<Proc> {
        trace!("parse_procedure");

        self.consume(TK::Keyword(KW::Procedure))?;
        let name = self.parse_ident()?;
        self.consume(TK::Semi)?;
        let body = self.parse_block()?;
        self.consume(TK::Semi)?;
        Ok(Proc { name, body })
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
            TK::Ident => self.parse_assign().map(Box::new).map(Stmt::Assign),
            TK::Keyword(keyword) => match keyword {
                KW::Call => self.parse_call().map(Box::new).map(Stmt::Call),
                KW::Read => self.parse_read().map(Box::new).map(Stmt::Read),
                KW::Write => self.parse_write().map(Stmt::Write),
                KW::Begin => self.parse_begin().map(Stmt::SubBlock),
                KW::If => self.parse_if().map(Box::new).map(Stmt::If),
                KW::While => self.parse_while().map(Box::new).map(Stmt::While),
                _ => error!("parser", "unexpected keyword: {kind:?}").into(),
            },
            TK::Eof => error!("parser", "unexpected end-of-file").into(),
            _ => error!("parser", "unexpected token: {kind:?}").into(),
        }
    }

    fn parse_assign(&mut self) -> Result<AssignStmt> {
        trace!("parse_assign");

        let lhs = self.parse_ident()?;
        self.expect_op(TK::Assign)?;
        let rhs = self.parse_expr()?;
        Ok(AssignStmt { lhs, rhs })
    }

    fn parse_call(&mut self) -> Result<CallStmt> {
        trace!("parse_call");
        self.consume(TK::Keyword(KW::Call))?;
        let name = self.parse_ident()?;
        Ok(CallStmt { name })
    }

    fn parse_write(&mut self) -> Result<WriteStmt> {
        trace!("parse_write");

        self.consume(TK::Keyword(KW::Write))?;
        let expr = self.parse_expr()?;

        Ok(WriteStmt { expr })
    }

    fn parse_read(&mut self) -> Result<ReadStmt> {
        trace!("parse_read");

        self.consume(TK::Keyword(KW::Read))?;
        let name = self.parse_ident()?;

        Ok(ReadStmt { name })
    }

    fn parse_begin(&mut self) -> Result<SubBlock> {
        trace!("parse_begin");

        self.consume(TK::Keyword(KW::Begin))?;
        let stmts = self.parse_stmts()?;
        self.consume(TK::Keyword(KW::End))?;

        Ok(SubBlock { stmts })
    }

    fn parse_if(&mut self) -> Result<IfStmt> {
        trace!("parse_if");

        self.consume(TK::Keyword(KW::If))?;
        let head = self.parse_cond()?;
        self.consume(TK::Keyword(KW::Then))?;
        let body = self.parse_stmt()?;
        Ok(IfStmt { head, body })
    }

    fn parse_while(&mut self) -> Result<WhileStmt> {
        trace!("parse_while");

        self.consume(TK::Keyword(KW::While))?;
        let head = self.parse_cond()?;
        self.consume(TK::Keyword(KW::Do))?;
        let body = self.parse_stmt()?;
        Ok(WhileStmt { head, body })
    }

    fn parse_cond(&mut self) -> Result<Cond> {
        trace!("parse_cond");

        if self.peek()? == TK::Keyword(KW::Odd) {
            self.parse_odd_cond().map(Cond::Odd)
        } else {
            self.parse_binary_cond().map(Cond::Bin)
        }
    }

    fn parse_odd_cond(&mut self) -> Result<OddCond> {
        trace!("parse_odd_cond");

        self.consume(TK::Keyword(KW::Odd))?;
        let expr = self.parse_expr()?;
        Ok(OddCond { expr })
    }

    fn parse_binary_cond(&mut self) -> Result<BinaryCond> {
        trace!("parse_binary_cond");

        let lhs = self.parse_expr()?;
        let op = self.parse_cond_op()?;
        let rhs = self.parse_expr()?;
        Ok(BinaryCond { op, lhs, rhs })
    }

    fn parse_cond_op(&mut self) -> Result<CondOp> {
        trace!("parse_cond_op");

        let token = self.next_token()?;
        let op = match token.kind {
            TK::Eq => CondOp::Eq,
            TK::Hash => CondOp::NotEq,
            TK::Less => CondOp::Less,
            TK::LessEq => CondOp::LessEq,
            TK::Great => CondOp::Great,
            TK::GreatEq => CondOp::GreatEq,
            kind => return error!("parser", "expected conditional operator; found {kind:?}").into(),
        };
        Ok(op)
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

    fn parse_expr(&mut self) -> Result<Expr> {
        trace!("parse_expr");

        // Unary expression
        let mut lhs = match self.peek()? {
            TK::Plus => {
                self.next_token()?; // +
                self.parse_term()
                    .map(|expr| UnExpr { op: UnOp::Pos, expr })
                    .map(Box::new)
                    .map(Expr::Unary)?
            }
            TK::Minus => {
                self.next_token()?; // -
                self.parse_term()
                    .map(|expr| UnExpr { op: UnOp::Neg, expr })
                    .map(Box::new)
                    .map(Expr::Unary)?
            }
            _ => self.parse_term()?,
        };

        // Binary expression (zero or more)
        loop {
            match self.peek()? {
                TK::Plus => {
                    self.next_token()?; // +
                    lhs = self
                        .parse_term()
                        .map(|rhs| BinExpr {
                            op: BinOp::Add,
                            lhs,
                            rhs,
                        })
                        .map(Box::new)
                        .map(Expr::Binary)?;
                }
                TK::Minus => {
                    self.next_token()?; // -
                    lhs = self
                        .parse_term()
                        .map(|rhs| BinExpr {
                            op: BinOp::Sub,
                            lhs,
                            rhs,
                        })
                        .map(Box::new)
                        .map(Expr::Binary)?;
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    fn parse_term(&mut self) -> Result<Expr> {
        trace!("parse_term");

        let mut lhs = self.parse_factor()?;

        // Binary expression (zero or more)
        loop {
            match self.peek()? {
                TK::Star => {
                    self.next_token()?; // *
                    lhs = self
                        .parse_term()
                        .map(|rhs| BinExpr {
                            op: BinOp::Mul,
                            lhs,
                            rhs,
                        })
                        .map(Box::new)
                        .map(Expr::Binary)?;
                }
                TK::Slash => {
                    self.next_token()?; // /
                    lhs = self
                        .parse_term()
                        .map(|rhs| BinExpr {
                            op: BinOp::Div,
                            lhs,
                            rhs,
                        })
                        .map(Box::new)
                        .map(Expr::Binary)?;
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    fn parse_factor(&mut self) -> Result<Expr> {
        trace!("parse_factor");

        match self.peek()? {
            TK::Ident => self.parse_ident().map(Expr::Name),
            TK::Num => self.parse_num().map(Expr::Num),
            TK::ParenLeft => self.parse_group(),
            kind => error!("parser", "expected identifier, number or parentheses; found {kind:?}").into(),
        }
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

    fn parse_num(&mut self) -> Result<Num> {
        trace!("parse_num");

        let token = self.consume(TK::Num)?;
        let fragment = token.fragment(self.lexer.text());
        let num = fragment
            .parse::<i32>()
            .map_err(|e| error!("parser", "failed to parse number literal: {e}"))?;
        Ok(num)
    }

    fn parse_group(&mut self) -> Result<Expr> {
        trace!("parse_group");

        self.consume(TK::ParenLeft)?;
        let expr = self.parse_expr()?;
        self.consume(TK::ParenRight)?;
        Ok(expr)
    }
}
