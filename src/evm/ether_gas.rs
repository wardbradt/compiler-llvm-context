//!
//! Translates the value and balance operations.
//!

use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the `gas` instruction.
///
pub fn gas<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    Ok(context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::ErgsLeft),
        &[],
        "ergs_left",
    ))
}

///
/// Translates the `value` instruction.
///
pub fn value<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    Ok(context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::GetU128),
        &[],
        "get_u128_value",
    ))
}

///
/// Translates the `balance` instructions.
///
pub fn balance<'ctx, D>(
    context: &mut Context<'ctx, D>,
    address: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_ETH_TOKEN.into()),
        "balanceOf(address)",
        vec![address],
    )
    .map(Some)
}
