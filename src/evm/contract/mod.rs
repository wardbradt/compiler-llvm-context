//!
//! Translates a contract call.
//!

pub mod request;
pub mod simulation;

use inkwell::values::BasicValue;
use num::BigUint;

use crate::context::address_space::AddressSpace;
use crate::context::argument::Argument;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates a contract call.
///
#[allow(clippy::too_many_arguments)]
pub fn call<'ctx, D>(
    context: &mut Context<'ctx, D>,
    function: inkwell::values::FunctionValue<'ctx>,
    gas: inkwell::values::IntValue<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
    value: Option<inkwell::values::IntValue<'ctx>>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_length: inkwell::values::IntValue<'ctx>,
    simulation: Option<BigUint>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    if let Some(simulation) = simulation {
        if crate::evm::parse_address(compiler_common::ADDRESS_TO_L1) == simulation {
            let is_first = gas;
            let in_0 = value.unwrap_or_else(|| context.field_const(0));
            let in_1 = input_offset;
            return simulation::to_l1(context, is_first, in_0, in_1).map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_CODE_ADDRESS) == simulation {
            return simulation::code_source(context).map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_PRECOMPILE) == simulation {
            let in_0 = gas;
            let ergs_left = input_offset;

            return simulation::precompile(context, in_0, ergs_left).map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_META) == simulation {
            return simulation::meta(context).map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_MIMIC_CALL) == simulation {
            let address = gas;
            let mimic = value.unwrap_or_else(|| context.field_const(0));
            let abi_data = input_offset;

            return simulation::mimic_call(context, address, mimic, abi_data).map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_RAW_FAR_CALL) == simulation {
            let address = gas;
            let abi_data = input_offset;

            return simulation::raw_far_call(
                context,
                function,
                address,
                abi_data,
                output_offset,
                output_length,
            )
            .map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_SYSTEM_CALL) == simulation {
            let address = gas;
            let abi_data = input_offset;
            let extra_value_1 = value.unwrap_or_else(|| context.field_const(0));
            let extra_value_2 = input_length;

            return simulation::system_call(
                context,
                address,
                abi_data,
                output_offset,
                output_length,
                extra_value_1,
                extra_value_2,
            )
            .map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_SET_CONTEXT_VALUE_CALL)
            == simulation
        {
            let value = value.unwrap_or_else(|| context.field_const(0));

            return simulation::set_context_value(context, value).map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_SET_PUBDATA_PRICE)
            == simulation
        {
            let price = gas;

            return simulation::set_pubdata_price(context, price).map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_INCREMENT_TX_COUNTER)
            == simulation
        {
            return simulation::increment_tx_counter(context).map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_GET_GLOBAL_PTR_CALLDATA)
            == simulation
        {
            return simulation::get_global(
                context,
                BigUint::from(crate::r#const::GLOBAL_INDEX_CALLDATA_ABI),
            )
            .map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_GET_GLOBAL_CALL_FLAGS)
            == simulation
        {
            return simulation::get_global(
                context,
                BigUint::from(crate::r#const::GLOBAL_INDEX_CALL_FLAGS),
            )
            .map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_GET_GLOBAL_EXTRA_ABI_DATA_1)
            == simulation
        {
            return simulation::get_global(
                context,
                BigUint::from(crate::r#const::GLOBAL_INDEX_EXTRA_ABI_DATA_1),
            )
            .map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_GET_GLOBAL_EXTRA_ABI_DATA_2)
            == simulation
        {
            return simulation::get_global(
                context,
                BigUint::from(crate::r#const::GLOBAL_INDEX_EXTRA_ABI_DATA_2),
            )
            .map(Some);
        } else if crate::evm::parse_address(compiler_common::ADDRESS_GET_GLOBAL_PTR_RETURN_DATA)
            == simulation
        {
            return simulation::get_global(
                context,
                BigUint::from(crate::r#const::GLOBAL_INDEX_RETURN_DATA_ABI),
            )
            .map(Some);
        }
    }

    let identity_block = context.append_basic_block("contract_call_identity_block");
    let ordinary_block = context.append_basic_block("contract_call_ordinary_block");
    let join_block = context.append_basic_block("contract_call_join_block");

    let result_pointer = context.build_alloca(context.field_type(), "contract_call_result_pointer");
    context.build_store(result_pointer, context.field_const(0));

    context.builder().build_switch(
        address,
        ordinary_block,
        &[(
            context.field_const_str(compiler_common::ADDRESS_IDENTITY),
            identity_block,
        )],
    );

    {
        context.set_basic_block(identity_block);
        let result = call_identity(context, output_offset, input_offset, output_length)?;
        context.build_store(result_pointer, result);
        context.build_unconditional_branch(join_block);
    }

    context.set_basic_block(ordinary_block);
    let result = if let Some(value) = value {
        call_default_wrapped(
            context,
            function,
            gas,
            value,
            address,
            input_offset,
            input_length,
            output_offset,
            output_length,
        )
    } else {
        call_default(
            context,
            function,
            gas,
            address,
            input_offset,
            input_length,
            output_offset,
            output_length,
        )
    }?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "contract_call_result");

    Ok(Some(result))
}

///
/// Translates a linker symbol.
///
pub fn linker_symbol<'ctx, D>(
    context: &mut Context<'ctx, D>,
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
/// Generates an ABI data for a default call.
///
pub fn abi_data<'ctx, D>(
    context: &mut Context<'ctx, D>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    gas: inkwell::values::IntValue<'ctx>,
    address_space: AddressSpace,
    is_system_call: bool,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let input_offset_truncated = context.builder().build_and(
        input_offset,
        context.field_const(u32::MAX as u64),
        "abi_data_input_offset_truncated",
    );
    let input_length_truncated = context.builder().build_and(
        input_length,
        context.field_const(u32::MAX as u64),
        "abi_data_input_length_truncated",
    );
    let gas_truncated = context.builder().build_and(
        gas,
        context.field_const(u32::MAX as u64),
        "abi_data_gas_truncated",
    );

    let input_offset_shifted = context.builder().build_left_shift(
        input_offset_truncated,
        context.field_const((compiler_common::BITLENGTH_X32 * 2) as u64),
        "abi_data_input_offset_shifted",
    );
    let input_length_shifted = context.builder().build_left_shift(
        input_length_truncated,
        context.field_const((compiler_common::BITLENGTH_X32 * 3) as u64),
        "abi_data_input_length_shifted",
    );
    let gas_shifted = context.builder().build_left_shift(
        gas_truncated,
        context.field_const((compiler_common::BITLENGTH_X32 * 6) as u64),
        "abi_data_gas_shifted",
    );

    let mut abi_data = context.builder().build_int_add(
        input_offset_shifted,
        input_length_shifted,
        "abi_data_offset_and_length",
    );
    abi_data = context
        .builder()
        .build_int_add(abi_data, gas_shifted, "abi_data_add_gas");
    if let AddressSpace::HeapAuxiliary = address_space {
        let auxiliary_heap_marker_shifted = context.builder().build_left_shift(
            context.field_const(zkevm_opcode_defs::FarCallForwardPageType::UseAuxHeap as u64),
            context.field_const(
                (compiler_common::BITLENGTH_X32 * 7 + compiler_common::BITLENGTH_BYTE) as u64,
            ),
            "abi_data_auxiliary_heap_marker_shifted",
        );
        abi_data = context.builder().build_int_add(
            abi_data,
            auxiliary_heap_marker_shifted,
            "abi_data_add_heap_auxiliary_marker",
        );
    }
    if is_system_call {
        let auxiliary_heap_marker_shifted = context.builder().build_left_shift(
            context.field_const(zkevm_opcode_defs::FarCallForwardPageType::UseAuxHeap as u64),
            context.field_const(
                ((compiler_common::BITLENGTH_X32 * 7) + (compiler_common::BITLENGTH_BYTE * 3))
                    as u64,
            ),
            "abi_data_system_call_marker_shifted",
        );
        abi_data = context.builder().build_int_add(
            abi_data,
            auxiliary_heap_marker_shifted,
            "abi_data_add_system_call_marker",
        );
    }

    Ok(abi_data.as_basic_value_enum())
}

///
/// The default call wrapper, which makes the necessary ABI tweaks if `msg.value` is not zero.
///
#[allow(clippy::too_many_arguments)]
fn call_default_wrapped<'ctx, D>(
    context: &mut Context<'ctx, D>,
    function: inkwell::values::FunctionValue<'ctx>,
    gas: inkwell::values::IntValue<'ctx>,
    value: inkwell::values::IntValue<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let value_zero_block = context.append_basic_block("contract_call_value_zero_block");
    let value_non_zero_block = context.append_basic_block("contract_call_value_non_zero_block");
    let value_join_block = context.append_basic_block("contract_call_value_join_block");

    let result_pointer =
        context.build_alloca(context.field_type(), "contract_call_address_result_pointer");
    context.build_store(result_pointer, context.field_const(0));
    let is_value_zero = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        value,
        context.field_const(0),
        "contract_call_is_value_zero",
    );
    context.build_conditional_branch(is_value_zero, value_zero_block, value_non_zero_block);

    context.set_basic_block(value_non_zero_block);
    let abi_data = abi_data(
        context,
        input_offset,
        input_length,
        gas,
        AddressSpace::Heap,
        true,
    )?
    .into_int_value();
    let result = call_system(
        context,
        context.field_const_str_hex(compiler_common::ADDRESS_MSG_VALUE),
        abi_data,
        output_offset,
        output_length,
        value,
        address,
    )?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(value_join_block);

    context.set_basic_block(value_zero_block);
    let result = call_default(
        context,
        function,
        gas,
        address,
        input_offset,
        input_length,
        output_offset,
        output_length,
    )?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(value_join_block);

    context.set_basic_block(value_join_block);
    let address = context.build_load(result_pointer, "contract_call_address_result");
    Ok(address)
}

