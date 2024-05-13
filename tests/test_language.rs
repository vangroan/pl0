#[test]
fn test_hello_world() {
    const SOURCE: &str = include_str!("hello_world.pas");
    let chunk = pl0::compile(SOURCE).expect("failed to compile");
    let mut vm = pl0::Vm::new();
    vm.eval(&chunk)
}
