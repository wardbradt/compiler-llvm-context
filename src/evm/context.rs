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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
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
        context.field_const_str(compiler_common::ADDRESS_SYSTEM_CONTEXT),
        "msize()",
        vec![],
    )
    .map(Some)
}
