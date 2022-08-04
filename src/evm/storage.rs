//!
//! Translates the contract storage operations.
//!

use inkwell::values::BasicValue;

use crate::context::Context;
use crate::Dependency;

///
/// Translates the contract storage load.
///
pub fn load<'ctx, D>(
    context: &mut Context<'ctx, D>,
    position: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let value = context
        .build_call(
            context.runtime.storage_load,
            &[position.as_basic_value_enum()],
            "storage_load",
        )
        .expect("Contract storage always returns a value");
    Ok(Some(value))
}

///
/// Translates the contract storage store.
///
/// Beware that the `position` and `value` arguments have different order in Yul and LLVM IR.
///
pub fn store<'ctx, D>(
    context: &mut Context<'ctx, D>,
    position: inkwell::values::IntValue<'ctx>,
    value: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    context.build_invoke(
        context.runtime.storage_store,
        &[value.as_basic_value_enum(), position.as_basic_value_enum()],
        "storage_store",
    );
    Ok(None)
}
