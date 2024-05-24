//! Lexical tokens.
#![allow(dead_code)]
use std::fmt::{self, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: (u32, u32),
}

impl Token {
    pub fn new(kind: TokenKind, span: (u32, u32)) -> Self {
        Self { kind, span }
    }

    pub fn fragment<'a>(&self, text: &'a str) -> &'a str {
        let (lo, hi) = self.span;
        let lo = lo as usize;
        let hi = hi as usize;
        &text[lo..lo + hi]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum TokenKind {
    Assign,  // :=
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

    Less,        // <
    LessEq,      // <=
    Great,       // >
    GreatEq,     // >=

    Ident,   // identifier
    Num,     // integer literal

    Keyword(Keyword),

    Eof,     // End-of-file
}

impl TokenKind {
    #[rustfmt::skip]
    pub fn name(&self) -> &str {
        match self {
            TokenKind::Assign => "assignment",
            TokenKind::Comma  => "comma",
            TokenKind::Dot    => "period",
            TokenKind::Eq     => "equal sign",
            TokenKind::Hash   => "hash",
            TokenKind::Semi   => "semicolon",
            TokenKind::Plus   => "plus",
            TokenKind::Minus  => "minus",
            TokenKind::Star   => "asterisk",
            TokenKind::Slash  => "forward slash",
            TokenKind::ParenLeft  => "left parentheses",
            TokenKind::ParenRight => "right parentheses",
            TokenKind::Less   => "less than",
            TokenKind::LessEq => "less-or-equal than",
            TokenKind::Great  => "greater than",
            TokenKind::GreatEq => "greater-or-equal than",
            TokenKind::Ident  => "identifier",
            TokenKind::Num    => "number",
            TokenKind::Keyword(kw) => kw.name(),
            TokenKind::Eof    => "end-of-file",
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Display::fmt(self.name(), f)
    }
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

impl Keyword {
    #[rustfmt::skip]
    pub fn name(&self) -> &str {
        match self {
            Keyword::Begin => "'begin'",
            Keyword::Call  => "'call'",
            Keyword::Const => "'const'",
            Keyword::Do    => "'do'",
            Keyword::End   => "'end'",
            Keyword::If    => "'if'",
            Keyword::Odd   => "'odd'",
            Keyword::Procedure => "'procedure'",
            Keyword::Read  => "'read'",
            Keyword::Then  => "'then'",
            Keyword::Var   => "'var'",
            Keyword::While => "'while'",
            Keyword::Write => "'write'",
        }
    }
}
