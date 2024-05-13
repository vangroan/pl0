//! Code generator interface.
use crate::errors::Result;

pub trait CodeGen {
    fn emit_lit(&mut self, num: i32) -> Result<()>;
    fn emit_write(&mut self) -> Result<()>;
}
