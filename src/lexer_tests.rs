use crate::lexer::Lexer;
use crate::tokens::{Keyword as KW, Token, TokenKind as TK};

#[test]
fn test_basic_program() {
    const SOURCE: &str = r"var i, s;
begin
  i: := 0; s := 0;
  while i < 5 do
  begin
    i := i + 1;
    s := s + i * i
  end
end.";
    let mut lex = Lexer::new(SOURCE);

    assert_eq!(lex.next_token().unwrap(), Token::new(TK::Keyword(KW::Var), (0, 3))); // var
    assert_eq!(lex.next_token().unwrap(), Token::new(TK::Ident, (4, 1))); // i
    assert_eq!(lex.next_token().unwrap(), Token::new(TK::Comma, (5, 1))); // ,
    assert_eq!(lex.next_token().unwrap(), Token::new(TK::Ident, (7, 1))); // s
    assert_eq!(lex.next_token().unwrap(), Token::new(TK::Semi, (8, 1))); // ;
    assert_eq!(lex.next_token().unwrap(), Token::new(TK::Keyword(KW::Begin), (10, 5))); // begin
    assert_eq!(lex.next_token().unwrap(), Token::new(TK::Ident, (18, 1))); // i

    // if let Err(err) = lex.next_token() {
    //     eprintln!("{err}");
    //     panic!();
    // }
}
