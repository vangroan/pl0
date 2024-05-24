use crate::lexer::Lexer;
use crate::tokens::{Keyword as KW, Token, TokenKind as TK};

fn tok(index: u32, size: u32, kind: TK) -> Token {
    Token::new(kind, (index, size))
}

fn kw(index: u32, size: u32, kind: KW) -> Token {
    Token::new(TK::Keyword(kind), (index, size))
}

#[test]
fn test_basic_program() {
    const SOURCE: &str = r"var i, s;
begin
  i := 0; s := 0;
  while i < 5 do
  begin
    i := i + 1;
    s := s + i * i
  end
end.";
    let mut lex = Lexer::new(SOURCE, "<test>");

    assert_eq!(lex.next_token().unwrap(), kw(0, 3, KW::Var)); // var
    assert_eq!(lex.next_token().unwrap(), tok(4, 1, TK::Ident)); // i
    assert_eq!(lex.next_token().unwrap(), tok(5, 1, TK::Comma)); // ,
    assert_eq!(lex.next_token().unwrap(), tok(7, 1, TK::Ident)); // s
    assert_eq!(lex.next_token().unwrap(), tok(8, 1, TK::Semi)); // ;
    assert_eq!(lex.next_token().unwrap(), kw(10, 5, KW::Begin)); // begin
    assert_eq!(lex.next_token().unwrap(), tok(18, 1, TK::Ident)); // i
    assert_eq!(lex.next_token().unwrap(), tok(20, 2, TK::Assign)); // :=
    assert_eq!(lex.next_token().unwrap(), tok(23, 1, TK::Num)); // 0
    assert_eq!(lex.next_token().unwrap(), tok(24, 1, TK::Semi)); // ;
    assert_eq!(lex.next_token().unwrap(), tok(26, 1, TK::Ident)); // s
    assert_eq!(lex.next_token().unwrap(), tok(28, 2, TK::Assign)); // :=
    assert_eq!(lex.next_token().unwrap(), tok(31, 1, TK::Num)); // 0
    assert_eq!(lex.next_token().unwrap(), tok(32, 1, TK::Semi)); // ;
    assert_eq!(lex.next_token().unwrap(), kw(36, 5, KW::While)); // while
    assert_eq!(lex.next_token().unwrap(), tok(42, 1, TK::Ident)); // i
    assert_eq!(lex.next_token().unwrap(), tok(44, 1, TK::Less)); // <
    assert_eq!(lex.next_token().unwrap(), tok(46, 1, TK::Num)); // 5
    assert_eq!(lex.next_token().unwrap(), kw(48, 2, KW::Do)); // do
    assert_eq!(lex.next_token().unwrap(), kw(53, 5, KW::Begin)); // begin
    assert_eq!(lex.next_token().unwrap(), tok(63, 1, TK::Ident)); // i
    assert_eq!(lex.next_token().unwrap(), tok(65, 2, TK::Assign)); // :=
    assert_eq!(lex.next_token().unwrap(), tok(68, 1, TK::Ident)); // i
    assert_eq!(lex.next_token().unwrap(), tok(70, 1, TK::Plus)); // +
    assert_eq!(lex.next_token().unwrap(), tok(72, 1, TK::Num)); // 1
    assert_eq!(lex.next_token().unwrap(), tok(73, 1, TK::Semi)); // ;
    assert_eq!(lex.next_token().unwrap(), tok(79, 1, TK::Ident)); // s
    assert_eq!(lex.next_token().unwrap(), tok(81, 2, TK::Assign)); // :=
    assert_eq!(lex.next_token().unwrap(), tok(84, 1, TK::Ident)); // s
    assert_eq!(lex.next_token().unwrap(), tok(86, 1, TK::Plus)); // +
    assert_eq!(lex.next_token().unwrap(), tok(88, 1, TK::Ident)); // i
    assert_eq!(lex.next_token().unwrap(), tok(90, 1, TK::Star)); // *
    assert_eq!(lex.next_token().unwrap(), tok(92, 1, TK::Ident)); // i
    assert_eq!(lex.next_token().unwrap(), kw(96, 3, KW::End)); // end
    assert_eq!(lex.next_token().unwrap(), kw(100, 3, KW::End)); // end
    assert_eq!(lex.next_token().unwrap(), tok(103, 1, TK::Dot)); // .
}
