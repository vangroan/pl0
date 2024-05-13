use crate::{
    bytecode::{Instr, Math, OpCode},
    Chunk, Num,
};

pub struct Vm {
    /// Program counter register (instruction pointer).
    pc: usize,
    /// Base register.
    ///
    /// Index to the base of the stack for the current call frame.
    base: usize,
    /// Top register.
    ///
    /// Index to the top of the stack.
    top: usize,
    /// Operand stack.
    stack: [i32; Self::STACK_SIZE],
    code: [Instr; Self::CODE_SIZE],
}

pub struct _VmConfig {
    pub write: fn(arg: Num) -> (),
    pub read: fn() -> Num,
}

impl Vm {
    /// Maximum size of the operand stack.
    const STACK_SIZE: usize = 500;
    /// Maximum size of bytecode.
    pub const CODE_SIZE: usize = 1 << 10;

    pub fn new() -> Self {
        Self {
            pc: 0,
            base: 0,
            top: 0,
            stack: [0; Self::STACK_SIZE],
            code: [Instr::default(); Self::CODE_SIZE],
        }
    }

    pub fn eval(&mut self, chunk: &Chunk) {
        // Initialise the machine to execute the top level program.
        self.top = 0;
        self.base = 1;
        self.pc = 0;
        self.stack.fill(0);

        self.code.fill(Instr::default());

        for (idx, instr) in chunk.code.iter().enumerate() {
            self.code[idx] = *instr;
        }

        run_interpreter(self)
    }

    /// Find stack base `level` levels down.
    fn _find_base(&self, level: usize) -> usize {
        let (mut b, mut l) = (self.base, level);
        while l > 0 {
            b = self.stack[b] as usize;
            l -= 1;
        }
        l
    }
}

#[inline(always)]
fn run_interpreter(vm: &mut Vm) {
    assert!(Vm::CODE_SIZE.is_power_of_two(), "pc wrapping relies on bitwise mask");

    let Vm {
        pc,
        base: _b,
        top,
        stack,
        code,
    } = vm;

    loop {
        let Instr { opcode, l: _l, a } = code[*pc];
        *pc = (*pc + 1) & (Vm::CODE_SIZE - 1);

        match opcode {
            OpCode::NoOp => { /* Only pc is increased */ }
            OpCode::Lit => {
                *top += 1;
                stack[*top] = a as i32;
            }
            OpCode::Return => todo!(),
            OpCode::Math(m) => match m {
                Math::Neg => {
                    stack[*top] = -stack[*top];
                }
                Math::Add => {
                    *top -= 1;
                    stack[*top] = stack[*top] + stack[*top + 1];
                }
                Math::Sub => {
                    *top -= 1;
                    stack[*top] = stack[*top] - stack[*top + 1];
                }
                Math::Mul => {
                    *top -= 1;
                    stack[*top] = stack[*top] * stack[*top + 1];
                }
                Math::Div => {
                    *top -= 1;
                    stack[*top] = stack[*top] / stack[*top + 1];
                }
                Math::Odd => todo!(),
                Math::Eq => todo!(),
                Math::NotEq => todo!(),
                Math::Less => todo!(),
                Math::GreatEq => todo!(),
                Math::Great => todo!(),
                Math::LessEq => todo!(),
            },
            OpCode::Load => todo!(),
            OpCode::Store => todo!(),
            OpCode::Call => todo!(),
            OpCode::Incr => todo!(),
            OpCode::Jump => todo!(),
            OpCode::JumpIfFalse => todo!(),
            OpCode::Write => {
                println!("{}", stack[*top]);
                *top -= 1;
            }
            OpCode::Read => todo!(),
        }

        // Machine halts if it jumps to bytecode zero.
        if *pc == 0 {
            break;
        }
    }
}
