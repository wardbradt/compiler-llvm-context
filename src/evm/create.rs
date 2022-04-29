//!
//! Translates the contract creation instructions.
//!

use inkwell::types::BasicType;
use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the contract `create` instruction.
///
pub fn create<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    create2(context, value, input_offset, input_length, None)
}

///
/// Translates the contract `create2` instruction.
///
pub fn create2<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
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
    let constructor_input_length = context.builder().build_int_sub(
        input_length,
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "create_input_length",
    );

    let address = call_precompile(
        context,
        hash.into_int_value(),
        salt.unwrap_or_else(|| context.field_const(0)),
        constructor_input_offset,
        constructor_input_length,
    )?;

    Ok(Some(address.as_basic_value_enum()))
}

///
/// Translates the contract hash instruction, which is actually used to set the hash of the contract
/// being created, or other related auxiliary data.
///
/// `dataoffset` in Yul, `PUSH [$]` in legacy assembly.
///
pub fn contract_hash<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    identifier: String,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
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
/// Translates the contract hash size instruction, which is actually used to set the hash of the
/// contract being created, or other related auxiliary data.
///
/// `datasize` in Yul, `PUSH #[$]` in legacy assembly.
///
pub fn contract_hash_size<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    identifier: String,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
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
    let address = context.field_const_str_hex(compiler_common::ABI_ADDRESS_KNOWN_CODE_FACTORY);

    let input_length_shifted = context.builder().build_left_shift(
        input_length,
        context.field_const(compiler_common::BITLENGTH_X64 as u64),
        "create_precompile_call_input_length_shifted",
    );
    let abi_data = context.builder().build_int_add(
        input_length_shifted,
        input_offset,
        "create_precompile_call_abi_data",
    );

    let result_type = context
        .structure_type(vec![
            context
                .integer_type(compiler_common::BITLENGTH_FIELD)
                .as_basic_type_enum(),
            context
                .integer_type(compiler_common::BITLENGTH_BOOLEAN)
                .as_basic_type_enum(),
        ])
        .as_basic_type_enum();
    let result_pointer =
        context.build_alloca(result_type, "contract_precompile_call_result_pointer");

    let result_pointer = context
        .build_invoke(
            context.runtime.far_call,
            &[
                address.as_basic_value_enum(),
                abi_data.as_basic_value_enum(),
                result_pointer.as_basic_value_enum(),
            ],
            "create_precompile_call_external",
        )
        .expect("Always returns a value");
    let result_address_pointer = unsafe {
        context.builder().build_gep(
            result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_zero(),
            ],
            "create_precompile_call_external_result_address_pointer",
        )
    };
    let result_address = context.build_load(
        result_address_pointer,
        "create_precompile_call_external_result_address",
    );

    Ok(result_address)
}
