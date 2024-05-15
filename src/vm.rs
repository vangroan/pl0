use std::any::Any;
use std::io::{self, BufRead};

use crate::bytecode::{Instr, Math, OpCode};
use crate::limits::*;
use crate::{Chunk, Num, Pl0Config};

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
    stack: [i32; STACK_SIZE],
    /// Executable bytecode.
    code: [Instr; CODE_SIZE],
    /// User injected callbacks and data.
    config: Pl0Config,
}

/// Default `write <expr>` handler.
pub(crate) fn default_write(_user_data: Option<&dyn Any>, arg: Num) {
    println!("{arg}");
}

/// Default `read <ident>` handler.
pub(crate) fn default_read(_user_data: Option<&dyn Any>) -> Option<Num> {
    let stdin = io::stdin();
    match stdin.lock().lines().next() {
        Some(result) => match result {
            Ok(line) => match line.parse::<i32>() {
                Ok(num) => Some(num),
                Err(err) => {
                    eprintln!("failed to read line: {err}");
                    None
                }
            },
            Err(err) => {
                eprintln!("failed to read line: {err}");
                None
            }
        },
        None => None,
    }
}

impl Vm {
    pub fn new() -> Self {
        Self::from_config(Pl0Config::new())
    }

    pub fn from_config(config: Pl0Config) -> Self {
        Self {
            pc: 0,
            base: 0,
            top: 0,
            stack: [0; STACK_SIZE],
            code: [Instr::default(); CODE_SIZE],
            config,
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

    fn user_data(&self) -> Option<&dyn Any> {
        self.config.user_data.as_deref()
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

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

#[inline(always)]
fn run_interpreter(vm: &mut Vm) {
    assert!(CODE_SIZE.is_power_of_two(), "pc wrapping relies on bitwise mask");

    loop {
        let Instr { opcode, l, a } = vm.code[vm.pc];
        vm.pc = (vm.pc + 1) & (CODE_SIZE - 1);

        match opcode {
            OpCode::NoOp => { /* Only pc is increased */ }
            OpCode::Lit => {
                trace!("{:04} lit {}", vm.pc, a as i32);
                vm.top += 1;
                vm.stack[vm.top] = a as i32;
            }
            OpCode::Return => {
                trace!("{:04} return", vm.pc);
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
                Math::Odd => {
                    trace!("{:04} odd", vm.pc);
                    vm.stack[vm.top] = if vm.stack[vm.top] % 2 == 0 { 0 } else { 1 };
                }
                Math::Eq => {
                    trace!("{:04} eq", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] = if vm.stack[vm.top] == vm.stack[vm.top + 1] { 1 } else { 0 };
                }
                Math::NotEq => {
                    trace!("{:04} neq", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] = if vm.stack[vm.top] != vm.stack[vm.top + 1] { 1 } else { 0 };
                }
                Math::Less => {
                    trace!("{:04} lt", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] = if vm.stack[vm.top] < vm.stack[vm.top + 1] { 1 } else { 0 };
                }
                Math::GreatEq => {
                    trace!("{:04} gte", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] = if vm.stack[vm.top] >= vm.stack[vm.top + 1] { 1 } else { 0 };
                }
                Math::Great => {
                    trace!("{:04} gt", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] = if vm.stack[vm.top] > vm.stack[vm.top + 1] { 1 } else { 0 };
                }
                Math::LessEq => {
                    trace!("{:04} lte", vm.pc);
                    vm.top -= 1;
                    vm.stack[vm.top] = if vm.stack[vm.top] <= vm.stack[vm.top + 1] { 1 } else { 0 };
                }
            },
            OpCode::Load => {
                trace!("{:04} load {l} {a:04}", vm.pc);
                vm.top += 1;
                vm.stack[vm.top] = vm.stack[vm.find_base(l) + a as usize];
            }
            OpCode::Store => {
                trace!("{:04} store {l} {a:04}", vm.pc);
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
            OpCode::JumpIfZero => {
                trace!("{:04} jpc {a:04}", vm.pc);
                vm.top -= 1;
                if vm.stack[vm.top + 1] == 0 {
                    vm.pc = a as usize;
                }
            }
            OpCode::Write => {
                trace!("{:04} write", vm.pc);
                (vm.config.write)(vm.user_data(), vm.stack[vm.top]);
                vm.top -= 1;
            }
            OpCode::Read => {
                trace!("{:04} read", vm.pc);
                vm.top += 1;
                vm.stack[vm.top] = (vm.config.read)(vm.user_data()).unwrap_or_default();
            }
        }

        // Machine halts if it jumps to bytecode zero.
        if vm.pc == 0 {
            break;
        }
    }
}