///
/// Generates a default contract call.
///
#[allow(clippy::too_many_arguments)]
fn call_default<'ctx, D>(
    context: &mut Context<'ctx, D>,
    function: inkwell::values::FunctionValue<'ctx>,
    gas: inkwell::values::IntValue<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let join_block = context.append_basic_block("contract_call_join_block");

    let status_code_result_pointer = context.build_alloca(
        context.field_type(),
        "contract_call_result_status_code_pointer",
    );
    context.build_store(status_code_result_pointer, context.field_const(0));

    let abi_data = abi_data(
        context,
        input_offset,
        input_length,
        gas,
        AddressSpace::Heap,
        false,
    )?
    .into_int_value();

    let result_pointer = context
        .build_invoke_far_call(
            function,
            vec![
                abi_data.as_basic_value_enum(),
                address.as_basic_value_enum(),
            ],
            "contract_call_external",
        )
        .expect("IntrinsicFunction always returns a flag");

    let result_abi_data_pointer = unsafe {
        context.builder().build_gep(
            result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_zero(),
            ],
            "contract_call_external_result_abi_data_pointer",
        )
    };
    let result_abi_data = context.build_load(
        result_abi_data_pointer,
        "contract_call_external_result_abi_data",
    );
    let result_abi_data_casted = context.builder().build_pointer_cast(
        result_abi_data.into_pointer_value(),
        context.field_type().ptr_type(AddressSpace::Generic.into()),
        "contract_call_external_result_abi_data_casted",
    );

    let result_status_code_pointer = unsafe {
        context.builder().build_gep(
            result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_int(1, false),
            ],
            "contract_call_external_result_status_code_pointer",
        )
    };
    let result_status_code_boolean = context.build_load(
        result_status_code_pointer,
        "contract_call_external_result_status_code_boolean",
    );
    let result_status_code = context.builder().build_int_z_extend_or_bit_cast(
        result_status_code_boolean.into_int_value(),
        context.field_type(),
        "contract_call_external_result_status_code",
    );
    context.build_store(status_code_result_pointer, result_status_code);

    let source = result_abi_data_casted;

    let destination = context.access_memory(
        output_offset,
        AddressSpace::Heap,
        "contract_call_destination",
    );

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromGeneric,
        destination,
        source,
        output_length,
        "contract_call_memcpy_from_child",
    );

    context.write_abi_return_data(result_abi_data.into_pointer_value());
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let status_code_result =
        context.build_load(status_code_result_pointer, "contract_call_status_code");
    Ok(status_code_result)
}

