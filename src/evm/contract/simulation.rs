//!
//! Translates simulations of the Yul's external call instructions.
//!

use inkwell::values::BasicValue;

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
/// Generates a system call.
///
pub fn system_call<'ctx, D>(
    context: &mut Context<'ctx, D>,
    address: inkwell::values::IntValue<'ctx>,
    abi_data: inkwell::values::IntValue<'ctx>,
    is_delegate: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let join_block = context.append_basic_block("system_call_join_block");

    let system_far_call_block = context.append_basic_block("system_far_call_block");
    let system_delegate_call_block = context.append_basic_block("system_delegate_call_block");

    let result_pointer = context.build_alloca(context.field_type(), "system_call_result_pointer");
    context.build_store(result_pointer, context.field_const(0));

    let is_delegate_equals_zero = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        is_delegate,
        context.field_const(0),
        "system_call_is_delegate_equals_zero",
    );
    context.build_conditional_branch(
        is_delegate_equals_zero,
        system_far_call_block,
        system_delegate_call_block,
    );

    {
        context.set_basic_block(system_far_call_block);
        let result = super::call_system(
            context,
            context.runtime.far_call,
            address,
            abi_data,
            output_offset,
            output_length,
        )?;
        context.build_store(result_pointer, result);
        context.build_unconditional_branch(join_block);
    }

    {
        context.set_basic_block(system_delegate_call_block);
        let result = super::call_system(
            context,
            context.runtime.delegate_call,
            address,
            abi_data,
            output_offset,
            output_length,
        )?;
        context.build_store(result_pointer, result);
        context.build_unconditional_branch(join_block);
    }

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "system_call_result");
    Ok(result)
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
