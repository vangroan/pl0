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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Increase the stack top register by `a`.
    IncTop,
    Jump,
    JumpIfZero,

    // Extented operations not in original implementation.
    Write,
    Read,
}

/// Arithmetic operator instruction types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Math {
    Neg,
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
