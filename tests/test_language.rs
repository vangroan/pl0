#[test]
fn test_hello_world() {
    const SOURCE: &str = include_str!("hello_world.pas");
    let chunk = pl0::compile("hello_world.pas", SOURCE).expect("failed to compile");
    let mut vm = pl0::Vm::new();
    vm.eval(&chunk)
}

#[test]
fn test_expressions() {
    const SOURCE: &str = include_str!("expressions.pas");
    let chunk = pl0::compile("expressions.pas", SOURCE).expect("failed to compile");
    let mut vm = pl0::Vm::new();
    vm.eval(&chunk)
}

#[test]
fn test_procedures() {
    const SOURCE: &str = include_str!("procedures.pas");
    let chunk = pl0::compile("procedures.pas", SOURCE).expect("failed to compile");
    chunk.dump();
    let mut vm = pl0::Vm::new();
    vm.eval(&chunk);
}

#[test]
fn test_conditionals() {
    const SOURCE: &str = include_str!("conditionals.pas");
    let chunk = pl0::compile("conditionals.pas", SOURCE).expect("failed to compile");
    chunk.dump();
    let mut vm = pl0::Vm::new();
    vm.eval(&chunk);
}

// TODO: Test robot that can input like a user.
// #[test]
fn _test_read() {
    const SOURCE: &str = include_str!("read.pas");
    let chunk = pl0::compile("read.pas", SOURCE).expect("failed to compile");
    chunk.dump();
    let mut vm = pl0::Vm::new();
    vm.eval(&chunk);
}

// TODO: Test robot that can input like a user.
// #[test]
fn _test_fibonacci() {
    const SOURCE: &str = include_str!("fibonacci.pas");
    let chunk = pl0::compile("fibonacci.pas", SOURCE).expect("failed to compile");
    chunk.dump();
    let mut vm = pl0::Vm::new();
    vm.eval(&chunk);
}

#[test]
fn test_parse_errors() {
    const SOURCE: &str = include_str!("error.pl0");
    match pl0::compile("error.pl0", SOURCE) {
        Ok(_) => panic!("unexpected success (lol)"),
        Err(err) => eprintln!("{}", err.pretty(SOURCE)),
    }
}
