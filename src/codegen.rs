//! Code generator interface.
use crate::errors::Result;

pub trait CodeGen {
    fn emit_lit(&mut self, num: i32) -> Result<()>;
    fn emit_return(&mut self) -> Result<()>;
    fn emit_math_neg(&mut self) -> Result<()>;
    fn emit_math_add(&mut self) -> Result<()>;
    fn emit_math_sub(&mut self) -> Result<()>;
    fn emit_math_mul(&mut self) -> Result<()>;
    fn emit_math_div(&mut self) -> Result<()>;
    fn emit_math_odd(&mut self) -> Result<()>;
    fn emit_math_eq(&mut self) -> Result<()>;
    fn emit_math_noteq(&mut self) -> Result<()>;
    fn emit_math_lt(&mut self) -> Result<()>;
    fn emit_math_gte(&mut self) -> Result<()>;
    fn emit_math_gt(&mut self) -> Result<()>;
    fn emit_math_lte(&mut self) -> Result<()>;
    fn emit_load(&mut self, level: u8, addr: u16) -> Result<()>;
    fn emit_store(&mut self, level: u8, addr: u16) -> Result<()>;
    fn emit_call(&mut self, level: u8, addr: u16) -> Result<()>;
    fn emit_write(&mut self) -> Result<()>;
    fn emit_read(&mut self) -> Result<()>;
    fn emit_inc_top(&mut self, offset: u16) -> Result<()>;
    fn emit_jump(&mut self, addr: u16) -> Result<()>;
    fn reserve_jump(&mut self) -> Result<usize>;
    fn reserve_jump_if_zero(&mut self) -> Result<usize>;
    fn patch_jump(&mut self, index: usize, addr: u16) -> Result<()>;

    fn len(&self) -> usize;
}
