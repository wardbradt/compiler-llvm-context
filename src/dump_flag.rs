//!
//! The compiler tester dump flag.
//!

///
/// The intermediate representation dump flags.
///
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DumpFlag {
    /// Whether to dump the Yul code.
    Yul,
    /// Whether to dump the Ethereal IR.
    EthIR,
    /// Whether to dump the EVM code.
    EVM,
    /// Whether to dump the LLVM code.
    LLVM,
    /// Whether to dump the assembly code.
    zkEVM,
    /// Whether to dump the Vyper LLL IR.
    Vyper,
}
