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
            (compiler_common::ABI_MEMORY_OFFSET_CALLDATA_OFFSET * compiler_common::SIZE_FIELD)
                as u64,
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
            (compiler_common::ABI_MEMORY_OFFSET_CALLDATA_LENGTH * compiler_common::SIZE_FIELD)
                as u64,
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
    let memory_zero_block = context.append_basic_block("calldata_copy_memory_zero_block");
    let default_block = context.append_basic_block("calldata_copy_default_block");
    let join_block = context.append_basic_block("calldata_copy_join_block");

    let destination_offset = arguments[0].into_int_value();
    let destination = context.access_memory(
        destination_offset,
        AddressSpace::Heap,
        "calldata_copy_destination_pointer",
    );

    let parent_offset_pointer = context.access_memory(
        context.field_const(
            (compiler_common::ABI_MEMORY_OFFSET_CALLDATA_OFFSET * compiler_common::SIZE_FIELD)
                as u64,
        ),
        AddressSpace::Heap,
        "calldata_copy_parent_offset_pointer",
    );
    let parent_offset = context.build_load(parent_offset_pointer, "calldata_copy_parent_offset");
    let source_offset = arguments[1].into_int_value();
    let combined_offset = context.builder().build_int_add(
        source_offset,
        parent_offset.into_int_value(),
        "calldata_copy_combined_offset",
    );

    let size = arguments[2].into_int_value();

    let calldata_size = self::size(context)?
        .expect("Always exists")
        .into_int_value();
    let is_source_calldata_size = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        arguments[1].into_int_value(),
        calldata_size,
        "calldata_copy_is_source_calldata_size",
    );
    context.build_conditional_branch(is_source_calldata_size, memory_zero_block, default_block);

    context.set_basic_block(default_block);
    let source = context.access_memory(
        combined_offset,
        AddressSpace::Parent,
        "calldata_copy_source_pointer",
    );
    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromParent,
        destination,
        source,
        size,
        "calldata_copy_memcpy_from_child",
    );
    context.build_unconditional_branch(join_block);

    context.set_basic_block(memory_zero_block);
    self::memory_zero(context, destination_offset, size)?;
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    Ok(None)
}

fn memory_zero<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    destination_offset: inkwell::values::IntValue<'ctx>,
    size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let condition_block = context.append_basic_block("calldata_memory_zero_condition");
    let body_block = context.append_basic_block("calldata_memory_zero_body");
    let increment_block = context.append_basic_block("calldata_memory_zero_increment");
    let join_block = context.append_basic_block("calldata_memory_zero_join");

    let index_pointer =
        context.build_alloca(context.field_type(), "calldata_memory_zero_index_pointer");
    context.build_store(index_pointer, context.field_const(0));
    context.build_unconditional_branch(condition_block);

    context.set_basic_block(condition_block);
    let index_value =
        context.build_load(index_pointer, "calldata_memory_zero_condition_index_value");
    let condition = context.builder().build_int_compare(
        inkwell::IntPredicate::ULT,
        index_value.into_int_value(),
        size,
        "calldata_memory_zero_condition_compared",
    );
    context.build_conditional_branch(condition, body_block, join_block);

    context.set_basic_block(body_block);
    let index_value = context.build_load(index_pointer, "calldata_memory_zero_body_index_value");
    let destination_offset = context.builder().build_int_add(
        destination_offset,
        index_value.into_int_value(),
        "calldata_memory_zero_body_destination_offset",
    );
    let destination_pointer = context.access_memory(
        destination_offset,
        AddressSpace::Heap,
        "calldata_memory_zero_body_destination_pointer",
    );
    context.build_store(destination_pointer, context.field_const(0));
    context.build_unconditional_branch(increment_block);

    context.set_basic_block(increment_block);
    let index_value =
        context.build_load(index_pointer, "calldata_memory_zero_increment_index_value");
    let index_value_incremented = context.builder().build_int_add(
        index_value.into_int_value(),
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "calldata_memory_zero_increment_index_value_incremented",
    );
    context.build_store(index_pointer, index_value_incremented);
    context.build_unconditional_branch(condition_block);

    context.set_basic_block(join_block);
    Ok(None)
}
