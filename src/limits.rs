//! Capability constants.

/// The stack offset, relative to the current call frame's base, where
/// the frame's variables start.
///
/// The first three slots of each frame's stack space is taken up
/// by the block mark (static link, dynamic link, return address).
pub const DATA_OFFSET: usize = 3;

/// Maximum size of the operand stack.
pub const STACK_SIZE: usize = 1 << 9; // 512

/// Maximum size of bytecode.
pub const CODE_SIZE: usize = 1 << 10; // 1024
