//! Lexical tokens.
#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: (usize, usize),
}

impl Token {
    pub fn new(kind: TokenKind, span: (usize, usize)) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum TokenKind {
    ColonEq, // :=
    Comma,   // ,
    Dot,     // .
    Eq,      // =
    Hash,    // #
    Semi,    // ;

    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /

    ParenLeft,   // (
    ParenRight,  // )

    Lt,      // <
    LtEq,    // <=
    Gt,      // >
    GtEq,    // >=

    Ident,   // identifier
    Num,     // integer literal

    Keyword(Keyword),

    EOF,     // End-of-file
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Begin,
    Call,
    Const,
    Do,
    End,
    If,
    Odd,
    Procedure,
    Read, // Same as ? <ident>
    Then,
    Var,
    While,
    Write, // Same as ! <expression>
}
