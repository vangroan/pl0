#![allow(dead_code)]
use crate::errors::Error;
use crate::Num;

pub struct Program {
    pub block: Block,
}

pub struct Block {
    pub consts: Vec<Const>,
    pub vars: Vec<Var>,
    pub procs: Vec<Proc>,
    pub stmts: Vec<Stmt>,
}

pub struct Const {
    pub ident: Ident,
    pub value: Num,
}

pub struct Var {
    pub ident: Ident,
}

pub struct Proc {
    pub ident: Ident,
    pub body: Block,
}

pub enum Stmt {
    /// `<ident> := <expression>`
    Assign(Box<AssignStmt>),
    /// `call <ident>`
    Call,
    /// `? <ident>` or `read <ident>`
    Read,
    /// `! <expression>` or `write <expression>`
    Write,
    /// `begin <statements> (";" statement)? end`
    SubBlock(Vec<Stmt>),
    /// `if <condition> then <statement>`
    If(Box<IfStmt>),
    /// `while <condition> do <statement>`
    While(Box<WhileStmt>),
}

/// Error while parsing statement.
pub struct ErrStmt {
    pub err: Error,
}

pub struct AssignStmt {
    pub lhs: Ident,
    pub rhs: Expr,
}

pub struct IfStmt {
    pub cond: Cond,
    pub stmt: Box<Stmt>,
}

pub struct WhileStmt {
    pub cond: Cond,
    pub stmt: Box<Stmt>,
}

/// Conditional statement operator.
pub enum CondOp {
    Eq,      // =
    NotEq,   // #
    Less,    // <
    LessEq,  // <=
    Great,   // >
    GreatEq, // >=
}

pub struct Cond {
    pub is_odd: bool,
    pub op: CondOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

pub enum Expr {
    Binary(Box<BinExpr>),
    Err(),
}

/// Error while parsing expression.
pub struct ErrExpr {
    pub err: Error,
}

pub struct BinExpr {}

pub struct Ident {
    pub name: String,
}
