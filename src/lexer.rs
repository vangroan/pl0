#![allow(dead_code)]
use crate::error;
use crate::errors::Result;
use crate::tokens::{Keyword, Token, TokenKind};

macro_rules! trace {
    ($($arg:tt)*) => {
        if cfg!(feature = "trace_lexer") {
            println!($($arg)*);
        }
    };
}

pub struct Lexer<'a> {
    /// Original source code text.
    text: &'a str,
    /// Remaining source code text to be lexed.
    rest: &'a str,
    /// Span of the text fragment that was consumed. `(byte_offset, size)`
    span: (u32, u32),
    /// File where the source text is from.
    pub(crate) file: String,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, file: impl ToString) -> Self {
        Self {
            text,
            rest: text,
            span: (0, 0),
            file: file.to_string(),
        }
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.ignore_whitespace();

        self.start_token();

        let token = match self.bump() {
            Some((_, ch)) => match ch {
                '0'..='9' => self.lex_number(),
                'a'..='z' | 'A'..='Z' => self.lex_ident(),
                ',' => self.make_token(TokenKind::Comma),
                '.' => self.make_token(TokenKind::Dot),
                '=' => self.make_token(TokenKind::Eq),
                '#' => self.make_token(TokenKind::Hash),
                ';' => self.make_token(TokenKind::Semi),
                '+' => self.make_token(TokenKind::Plus),
                '-' => self.make_token(TokenKind::Minus),
                '*' => self.make_token(TokenKind::Star),
                '/' => self.make_token(TokenKind::Slash),
                ':' => {
                    if self.peek() == Some('=') {
                        self.bump();
                        self.make_token(TokenKind::Assign)
                    } else {
                        return error!("lexer", "unexpected character: {ch:?}").into();
                    }
                }
                '(' => self.make_token(TokenKind::ParenLeft),
                ')' => self.make_token(TokenKind::ParenRight),
                '<' => {
                    if self.peek() == Some('=') {
                        self.bump();
                        self.make_token(TokenKind::LessEq)
                    } else {
                        self.make_token(TokenKind::Less)
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.bump();
                        self.make_token(TokenKind::GreatEq)
                    } else {
                        self.make_token(TokenKind::Great)
                    }
                }
                '!' => self.make_token(TokenKind::Keyword(Keyword::Write)),
                '?' => self.make_token(TokenKind::Keyword(Keyword::Read)),
                _ => return error!("lexer", "unexpected character {ch:?}").into(),
            },
            // End-of-file
            None => self.make_token(TokenKind::Eof),
        };

        Ok(token)
    }

    fn fragment(&self) -> &str {
        let lo = self.span.0 as usize;
        let hi = self.span.1 as usize;
        &self.text[lo..(lo + hi)]
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        match self.rest.chars().next() {
            Some(c) => {
                // Length in bytes when UTF-8 encoded.
                let char_len = c.len_utf8();
                self.rest = &self.rest[char_len..];
                self.span.1 += char_len as u32;
                Some((self.pos(), c))
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<char> {
        self.rest.chars().next()
    }

    /// Current position in the source text.
    fn pos(&self) -> usize {
        (self.rest.as_ptr() as usize) - (self.text.as_ptr() as usize)
    }

    fn start_token(&mut self) {
        self.span = (self.pos() as u32, 0);
        trace!("start token at {}:", self.span.0);
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        trace!(
            "    token {}:{} {kind:?} {:?}",
            self.span.0,
            self.span.0 + self.span.1,
            self.fragment(),
        );
        Token { kind, span: self.span }
    }
}

impl<'a> Lexer<'a> {
    /// Ignore all whitespace. Newlines are not significant to this language.
    fn ignore_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    #[rustfmt::skip]
    fn try_keyword(&self) -> Option<Keyword> {
        use crate::tokens::Keyword::*;

        match self.fragment() {
            "begin"     => Some(Begin),
            "call"      => Some(Call),
            "const"     => Some(Const),
            "do"        => Some(Do),
            "end"       => Some(End),
            "if"        => Some(If),
            "odd"       => Some(Odd),
            "procedure" => Some(Procedure),
            "read"      => Some(Read),
            "then"      => Some(Then),
            "var"       => Some(Var),
            "while"     => Some(While),
            "write"     => Some(Write),
            _ => None,
        }
    }

    /// Numbers are sequences of digits.
    fn lex_number(&mut self) -> Token {
        trace!("    lex_number()");

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.bump();
            } else {
                break;
            }
        }

        self.make_token(TokenKind::Num)
    }

    /// Identifiers start with a..z, then can contain a..z or 0..9
    ///
    /// We add our own extension to include underscores.
    fn lex_ident(&mut self) -> Token {
        trace!("    lex_ident()");

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.bump();
            } else {
                break;
            }
        }

        let kind = match self.try_keyword() {
            Some(keyword) => TokenKind::Keyword(keyword),
            None => TokenKind::Ident,
        };

        self.make_token(kind)
    }
}
