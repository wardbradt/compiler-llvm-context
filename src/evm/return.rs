//!
//! Translates the transaction return operations.
//!

use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the normal return.
///
pub fn r#return<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    arguments: [inkwell::values::BasicValueEnum<'ctx>; 2],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let offset = arguments[0].into_int_value();
    let size = arguments[1].into_int_value();

    context.build_exit(IntrinsicFunction::Return, offset, size);
    Ok(None)
}

///
/// Translates the revert.
///
pub fn revert<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    arguments: [inkwell::values::BasicValueEnum<'ctx>; 2],
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let offset = arguments[0].into_int_value();
    let size = arguments[1].into_int_value();

    context.build_exit(IntrinsicFunction::Revert, offset, size);
    Ok(None)
}

///
/// Translates the stop.
///
pub fn stop<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    context.build_exit(
        IntrinsicFunction::Return,
        context.field_const(0),
        context.field_const(0),
    );
    Ok(None)
}

///
/// Translates the invalid.
///
pub fn invalid<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    context.build_exit(
        IntrinsicFunction::Revert,
        context.field_const(0),
        context.field_const(0),
    );
    Ok(None)
}
