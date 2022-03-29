//!
//! Translates a contract call.
//!

use inkwell::types::BasicType;
use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::argument::Argument;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates a contract call.
///
#[allow(clippy::too_many_arguments)]
pub fn call<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    function: inkwell::values::FunctionValue<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
    value: Option<inkwell::values::IntValue<'ctx>>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_size: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    if let Some(value) = value {
        crate::evm::check_value_zero(context, value);
    }

    let identity_block = context.append_basic_block("contract_call_identity_block");
    let ordinary_block = context.append_basic_block("contract_call_ordinary_block");
    let join_block = context.append_basic_block("contract_call_join_block");

    let result_pointer = context.build_alloca(context.field_type(), "contract_call_result_pointer");
    context.build_store(result_pointer, context.field_const(0));

    let is_address_identity = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        address,
        context.field_const_str(compiler_common::ABI_ADDRESS_IDENTITY),
        "contract_call_is_address_identity",
    );
    context.build_conditional_branch(is_address_identity, identity_block, ordinary_block);

    context.set_basic_block(identity_block);
    let result = call_identity(context, output_offset, input_offset, output_size)?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(ordinary_block);
    let result = call_ordinary(
        context,
        function,
        address,
        input_offset,
        input_size,
        output_offset,
        output_size,
    )?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "contract_call_result");

    Ok(Some(result))
}

///
/// Translates a linker symbol.
///
pub fn linker_symbol<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    mut arguments: [Argument<'ctx>; 1],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let path = arguments[0]
        .original
        .take()
        .ok_or_else(|| anyhow::anyhow!("Linker symbol literal is missing"))?;

    Ok(Some(
        context
            .resolve_library(path.as_str())?
            .as_basic_value_enum(),
    ))
}

///
/// Generates an ordinary contract call
///
fn call_ordinary<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    function: inkwell::values::FunctionValue<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_size: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let input_size_shifted = context.builder().build_left_shift(
        input_size,
        context.field_const(compiler_common::BITLENGTH_X32 as u64),
        "contract_call_input_size_shifted",
    );
    let abi_data =
        context
            .builder()
            .build_int_add(input_size_shifted, input_offset, "contract_call_abi_data");

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
    let result_pointer = context.build_alloca(result_type, "contract_call_result_pointer");

    let result_pointer = context
        .build_invoke(
            function,
            &[
                abi_data.as_basic_value_enum(),
                address.as_basic_value_enum(),
                result_pointer.as_basic_value_enum(),
            ],
            "contract_call_external",
        )
        .expect("IntrinsicFunction always returns a flag");
    let result_abi_data_pointer = context
        .builder()
        .build_struct_gep(
            result_pointer.into_pointer_value(),
            0,
            "contract_call_external_result_abi_data_pointer",
        )
        .expect("Always valid");
    let result_abi_data = context.build_load(
        result_abi_data_pointer,
        "contract_call_external_result_abi_data",
    );
    let result_status_code_pointer = context
        .builder()
        .build_struct_gep(
            result_pointer.into_pointer_value(),
            0,
            "contract_call_external_result_status_code_pointer",
        )
        .expect("Always valid");
    let result_status_code = context.build_load(
        result_status_code_pointer,
        "contract_call_external_result_status_code",
    );

    let child_data_offset = context.builder().build_and(
        result_abi_data.into_int_value(),
        context.field_const(compiler_common::BITLENGTH_X32 as u64),
        "contract_call_child_data_offset",
    );
    let source = context.access_memory(
        child_data_offset,
        AddressSpace::Child,
        "contract_call_source",
    );

    let destination = context.access_memory(
        output_offset,
        AddressSpace::Heap,
        "contract_call_destination",
    );

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromChild,
        destination,
        source,
        output_size,
        "contract_call_memcpy_from_child",
    );

    Ok(result_status_code)
}

///
/// Generates a memcopy call for the Identity precompile.
///
fn call_identity<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    destination: inkwell::values::IntValue<'ctx>,
    source: inkwell::values::IntValue<'ctx>,
    size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let destination = context.access_memory(
        destination,
        AddressSpace::Heap,
        "contract_call_identity_destination",
    );
    let source = context.access_memory(source, AddressSpace::Heap, "contract_call_identity_source");

    context.build_memcpy(
        IntrinsicFunction::MemoryCopy,
        destination,
        source,
        size,
        "contract_call_memcpy_to_child",
    );

    Ok(context.field_const(1).as_basic_value_enum())
}
