//!
//! Translates the mathematics operation.
//!

use crate::context::Context;
use crate::Dependency;
use inkwell::values::BasicValue;

///
/// Translates the modular addition operation.
///
pub fn add_mod<'ctx, D>(
    context: &mut Context<'ctx, D>,
    operand_1: inkwell::values::IntValue<'ctx>,
    operand_2: inkwell::values::IntValue<'ctx>,
    modulo: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    Ok(context.build_call(
        context.runtime.add_mod,
        &[
            operand_1.as_basic_value_enum(),
            operand_2.as_basic_value_enum(),
            modulo.as_basic_value_enum(),
        ],
        "add_mod_call",
    ))
}

///
/// Translates the modular multiplication operation.
///
pub fn mul_mod<'ctx, D>(
    context: &mut Context<'ctx, D>,
    operand_1: inkwell::values::IntValue<'ctx>,
    operand_2: inkwell::values::IntValue<'ctx>,
    modulo: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    Ok(context.build_call(
        context.runtime.mul_mod,
        &[
            operand_1.as_basic_value_enum(),
            operand_2.as_basic_value_enum(),
            modulo.as_basic_value_enum(),
        ],
        "mul_mod_call",
    ))
}

///
/// Translates the exponent operation.
///
pub fn exponent<'ctx, D>(
    context: &mut Context<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
    exponent: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let condition_block = context.append_basic_block("exponent_loop_condition");
    let body_block = context.append_basic_block("exponent_loop_body");
    let multiplying_block = context.append_basic_block("exponent_loop_multiplying");
    let iterating_block = context.append_basic_block("exponent_loop_iterating");
    let join_block = context.append_basic_block("exponent_loop_join");

    let factor_pointer = context.build_alloca(context.field_type(), "exponent_factor");
    context.build_store(factor_pointer, value);
    let power_pointer = context.build_alloca(context.field_type(), "exponent_loop_power_pointer");
    context.build_store(power_pointer, exponent);
    let result_pointer = context.build_alloca(context.field_type(), "exponent_result");
    context.build_store(result_pointer, context.field_const(1));
    context.build_unconditional_branch(condition_block);

    context.set_basic_block(condition_block);
    let power_value = context
        .build_load(power_pointer, "exponent_loop_power_value_condition")
        .into_int_value();
    let condition = context.builder().build_int_compare(
        inkwell::IntPredicate::UGT,
        power_value,
        context.field_const(0),
        "exponent_loop_is_power_value_non_zero",
    );
    context.build_conditional_branch(condition, body_block, join_block);

    context.set_basic_block(iterating_block);
    let factor_value = context
        .build_load(factor_pointer, "exponent_loop_factor_value_to_square")
        .into_int_value();
    let factor_value_squared = context.builder().build_int_mul(
        factor_value,
        factor_value,
        "exponent_loop_factor_value_squared",
    );
    context.build_store(factor_pointer, factor_value_squared);
    let power_value = context
        .build_load(power_pointer, "exponent_loop_power_value_to_half")
        .into_int_value();
    let power_value_halved = context.builder().build_int_unsigned_div(
        power_value,
        context.field_const(2),
        "exponent_loop_power_value_halved",
    );
    context.build_store(power_pointer, power_value_halved);
    context.build_unconditional_branch(condition_block);

    context.set_basic_block(body_block);
    let power_value = context
        .build_load(power_pointer, "exponent_loop_power_value_body")
        .into_int_value();
    let power_lowest_bit = context.builder().build_int_truncate_or_bit_cast(
        power_value,
        context.integer_type(compiler_common::BITLENGTH_BOOLEAN),
        "exponent_loop_power_body_lowest_bit",
    );
    context.build_conditional_branch(power_lowest_bit, multiplying_block, iterating_block);

    context.set_basic_block(multiplying_block);
    let intermediate = context
        .build_load(result_pointer, "exponent_loop_intermediate_result")
        .into_int_value();
    let factor_value = context
        .build_load(factor_pointer, "exponent_loop_intermediate_factor_value")
        .into_int_value();
    let result = context.builder().build_int_mul(
        intermediate,
        factor_value,
        "exponent_loop_intermediate_result_multiplied",
    );
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(iterating_block);

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "exponent_result");

    Ok(Some(result))
}

///
/// Translates the sign extension operation.
///
pub fn sign_extend<'ctx, D>(
    context: &mut Context<'ctx, D>,
    bytes: inkwell::values::IntValue<'ctx>,
    value: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    Ok(context.build_call(
        context.runtime.sign_extend,
        &[bytes.as_basic_value_enum(), value.as_basic_value_enum()],
        "sign_extend_call",
    ))
}
