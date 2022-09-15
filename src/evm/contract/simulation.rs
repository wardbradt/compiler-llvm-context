//!
//! Translates simulations of the Yul's external call instructions.
//!

use inkwell::values::BasicValue;
use num::BigUint;
use num::ToPrimitive;

use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Generates a call to L1.
///
pub fn to_l1<'ctx, D>(
    context: &mut Context<'ctx, D>,
    is_first: inkwell::values::IntValue<'ctx>,
    in_0: inkwell::values::IntValue<'ctx>,
    in_1: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let join_block = context.append_basic_block("contract_call_toL1_join_block");

    let contract_call_tol1_is_first_block =
        context.append_basic_block("contract_call_toL1_is_first_block");
    let contract_call_tol1_is_not_first_block =
        context.append_basic_block("contract_call_toL1_is_not_first_block");

    let is_first_equals_zero = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        is_first,
        context.field_const(0),
        "contract_call_toL1_is_first_equals_zero",
    );
    context.build_conditional_branch(
        is_first_equals_zero,
        contract_call_tol1_is_not_first_block,
        contract_call_tol1_is_first_block,
    );

    {
        context.set_basic_block(contract_call_tol1_is_not_first_block);
        let is_first = context.field_const(0);
        context.build_call(
            context.get_intrinsic_function(IntrinsicFunction::ToL1),
            &[
                in_0.as_basic_value_enum(),
                in_1.as_basic_value_enum(),
                is_first.as_basic_value_enum(),
            ],
            "contract_call_simulation_tol1",
        );
        context.build_unconditional_branch(join_block);
    }

    {
        context.set_basic_block(contract_call_tol1_is_first_block);
        let is_first = context.field_const(1);
        context.build_call(
            context.get_intrinsic_function(IntrinsicFunction::ToL1),
            &[
                in_0.as_basic_value_enum(),
                in_1.as_basic_value_enum(),
                is_first.as_basic_value_enum(),
            ],
            "contract_call_simulation_tol1",
        );
        context.build_unconditional_branch(join_block);
    }

    context.set_basic_block(join_block);
    Ok(context.field_const(1).as_basic_value_enum())
}

///
/// Generates a `code source` call.
///
pub fn code_source<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let result = context
        .build_call(
            context.get_intrinsic_function(IntrinsicFunction::CodeSource),
            &[],
            "contract_call_simulation_code_source",
        )
        .expect("Always exists");
    Ok(result)
}

///
/// Generates a precompile call.
///
pub fn precompile<'ctx, D>(
    context: &mut Context<'ctx, D>,
    in_0: inkwell::values::IntValue<'ctx>,
    ergs_left: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let result = context
        .build_call(
            context.get_intrinsic_function(IntrinsicFunction::Precompile),
            &[in_0.as_basic_value_enum(), ergs_left.as_basic_value_enum()],
            "contract_call_simulation_precompile",
        )
        .expect("Always exists");
    Ok(result)
}

///
/// Generates a `meta` call.
///
pub fn meta<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let result = context
        .build_call(
            context.get_intrinsic_function(IntrinsicFunction::Meta),
            &[],
            "contract_call_simulation_meta",
        )
        .expect("Always exists");
    Ok(result)
}

///
/// Generates a mimic call.
///
pub fn mimic_call<'ctx, D>(
    context: &mut Context<'ctx, D>,
    address: inkwell::values::IntValue<'ctx>,
    mimic: inkwell::values::IntValue<'ctx>,
    abi_data: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    super::call_mimic(
        context,
        context.runtime.mimic_call,
        address,
        mimic,
        abi_data,
    )
}

///
/// Generates a raw far call.
///
pub fn raw_far_call<'ctx, D>(
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
    super::call_far_raw(
        context,
        function,
        address,
        abi_data,
        output_offset,
        output_length,
    )
}

///
/// Generates a system call.
///
#[allow(clippy::too_many_arguments)]
pub fn system_call<'ctx, D>(
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
    super::call_system(
        context,
        address,
        abi_data,
        output_offset,
        output_length,
        extra_value_1,
        extra_value_2,
    )
}

///
/// Generates a `u128` context value setter call.
///
pub fn set_context_value<'ctx, D>(
    context: &mut Context<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::SetU128),
        &[value.as_basic_value_enum()],
        "contract_call_simulation_set_context_value",
    );
    Ok(context.field_const(1).as_basic_value_enum())
}

///
/// Generates a public data price setter call.
///
pub fn set_pubdata_price<'ctx, D>(
    context: &mut Context<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::SetPubdataPrice),
        &[value.as_basic_value_enum()],
        "contract_call_simulation_set_pubdata_price",
    );
    Ok(context.field_const(1).as_basic_value_enum())
}

///
/// Generates a transaction counter increment call.
///
pub fn increment_tx_counter<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::IncrementTxCounter),
        &[],
        "contract_call_simulation_increment_tx_counter",
    );
    Ok(context.field_const(1).as_basic_value_enum())
}

///
/// Generates an extra ABI data getter call.
///
pub fn get_global<'ctx, D>(
    context: &mut Context<'ctx, D>,
    index: BigUint,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    match index.to_usize() {
        Some(crate::r#const::GLOBAL_INDEX_CALLDATA_ABI) => {
            let pointer = context.get_global(crate::r#const::GLOBAL_CALLDATA_ABI)?;
            let value = context.builder().build_ptr_to_int(
                pointer.into_pointer_value(),
                context.field_type(),
                "calldata_abi_integer",
            );
            Ok(value.as_basic_value_enum())
        }
        Some(crate::r#const::GLOBAL_INDEX_CALL_FLAGS) => {
            context.get_global(crate::r#const::GLOBAL_CALL_FLAGS)
        }
        Some(
            index @ crate::r#const::GLOBAL_INDEX_EXTRA_ABI_DATA_1
            | index @ crate::r#const::GLOBAL_INDEX_EXTRA_ABI_DATA_2,
        ) => {
            let extra_abi_data_pointer =
                context.get_global_ptr(crate::r#const::GLOBAL_EXTRA_ABI_DATA)?;
            let extra_abi_data_index =
                context.field_const((index - crate::r#const::EXTRA_ABI_DATA_SIZE) as u64);
            let extra_abi_data_element_pointer = unsafe {
                context.builder().build_gep(
                    extra_abi_data_pointer,
                    &[context.field_const(0), extra_abi_data_index],
                    "extra_abi_data_element_pointer",
                )
            };
            let extra_abi_data_element = context.build_load(
                extra_abi_data_element_pointer,
                "extra_abi_data_element_value",
            );
            Ok(extra_abi_data_element)
        }
        Some(crate::r#const::GLOBAL_INDEX_RETURN_DATA_ABI) => {
            let pointer = context.get_global(crate::r#const::GLOBAL_RETURN_DATA_ABI)?;
            let value = context.builder().build_ptr_to_int(
                pointer.into_pointer_value(),
                context.field_type(),
                "return_data_abi_integer",
            );
            Ok(value.as_basic_value_enum())
        }
        _ => anyhow::bail!(
            "Global variable `{}` is unknown to the call-simulation access",
            index
        ),
    }
}
