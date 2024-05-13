use crate::{
    bytecode::{Instr, Math, OpCode},
    Chunk, Num,
};

macro_rules! trace {
    ($($arg:tt)*) => {
        if cfg!(feature = "trace_opcodes") {
            println!($($arg)*);
        }
    };
}

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
    fn find_base(&self, level: u8) -> usize {
        let (mut base, mut l) = (self.base, level);
        while l > 0 {
            base = self.stack[base] as usize;
            l -= 1;
        }
        base
    }
}

#[inline(always)]
fn run_interpreter(vm: &mut Vm) {
    assert!(Vm::CODE_SIZE.is_power_of_two(), "pc wrapping relies on bitwise mask");

    // let Vm {
    //     pc,
    //     base: _b,
    //     top,
    //     stack,
    //     code,
    // } = vm;

    loop {
        let Instr { opcode, l, a } = vm.code[vm.pc];
        vm.pc = (vm.pc + 1) & (Vm::CODE_SIZE - 1);

        match opcode {
            OpCode::NoOp => { /* Only pc is increased */ }
            OpCode::Lit => {
                trace!("{:04} lit {}", vm.pc, a as i32);
                vm.top += 1;
                vm.stack[vm.top] = a as i32;
            }
            OpCode::Return => {
                trace!("{:04} return", vm.pc);
                // t := b - 1; p := s[t + 3]; b := s[t + 2];
                vm.top = vm.base - 1;
                vm.pc = vm.stack[vm.top + 3] as usize;
                vm.base = vm.stack[vm.top + 2] as usize;
            }
            OpCode::Math(m) => match m {
                Math::Neg => {
                    trace!("{:04} neg", vm.pc);
                    vm.stack[vm.top] = -vm.stack[vm.top];
                }
                Math::Add => {
                    trace!("{:04} add", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] += vm.stack[vm.top + 1];
                }
                Math::Sub => {
                    trace!("{:04} sub", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] -= vm.stack[vm.top + 1];
                }
                Math::Mul => {
                    trace!("{:04} mul", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] *= vm.stack[vm.top + 1];
                }
                Math::Div => {
                    trace!("{:04} div", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] /= vm.stack[vm.top + 1];
                }
                Math::Odd => todo!(),
                Math::Eq => todo!(),
                Math::NotEq => todo!(),
                Math::Less => todo!(),
                Math::GreatEq => todo!(),
                Math::Great => todo!(),
                Math::LessEq => todo!(),
            },
            OpCode::Load => {
                trace!("{:04} load {l} {a:04}", vm.pc);
                // t := t + 1; s[t] := s[base(l) + a]
                vm.top += 1;
                vm.stack[vm.top] = vm.stack[vm.find_base(l) + a as usize];
            }
            OpCode::Store => {
                trace!("{:04} store {l} {a:04}", vm.pc);
                // s[base(l)+a] := s[t]; writeln(s[t]); t := t - 1
                vm.stack[vm.find_base(l) + a as usize] = vm.stack[vm.top];
                vm.top -= 1;
            }
            OpCode::Call => {
                trace!("{:04} call {l} {a:04}", vm.pc);
                // Generate new block mark
                vm.stack[vm.top + 1] = vm.find_base(l) as i32;
                vm.stack[vm.top + 2] = vm.base as i32;
                vm.stack[vm.top + 3] = vm.pc as i32;
                trace!(
                    "     block mark: SL={}; DL={}; RA={}",
                    vm.stack[vm.top + 1],
                    vm.stack[vm.top + 2],
                    vm.stack[vm.top + 3]
                );

                vm.base = vm.top + 1;
                vm.pc = a as usize;
            }
            OpCode::IncTop => {
                trace!("{:04} inc_top {a:04}", vm.pc);
                vm.top += a as usize;
            }
            OpCode::Jump => {
                trace!("{:04} jump {a:04}", vm.pc);
                vm.pc = a as usize;
            }
            OpCode::JumpIfFalse => todo!(),
            OpCode::Write => {
                trace!("{:04} write", vm.pc);
                println!("{}", vm.stack[vm.top]);
                vm.top -= 1;
            }
            OpCode::Read => todo!(),
        }

        // Machine halts if it jumps to bytecode zero.
        if vm.pc == 0 {
            break;
        }
    }
}
