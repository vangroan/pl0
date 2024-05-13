use crate::bytecode::{Instr, Math, OpCode};
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

    fn emit_return(&mut self) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Return,
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_neg(&mut self) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Neg),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_add(&mut self) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Add),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_sub(&mut self) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Sub),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_mul(&mut self) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Mul),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_div(&mut self) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Div),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_load(&mut self, level: u8, addr: u16) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Load,
            l: level,
            a: addr,
        });
        Ok(())
    }

    fn emit_store(&mut self, level: u8, addr: u16) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Store,
            l: level,
            a: addr,
        });
        Ok(())
    }

    fn emit_call(&mut self, level: u8, addr: u16) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Call,
            l: level,
            a: addr,
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

    fn emit_inc_top(&mut self, offset: u16) -> crate::Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::IncTop,
            l: 0,
            a: offset,
        });
        Ok(())
    }

    fn reserve_jump(&mut self) -> crate::Result<usize> {
        let index = self.buf.len();
        self.buf.push(Instr {
            opcode: OpCode::Jump,
            l: 0,
            a: 0,
        });
        Ok(index)
    }

    fn patch_jump(&mut self, index: usize, addr: u16) -> crate::Result<()> {
        assert_eq!(self.buf[index].opcode, OpCode::Jump);
        self.buf[index].a = addr;
        Ok(())
    }

    fn len(&self) -> usize {
        self.buf.len()
    }
}
