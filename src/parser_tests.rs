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

    let stmt = &program.block.stmt;
    let sub_block = stmt.as_sub_block().expect("statement isn't sub-block");
    let writeln = sub_block.stmts[0].as_writeln().expect("statement isn't writeln");
    assert_eq!(writeln.expr.as_num(), Some(42));
}

#[test]
fn test_semicolon_statements() {
    const SOURCE: &str = "begin
    write 1;
    write 2;
    write 3
end.";

    let program = parse_program(SOURCE).expect("parsing failed");
    println!("{program:#?}");

    let stmts = program.block.stmt.as_sub_block().unwrap().stmts.as_slice();
    assert_eq!(stmts[0].as_writeln().unwrap().expr.as_num(), Some(1));
    assert_eq!(stmts[1].as_writeln().unwrap().expr.as_num(), Some(2));
    assert_eq!(stmts[2].as_writeln().unwrap().expr.as_num(), Some(3));
}

#[test]
fn test_const() {
    const SOURCE: &str = "const foobar = 42;
begin
    write 1
end.";

    let program = parse_program(SOURCE).expect("parsing failed");
    println!("{program:#?}");

    let consts = program.block.consts.as_slice();
    assert_eq!(consts[0].ident.name, "foobar");
    assert_eq!(consts[0].value, 42);

    let stmts = program.block.stmt.as_sub_block().unwrap().stmts.as_slice();
    assert_eq!(stmts[0].as_writeln().unwrap().expr.as_num(), Some(1));
}

#[test]
fn test_var() {
    const SOURCE: &str = "var foobar;
begin
    write 1
end.";

    let program = parse_program(SOURCE).expect("parsing failed");
    println!("{program:#?}");

    let vars = program.block.vars.as_slice();
    assert_eq!(vars[0].ident.name, "foobar");

    let stmts = program.block.stmt.as_sub_block().unwrap().stmts.as_slice();
    assert_eq!(stmts[0].as_writeln().unwrap().expr.as_num(), Some(1));
}
