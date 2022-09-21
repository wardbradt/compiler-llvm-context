//!
//! The LLVM generator function block key.
//!

use crate::context::code_type::CodeType;

///
/// The LLVM generator function block key.
///
/// Is only relevant to the EVM legacy assembly.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key {
    /// The block code type.
    pub code_type: CodeType,
    /// The block tag.
    pub tag: num::BigUint,
}

impl Key {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(code_type: CodeType, tag: num::BigUint) -> Self {
        Self { code_type, tag }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.code_type, self.tag)
    }
}
