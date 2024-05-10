use crate::ast::*;
use crate::errors::Result;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn parse_program(text: &str) -> Result<Program> {
    let lex = Lexer::new(text);
    let mut parser = Parser::new(lex);
    parser.parse_program()
}

#[test]
fn test_basic_program() {
    const SOURCE: &str = "begin write 42 end.";

    let program = parse_program(SOURCE).expect("parsing failed");
    println!("{program:#?}");

    let stmt = &program.block.stmts[0];
    let sub_block = stmt.as_sub_block().expect("statement isn't sub-block");
    let writeln = sub_block.stmts[0].as_writeln().expect("statement isn't writeln");
    assert_eq!(writeln.expr.as_num(), Some(42));
}