///
/// Generates a memcopy call for the `Identity` precompile.
///
fn call_identity<'ctx, D>(
    context: &mut Context<'ctx, D>,
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

///
/// Generates a mimic call.
///
fn call_mimic<'ctx, D>(
    context: &mut Context<'ctx, D>,
    function: inkwell::values::FunctionValue<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
    mimic: inkwell::values::IntValue<'ctx>,
    abi_data: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let join_block = context.append_basic_block("mimic_call_join_block");

    let status_code_result_pointer = context.build_alloca(
        context.field_type(),
        "mimic_call_result_status_code_pointer",
    );
    context.build_store(status_code_result_pointer, context.field_const(0));

    let far_call_result_pointer = context
        .build_invoke_far_call(
            function,
            vec![
                abi_data.as_basic_value_enum(),
                address.as_basic_value_enum(),
                mimic.as_basic_value_enum(),
            ],
            "mimic_call_external",
        )
        .expect("IntrinsicFunction always returns a flag");

    let result_abi_data_pointer = unsafe {
        context.builder().build_gep(
            far_call_result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_zero(),
            ],
            "mimic_call_external_result_abi_data_pointer",
        )
    };
    let result_abi_data = context.build_load(
        result_abi_data_pointer,
        "mimic_call_external_result_abi_data",
    );

    let result_status_code_pointer = unsafe {
        context.builder().build_gep(
            far_call_result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_int(1, false),
            ],
            "mimic_call_external_result_status_code_pointer",
        )
    };
    let result_status_code_boolean = context.build_load(
        result_status_code_pointer,
        "mimic_call_external_result_status_code_boolean",
    );
    let result_status_code = context.builder().build_int_z_extend_or_bit_cast(
        result_status_code_boolean.into_int_value(),
        context.field_type(),
        "mimic_call_external_result_status_code",
    );
    context.build_store(status_code_result_pointer, result_status_code);

    context.write_abi_return_data(result_abi_data.into_pointer_value());
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let status_code_result =
        context.build_load(status_code_result_pointer, "mimic_call_status_code");
    Ok(status_code_result)
}

