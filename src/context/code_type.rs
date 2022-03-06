//!
//! The contract code types.
//!

///
/// The contract code types.
///
/// They do not represent any entityin the final bytecode, but this separation is always present
/// in the IRs used for translation to the EVM bytecode.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodeType {
    /// The constructor (deploy) code.
    Deploy,
    /// The runtime (deployed) code.
    Runtime,
}
