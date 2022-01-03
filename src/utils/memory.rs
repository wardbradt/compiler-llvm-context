//!
//! Translates the heap memory operations.
//!

use crate::context::address_space::AddressSpace;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the heap memory load.
///
pub fn load<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    arguments: [inkwell::values::BasicValueEnum<'ctx>; 1],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let pointer = context.access_memory(
        arguments[0].into_int_value(),
        AddressSpace::Heap,
        "memory_load_pointer",
    );
    let result = context.build_load(pointer, "memory_load_result");
    Ok(Some(result))
}

///
/// Translates the heap memory store.
///
pub fn store<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    arguments: [inkwell::values::BasicValueEnum<'ctx>; 2],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let offset = arguments[0].into_int_value();
    let pointer = context.access_memory(offset, AddressSpace::Heap, "memory_store_pointer");
    context.build_store(pointer, arguments[1]);

    Ok(None)
}
