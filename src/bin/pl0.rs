use std::{env, fs};

use pl0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = args.get(1).expect("no file path given");
    let source_text = fs::read_to_string(file_path).expect("read source file");

    match pl0::compile(source_text.as_str()) {
        Ok(chunk) => {
            let mut vm = pl0::Vm::new();
            vm.eval(&chunk);
        }
        Err(err) => {
            eprintln!("{}", err.pretty(source_text.as_str()))
        }
    }
}
