//!
//! The LLVM intrinsic function.
//!

use inkwell::types::BasicType;

use crate::context::address_space::AddressSpace;
use crate::context::Context;
use crate::Dependency;

///
/// The LLVM intrinsic function, implemented in the LLVM back-end.
///
/// Most of the intrinsics are translated directly into bytecode instructions.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intrinsic {
    /// The event emitting.
    Event,
    /// The L1 interactor.
    ToL1,
    /// The precompile call.
    Precompile,
    /// The near call with ABI data.
    NearCall,
    /// The current contract's address.
    Address,
    /// The caller's address.
    Caller,
    /// The address where the current contract's code is deployed.
    CodeSource,
    /// The other data, including the block information and VM state.
    Meta,
    /// The remaining amount of ergs.
    ErgsLeft,
    /// The abstract `u128` getter.
    GetU128,
    /// The abstract `u128` setter.
    SetU128,
    /// The public data price setter.
    SetPubdataPrice,
    /// The transaction counter incrementor.
    IncrementTxCounter,
    /// The pointer shrink.
    PointerShrink,
    /// The pointer pack.
    PointerPack,

    /// The long return.
    Return,
    /// The long revert.
    Revert,

    /// The memory copy within the heap.
    MemoryCopy,
    /// The memory copy from a generic page.
    MemoryCopyFromGeneric,
}

impl Intrinsic {
    ///
    /// Returns the inner LLVM intrinsic function identifier.
    ///
    pub fn name(&self) -> &'static str {
        match self {
            Intrinsic::Event => "llvm.syncvm.event",
            Intrinsic::ToL1 => "llvm.syncvm.tol1",
            Intrinsic::Precompile => "llvm.syncvm.precompile",
            Intrinsic::NearCall => "llvm.syncvm.nearcall",
            Intrinsic::Address => "llvm.syncvm.this",
            Intrinsic::Caller => "llvm.syncvm.caller",
            Intrinsic::CodeSource => "llvm.syncvm.codesource",
            Intrinsic::Meta => "llvm.syncvm.meta",
            Intrinsic::ErgsLeft => "llvm.syncvm.ergsleft",
            Intrinsic::GetU128 => "llvm.syncvm.getu128",
            Intrinsic::SetU128 => "llvm.syncvm.setu128",
            Intrinsic::SetPubdataPrice => "llvm.syncvm.setpubdataprice",
            Intrinsic::IncrementTxCounter => "llvm.syncvm.inctx",
            Intrinsic::PointerShrink => "llvm.syncvm.ptr.shrink",
            Intrinsic::PointerPack => "llvm.syncvm.ptr.pack",

            Intrinsic::Return => "llvm.syncvm.return",
            Intrinsic::Revert => "llvm.syncvm.revert",

            Intrinsic::MemoryCopy => "llvm.memcpy",
            Intrinsic::MemoryCopyFromGeneric => "llvm.memcpy",
        }
    }

    ///
    /// Returns the LLVM types for selecting via the signature.
    ///
    pub fn argument_types<'ctx, D>(
        &self,
        context: &Context<'ctx, D>,
    ) -> Vec<inkwell::types::BasicTypeEnum<'ctx>>
    where
        D: Dependency,
    {
        match self {
            Self::MemoryCopy => vec![
                context
                    .field_type()
                    .ptr_type(AddressSpace::Heap.into())
                    .as_basic_type_enum(),
                context
                    .field_type()
                    .ptr_type(AddressSpace::Heap.into())
                    .as_basic_type_enum(),
                context.field_type().as_basic_type_enum(),
            ],
            Self::MemoryCopyFromGeneric => vec![
                context
                    .field_type()
                    .ptr_type(AddressSpace::Heap.into())
                    .as_basic_type_enum(),
                context
                    .field_type()
                    .ptr_type(AddressSpace::Generic.into())
                    .as_basic_type_enum(),
                context.field_type().as_basic_type_enum(),
            ],
            _ => vec![],
        }
    }
}
