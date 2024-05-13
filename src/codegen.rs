//! Code generator interface.
use crate::errors::Result;

pub trait CodeGen {
    fn emit_lit(&mut self, num: i32) -> Result<()>;
    fn emit_math_neg(&mut self) -> Result<()>;
    fn emit_math_add(&mut self) -> Result<()>;
    fn emit_math_sub(&mut self) -> Result<()>;
    fn emit_math_mul(&mut self) -> Result<()>;
    fn emit_math_div(&mut self) -> Result<()>;
    fn emit_write(&mut self) -> Result<()>;
}
