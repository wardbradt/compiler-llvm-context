//!
//! The LLVM generator function block EVM data.
//!

///
/// The LLVM generator function block EVM data.
///
/// Describes some data that is only relevant to the EVM legacy assembly.
///
#[derive(Debug, Clone)]
pub struct EVMData {
    /// The initial hash of the stack state.
    pub stack_hash: md5::Digest,
}

impl EVMData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(stack_hash: md5::Digest) -> Self {
        Self { stack_hash }
    }
}
