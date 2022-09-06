//!
//! Translates the calldata instructions.
//!

use crate::context::address_space::AddressSpace;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the calldata load.
///
pub fn load<'ctx, D>(
    context: &mut Context<'ctx, D>,
    offset: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let calldata_pointer = context.get_global(crate::r#const::GLOBAL_CALLDATA_ABI)?;
    let calldata_pointer = unsafe {
        context.builder().build_gep(
            calldata_pointer.into_pointer_value(),
            &[offset],
            "calldata_pointer_with_offset",
        )
    };
    let calldata_pointer_casted = context.builder().build_pointer_cast(
        calldata_pointer,
        context.field_type().ptr_type(AddressSpace::Generic.into()),
        "calldata_pointer_casted",
    );
    let value = context.build_load(calldata_pointer_casted, "calldata_value");

    Ok(Some(value))
}

///
/// Translates the calldata size.
///
pub fn size<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let value = context.get_global(crate::r#const::GLOBAL_CALLDATA_SIZE)?;

    Ok(Some(value))
}

///
/// Translates the calldata copy.
///
pub fn copy<'ctx, D>(
    context: &mut Context<'ctx, D>,
    destination_offset: inkwell::values::IntValue<'ctx>,
    source_offset: inkwell::values::IntValue<'ctx>,
    size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let destination = context.access_memory(
        destination_offset,
        AddressSpace::Heap,
        "calldata_copy_destination_pointer",
    );

    let calldata_pointer = context
        .get_global(crate::r#const::GLOBAL_CALLDATA_ABI)?
        .into_pointer_value();
    let calldata_pointer = unsafe {
        context.builder().build_gep(
            calldata_pointer,
            &[source_offset],
            "calldata_source_pointer",
        )
    };
    let source = context.builder().build_pointer_cast(
        calldata_pointer,
        context.field_type().ptr_type(AddressSpace::Generic.into()),
        "calldata_source_pointer_casted",
    );

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromGeneric,
        destination,
        source,
        size,
        "calldata_copy_memcpy_from_child",
    );

    Ok(None)
}
