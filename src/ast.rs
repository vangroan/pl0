#![allow(dead_code)]
use crate::errors::Error;
use crate::Num;

#[derive(Debug)]
pub struct Program {
    pub block: Block,
}

#[derive(Debug)]
pub struct Block {
    pub consts: Vec<Const>,
    pub vars: Vec<Var>,
    pub procs: Vec<Proc>,
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct Const {
    pub ident: Ident,
    pub value: Num,
}

#[derive(Debug)]
pub struct Var {
    pub ident: Ident,
}

#[derive(Debug)]
pub struct Proc {
    pub name: Ident,
    pub body: Block,
}

#[derive(Debug)]
pub enum Stmt {
    /// `<ident> := <expression>`
    Assign(Box<AssignStmt>),
    /// `call <ident>`
    Call(Box<CallStmt>),
    /// `? <ident>` or `read <ident>`
    Read(Box<ReadStmt>),
    /// `! <expression>` or `write <expression>`
    Write(WriteStmt),
    /// `begin <statements> (";" statement)? end`
    SubBlock(SubBlock),
    /// `if <condition> then <statement>`
    If(Box<IfStmt>),
    /// `while <condition> do <statement>`
    While(Box<WhileStmt>),
}

#[derive(Debug)]
/// Error while parsing statement.
pub struct ErrStmt {
    pub err: Error,
}

#[derive(Debug)]
pub struct AssignStmt {
    pub lhs: Ident,
    pub rhs: Expr,
}

#[derive(Debug)]
pub struct CallStmt {
    pub name: Ident,
}

#[derive(Debug)]
pub struct WriteStmt {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct ReadStmt {
    pub name: Ident,
}

#[derive(Debug)]
pub struct SubBlock {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub head: Cond,
    pub body: Stmt,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub head: Cond,
    pub body: Stmt,
}

#[derive(Debug, Clone, Copy)]
/// Conditional statement operator.
pub enum CondOp {
    Eq,      // =
    NotEq,   // #
    Less,    // <
    LessEq,  // <=
    Great,   // >
    GreatEq, // >=
}

#[derive(Debug)]
pub enum Cond {
    Odd(OddCond),
    Bin(BinaryCond),
}

#[derive(Debug)]
pub struct OddCond {
    pub expr: Expr,
}

#[derive(Debug)]
pub struct BinaryCond {
    pub op: CondOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Num(Num),
    Unary(Box<UnExpr>),
    Binary(Box<BinExpr>),
    Name(Ident), // var or const access
    Err(),
}

#[derive(Debug)]
/// Error while parsing expression.
pub struct ErrExpr {
    pub err: Error,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnOp {
    Pos, // +
    Neg, // -
}

#[derive(Debug)]
pub struct UnExpr {
    pub op: UnOp,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum BinOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

#[derive(Debug)]
pub struct BinExpr {
    pub op: BinOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
}

impl Stmt {
    pub fn as_writeln(&self) -> Option<&WriteStmt> {
        match self {
            Self::Write(stmt) => Some(stmt),
            _ => None,
        }
    }

    pub fn as_sub_block(&self) -> Option<&SubBlock> {
        match self {
            Self::SubBlock(stmt) => Some(stmt),
            _ => None,
        }
    }
}

impl Expr {
    pub fn as_num(&self) -> Option<Num> {
        match *self {
            Self::Num(expr) => Some(expr),
            _ => None,
        }
    }
}
