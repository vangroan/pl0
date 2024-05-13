//! Capability constants.

/// The stack offset, relative to the current call frame's base, where
/// the frame's variables start.
///
/// The first three slots of each frame's stack space is taken up
/// by the block mark.
pub const DATA_OFFSET: usize = 3;
