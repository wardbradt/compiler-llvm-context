//!
//! Translates the contract creation instructions.
//!

use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::argument::Argument;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the contract `create` instruction.
///
pub fn create<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    create2(context, value, input_offset, input_size, None)
}

///
/// Translates the contract `create2` instruction.
///
pub fn create2<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_size: inkwell::values::IntValue<'ctx>,
    salt: Option<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::check_value_zero(context, value);

    let hash_pointer =
        context.access_memory(input_offset, AddressSpace::Heap, "create_hash_pointer");
    let hash = context.build_load(hash_pointer, "create_hash_value");

    let constructor_input_offset = context.builder().build_int_add(
        input_offset,
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "create_input_offset",
    );
    let constructor_input_size = context.builder().build_int_sub(
        input_size,
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "create_input_size",
    );

    let address = call_precompile(
        context,
        hash.into_int_value(),
        salt.unwrap_or_else(|| context.field_const(0)),
        constructor_input_offset,
        constructor_input_size,
    )?;

    Ok(Some(address.as_basic_value_enum()))
}

///
/// Calls the `create` precompile, which returns the newly deployed contract address.
///
fn call_precompile<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    hash: inkwell::values::IntValue<'ctx>,
    salt: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let call_definition = context.builder().build_left_shift(
        context.field_const_str(compiler_common::ABI_ADDRESS_CREATE),
        context.field_const((compiler_common::BITLENGTH_X32) as u64),
        "",
    );
    let result = context
        .build_invoke(
            context.runtime.far_call,
            &[
                call_definition.as_basic_value_enum(),
                hash.as_basic_value_enum(),
                salt.as_basic_value_enum(),
                input_offset.as_basic_value_enum(),
                input_length.as_basic_value_enum(),
            ],
            "create_precompile_call_external",
        )
        .expect("Always returns a value");

    let child_offset = context.builder().build_and(
        result.into_int_value(),
        context.field_const(compiler_common::BITLENGTH_X32 as u64),
        "create_precompile_child_offset",
    );
    let child_pointer = context.access_memory(
        child_offset,
        AddressSpace::Child,
        "create_precompile_child_pointer",
    );

    let result = context.build_load(child_pointer, "create_precompile_result");

    Ok(result)
}

///
/// Translates the `datasize` instruction, which is actually used to set the hash of the contract
/// being created, or other related auxiliary data.
///
pub fn datasize<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    mut arguments: [Argument<'ctx>; 1],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let identifier = arguments[0]
        .original
        .take()
        .ok_or_else(|| anyhow::anyhow!("`datasize` object identifier is missing"))?;

    let parent = context.module().get_name().to_str().expect("Always valid");

    if identifier.ends_with("_deployed") || identifier.as_str() == parent {
        return Ok(Some(context.field_const(0).as_basic_value_enum()));
    }

    Ok(Some(
        context
            .field_const(compiler_common::SIZE_FIELD as u64)
            .as_basic_value_enum(),
    ))
}

///
/// Translates the `dataoffset` instruction, which is actually used to set the hash of the contract
/// being created, or other related auxiliary data.
///
pub fn dataoffset<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    mut arguments: [Argument<'ctx>; 1],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let identifier = arguments[0]
        .original
        .take()
        .ok_or_else(|| anyhow::anyhow!("`dataoffset` object identifier is missing"))?;

    let parent = context.module().get_name().to_str().expect("Always valid");

    if identifier.ends_with("_deployed") || identifier.as_str() == parent {
        return Ok(Some(context.field_const(0).as_basic_value_enum()));
    }

    let hash_value = context
        .compile_dependency(identifier.as_str())
        .map(|hash| context.field_const_str(hash.as_str()))
        .map(inkwell::values::BasicValueEnum::IntValue)?;

    Ok(Some(hash_value))
}

///
/// Translates the `datacopy` instruction, which is actually used to set the hash of the contract
/// being created, or other related auxiliary data.
///
pub fn datacopy<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    arguments: [inkwell::values::BasicValueEnum<'ctx>; 3],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let pointer = context.access_memory(
        arguments[0].into_int_value(),
        AddressSpace::Heap,
        "datacopy_pointer",
    );
    context.build_store(pointer, arguments[1]);

    Ok(None)
}
