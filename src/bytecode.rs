//! Executable VM bytecode.
#![allow(dead_code)]

/// Bytecode instruction.
#[derive(Debug, Clone, Copy)]
pub struct Instr {
    pub opcode: OpCode,
    /// Level.
    pub l: u8,
    /// Displacement address.
    pub a: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    NoOp,
    /// Push number literal onto the top of the stack.
    Lit,
    /// Return from a procedure call.
    Return,
    /// Arithmetic Operators.
    Math(Math),
    /// Load variable onto the top of the stack.
    Load,
    Store,
    /// Call a procedure.
    Call,
    /// Add `a` to the value at the top of the stack.
    Incr,
    Jump,
    JumpIfFalse,

    // Extented operations not in original implementation.
    Write,
    Read,
}

/// Arithmetic operator instruction types.
#[derive(Debug, Clone, Copy)]
pub enum Math {
    Add,
    Sub,
    Mul,
    Div,
    Odd,
    Eq,
    NotEq,
    Less,
    GreatEq,
    Great,
    LessEq,
}

impl Default for Instr {
    fn default() -> Self {
        Self {
            opcode: OpCode::NoOp,
            l: 0,
            a: 0,
        }
    }
}
