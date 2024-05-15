use crate::bytecode::{Instr, Math, OpCode};
use crate::codegen::CodeGen;
use crate::errors::Result;
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
            code: std::mem::take(&mut self.buf),
        }
    }
}

impl CodeGen for BytecodeGen {
    fn emit_lit(&mut self, num: i32) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Lit,
            l: 0,
            a: num as u16,
        });
        Ok(())
    }

    fn emit_return(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Return,
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_neg(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Neg),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_add(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Add),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_sub(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Sub),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_mul(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Mul),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_div(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Div),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_odd(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Odd),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_eq(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Eq),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_noteq(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::NotEq),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_lt(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Less),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_gte(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::GreatEq),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_gt(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::Great),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_math_lte(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Math(Math::LessEq),
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_load(&mut self, level: u8, addr: u16) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Load,
            l: level,
            a: addr,
        });
        Ok(())
    }

    fn emit_store(&mut self, level: u8, addr: u16) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Store,
            l: level,
            a: addr,
        });
        Ok(())
    }

    fn emit_call(&mut self, level: u8, addr: u16) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Call,
            l: level,
            a: addr,
        });
        Ok(())
    }

    fn emit_write(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Write,
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_read(&mut self) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Read,
            l: 0,
            a: 0,
        });
        Ok(())
    }

    fn emit_inc_top(&mut self, offset: u16) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::IncTop,
            l: 0,
            a: offset,
        });
        Ok(())
    }

    fn emit_jump(&mut self, addr: u16) -> Result<()> {
        self.buf.push(Instr {
            opcode: OpCode::Jump,
            l: 0,
            a: addr,
        });
        Ok(())
    }

    fn reserve_jump(&mut self) -> Result<usize> {
        let index = self.buf.len();
        self.buf.push(Instr {
            opcode: OpCode::Jump,
            l: 0,
            a: 0,
        });
        Ok(index)
    }

    fn reserve_jump_if_zero(&mut self) -> Result<usize> {
        let index = self.buf.len();
        self.buf.push(Instr {
            opcode: OpCode::JumpIfZero,
            l: 0,
            a: 0,
        });
        Ok(index)
    }

    fn patch_jump(&mut self, index: usize, addr: u16) -> Result<()> {
        assert!(matches!(self.buf[index].opcode, OpCode::Jump | OpCode::JumpIfZero));
        self.buf[index].a = addr;
        Ok(())
    }

    fn len(&self) -> usize {
        self.buf.len()
    }
}
