//!
//! Translates the calldata instructions.
//!

use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the calldata load.
///
pub fn load<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    arguments: [inkwell::values::BasicValueEnum<'ctx>; 1],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let parent_offset_pointer = context.access_memory(
        context.field_const(
            (compiler_common::ABI_MEMORY_OFFSET_DATA_OFFSET * compiler_common::SIZE_FIELD) as u64,
        ),
        AddressSpace::Heap,
        "calldata_parent_offset_pointer",
    );
    let parent_offset = context.build_load(parent_offset_pointer, "calldata_parent_offset");
    let offset = context.builder().build_int_add(
        arguments[0].into_int_value(),
        parent_offset.into_int_value(),
        "calldata_offset",
    );

    let pointer = context.access_memory(offset, AddressSpace::Parent, "calldata_pointer");
    let value = context.build_load(pointer, "calldata_value");

    Ok(Some(value))
}

///
/// Translates the calldata size.
///
pub fn size<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let length_pointer = context.access_memory(
        context.field_const(
            (compiler_common::ABI_MEMORY_OFFSET_DATA_LENGTH * compiler_common::SIZE_FIELD) as u64,
        ),
        AddressSpace::Heap,
        "calldata_size_length_pointer",
    );
    let length = context.build_load(length_pointer, "calldata_value");

    Ok(Some(length.as_basic_value_enum()))
}

///
/// Translates the calldata copy.
///
pub fn copy<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    arguments: [inkwell::values::BasicValueEnum<'ctx>; 3],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let destination = context.access_memory(
        arguments[0].into_int_value(),
        AddressSpace::Heap,
        "calldata_copy_destination_pointer",
    );

    let parent_offset_pointer = context.access_memory(
        context.field_const(
            (compiler_common::ABI_MEMORY_OFFSET_DATA_OFFSET * compiler_common::SIZE_FIELD) as u64,
        ),
        AddressSpace::Heap,
        "calldata_copy_parent_offset_pointer",
    );
    let parent_offset = context.build_load(parent_offset_pointer, "calldata_copy_parent_offset");
    let source_offset = context.builder().build_int_add(
        arguments[0].into_int_value(),
        parent_offset.into_int_value(),
        "calldata_copy_source_offset",
    );
    let source = context.access_memory(
        source_offset,
        AddressSpace::Parent,
        "calldata_copy_source_pointer",
    );

    let size = arguments[2].into_int_value();

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromParent,
        destination,
        source,
        size,
        "calldata_copy_memcpy_from_child",
    );

    Ok(None)
}

///
/// Translates the calldata copy from the `codecopy` instruction.
///
pub fn codecopy<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    arguments: [inkwell::values::BasicValueEnum<'ctx>; 3],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let destination = context.access_memory(
        arguments[0].into_int_value(),
        AddressSpace::Heap,
        "calldata_codecopy_destination_pointer",
    );

    let parent_offset_pointer = context.access_memory(
        context.field_const(
            (compiler_common::ABI_MEMORY_OFFSET_DATA_OFFSET * compiler_common::SIZE_FIELD) as u64,
        ),
        AddressSpace::Heap,
        "calldata_codecopy_parent_offset_pointer",
    );
    let parent_offset =
        context.build_load(parent_offset_pointer, "calldata_codecopy_parent_offset");
    let source_offset = context.builder().build_int_add(
        arguments[0].into_int_value(),
        parent_offset.into_int_value(),
        "calldata_codecopy_source_offset",
    );
    let source = context.access_memory(
        source_offset,
        AddressSpace::Parent,
        "calldata_codecopy_source_pointer",
    );

    let size = arguments[2].into_int_value();

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromParent,
        destination,
        source,
        size,
        "calldata_codecopy_memcpy_from_parent",
    );

    Ok(None)
}
