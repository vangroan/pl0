use crate::compile;

#[test]
fn test_scope() {
    const SOURCE: &str = r"
var x, y, z;

procedure foobar;
var a, b, c;
begin
    write a+x
end;

begin
    call foobar
end.
    ";
    let chunk = compile(SOURCE).expect("failed to compile");
    chunk.dump();
    // assert!(false);
}
