//!
//! Translates the return data instructions.
//!

use crate::context::address_space::AddressSpace;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;
use inkwell::values::BasicValue;

///
/// Translates the return data size.
///
pub fn size<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    match context.get_global(crate::r#const::GLOBAL_RETURN_DATA_SIZE) {
        Ok(global) => Ok(Some(global)),
        Err(_error) => Ok(Some(context.field_const(0).as_basic_value_enum())),
    }
}

///
/// Translates the return data copy.
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
        "return_data_copy_destination_pointer",
    );

    let return_data_pointer = context
        .get_global(crate::r#const::GLOBAL_RETURN_DATA_ABI)?
        .into_pointer_value();
    let return_data_pointer = unsafe {
        context.builder().build_gep(
            return_data_pointer,
            &[source_offset],
            "return_data_source_pointer",
        )
    };
    let source = context.builder().build_pointer_cast(
        return_data_pointer,
        context.field_type().ptr_type(AddressSpace::Generic.into()),
        "return_data_source_pointer_casted",
    );

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromGeneric,
        destination,
        source,
        size,
        "return_data_copy_memcpy_from_return_data",
    );

    Ok(None)
}
