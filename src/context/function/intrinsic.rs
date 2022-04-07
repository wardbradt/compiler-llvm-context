//!
//! The LLVM intrinsic function.
//!

use inkwell::types::BasicType;

use crate::context::address_space::AddressSpace;
use crate::context::Context;
use crate::Dependency;

///
/// The LLVM intrinsic function.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intrinsic {
    /// The event emitting.
    Event,

    /// The contract context getter.
    Context,

    /// The long return.
    Return,
    /// The long revert.
    Revert,

    /// The memory copy within the heap.
    MemoryCopy,
    /// The memory copy from parent.
    MemoryCopyFromParent,
    /// The memory copy from child.
    MemoryCopyFromChild,
}

impl Intrinsic {
    ///
    /// Returns the inner LLVM intrinsic function identifier.
    ///
    pub fn name(&self) -> &'static str {
        match self {
            Intrinsic::Event => "llvm.syncvm.event",

            Intrinsic::Context => "llvm.syncvm.context",

            Intrinsic::Return => "llvm.syncvm.return",
            Intrinsic::Revert => "llvm.syncvm.revert",

            Intrinsic::MemoryCopy => "llvm.memcpy",
            Intrinsic::MemoryCopyFromParent => "llvm.memcpy",
            Intrinsic::MemoryCopyFromChild => "llvm.memcpy",
        }
    }

    ///
    /// Returns the LLVM types for selecting via the signature.
    ///
    pub fn argument_types<'ctx, 'dep, D>(
        &self,
        context: &Context<'ctx, 'dep, D>,
    ) -> Vec<inkwell::types::BasicTypeEnum<'ctx>>
    where
        D: Dependency,
    {
        match self {
            Self::Event => vec![],

            Self::Context => vec![],

            Self::Return => vec![],
            Self::Revert => vec![],

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
            Self::MemoryCopyFromParent => vec![
                context
                    .field_type()
                    .ptr_type(AddressSpace::Heap.into())
                    .as_basic_type_enum(),
                context
                    .field_type()
                    .ptr_type(AddressSpace::Parent.into())
                    .as_basic_type_enum(),
                context.field_type().as_basic_type_enum(),
            ],
            Self::MemoryCopyFromChild => vec![
                context
                    .field_type()
                    .ptr_type(AddressSpace::Heap.into())
                    .as_basic_type_enum(),
                context
                    .field_type()
                    .ptr_type(AddressSpace::Child.into())
                    .as_basic_type_enum(),
                context.field_type().as_basic_type_enum(),
            ],
        }
    }
}