///
/// Generates a raw far call.
///
#[allow(clippy::too_many_arguments)]
fn call_far_raw<'ctx, D>(
    context: &mut Context<'ctx, D>,
    function: inkwell::values::FunctionValue<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
    abi_data: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let join_block = context.append_basic_block("system_far_call_join_block");

    let status_code_result_pointer = context.build_alloca(
        context.field_type(),
        "system_far_call_result_status_code_pointer",
    );
    context.build_store(status_code_result_pointer, context.field_const(0));

    let far_call_result_pointer = context
        .build_invoke_far_call(
            function,
            vec![
                abi_data.as_basic_value_enum(),
                address.as_basic_value_enum(),
            ],
            "system_far_call_external",
        )
        .expect("IntrinsicFunction always returns a flag");

    let result_abi_data_pointer = unsafe {
        context.builder().build_gep(
            far_call_result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_zero(),
            ],
            "system_far_call_external_result_abi_data_pointer",
        )
    };
    let result_abi_data = context.build_load(
        result_abi_data_pointer,
        "system_far_call_external_result_abi_data",
    );
    let result_abi_data_casted = context.builder().build_pointer_cast(
        result_abi_data.into_pointer_value(),
        context.field_type().ptr_type(AddressSpace::Generic.into()),
        "system_far_call_external_result_abi_data_casted",
    );

    let result_status_code_pointer = unsafe {
        context.builder().build_gep(
            far_call_result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_int(1, false),
            ],
            "system_far_call_external_result_status_code_pointer",
        )
    };
    let result_status_code_boolean = context.build_load(
        result_status_code_pointer,
        "system_far_call_external_result_status_code_boolean",
    );
    let result_status_code = context.builder().build_int_z_extend_or_bit_cast(
        result_status_code_boolean.into_int_value(),
        context.field_type(),
        "system_far_call_external_result_status_code",
    );
    context.build_store(status_code_result_pointer, result_status_code);

    let source = result_abi_data_casted;

    let destination = context.access_memory(
        output_offset,
        AddressSpace::Heap,
        "system_far_call_destination",
    );

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromGeneric,
        destination,
        source,
        output_length,
        "system_far_call_memcpy_from_child",
    );

    context.write_abi_return_data(result_abi_data.into_pointer_value());
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let status_code_result =
        context.build_load(status_code_result_pointer, "system_call_status_code");
    Ok(status_code_result)
}

