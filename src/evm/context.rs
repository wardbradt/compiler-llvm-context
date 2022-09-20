//!
//! Translates the context getter instructions.
//!

use crate::context::Context;
use crate::Dependency;

///
/// Translates the `gas_limit` instruction.
///
pub fn gas_limit<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "blockErgsLimit()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `gas_price` instruction.
///
pub fn gas_price<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "ergsPrice()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `tx.origin` instruction.
///
pub fn origin<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "origin()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `chain_id` instruction.
///
pub fn chain_id<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "chainId()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `block_number` instruction.
///
pub fn block_number<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "getBlockNumber()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `block_timestamp` instruction.
///
pub fn block_timestamp<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "getBlockTimestamp()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `block_hash` instruction.
///
pub fn block_hash<'ctx, D>(
    context: &mut Context<'ctx, D>,
    index: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "blockHash(uint256)",
        vec![index],
    )
    .map(Some)
}

///
/// Translates the `difficulty` instruction.
///
pub fn difficulty<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "difficulty()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `coinbase` instruction.
///
pub fn coinbase<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "coinbase()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `basefee` instruction.
///
pub fn basefee<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "baseFee()",
        vec![],
    )
    .map(Some)
}

///
/// Translates the `memory_size` instruction.
///
pub fn msize<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    crate::evm::contract::request::request(
        context,
        context.field_const(compiler_common::ADDRESS_SYSTEM_CONTEXT.into()),
        "msize()",
        vec![],
    )
    .map(Some)
}
