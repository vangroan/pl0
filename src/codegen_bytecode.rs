use crate::bytecode::{Instr, OpCode};
use crate::codegen::CodeGen;
use crate::Chunk;

pub struct BytecodeGen {
    buf: Vec<Instr>,
}

impl BytecodeGen {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }

    pub fn make_chunk(&mut self) -> Chunk {
        Chunk {
            code: std::mem::replace(&mut self.buf, vec![]),
        }
    }
}

impl CodeGen for BytecodeGen {
    fn emit_lit(&mut self, num: i32) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Lit,
            l: 0,
            a: num as u16,
        });
        Ok(())
    }

    fn emit_write(&mut self) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Write,
            l: 0,
            a: 0,
        });
        Ok(())
    }
}