///
/// Generates a system call.
///
#[allow(clippy::too_many_arguments)]
fn call_system<'ctx, D>(
    context: &mut Context<'ctx, D>,
    address: inkwell::values::IntValue<'ctx>,
    abi_data: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_length: inkwell::values::IntValue<'ctx>,
    extra_value_1: inkwell::values::IntValue<'ctx>,
    extra_value_2: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let join_block = context.append_basic_block("system_far_call_join_block");

    let status_code_result_pointer = context.build_alloca(
        context.field_type(),
        "system_far_call_result_status_code_pointer",
    );
    context.build_store(status_code_result_pointer, context.field_const(0));

    let far_call_result_pointer = context
        .build_invoke_far_call(
            context.runtime.system_call,
            vec![
                abi_data.as_basic_value_enum(),
                address.as_basic_value_enum(),
                extra_value_1.as_basic_value_enum(),
                extra_value_2.as_basic_value_enum(),
            ],
            "system_far_call_external",
        )
        .expect("IntrinsicFunction always returns a flag");

    let result_abi_data_pointer = unsafe {
        context.builder().build_gep(
            far_call_result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_zero(),
            ],
            "system_far_call_external_result_abi_data_pointer",
        )
    };
    let result_abi_data = context.build_load(
        result_abi_data_pointer,
        "system_far_call_external_result_abi_data",
    );
    let result_abi_data_casted = context.builder().build_pointer_cast(
        result_abi_data.into_pointer_value(),
        context.field_type().ptr_type(AddressSpace::Generic.into()),
        "system_far_call_external_result_abi_data_casted",
    );

    let result_status_code_pointer = unsafe {
        context.builder().build_gep(
            far_call_result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_int(1, false),
            ],
            "system_far_call_external_result_status_code_pointer",
        )
    };
    let result_status_code_boolean = context.build_load(
        result_status_code_pointer,
        "system_far_call_external_result_status_code_boolean",
    );
    let result_status_code = context.builder().build_int_z_extend_or_bit_cast(
        result_status_code_boolean.into_int_value(),
        context.field_type(),
        "system_far_call_external_result_status_code",
    );
    context.build_store(status_code_result_pointer, result_status_code);

    let source = result_abi_data_casted;

    let destination = context.access_memory(
        output_offset,
        AddressSpace::Heap,
        "system_far_call_destination",
    );

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromGeneric,
        destination,
        source,
        output_length,
        "system_far_call_memcpy_from_child",
    );

    context.write_abi_return_data(result_abi_data.into_pointer_value());
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let status_code_result =
        context.build_load(status_code_result_pointer, "system_call_status_code");
    Ok(status_code_result)
}
