//!
//! Translates the transaction return operations.
//!

use crate::context::address_space::AddressSpace;
use crate::context::function::runtime::Runtime;
use crate::context::function::Function;
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
    let function = context.function().to_owned();

    let offset = arguments[0].into_int_value();
    let size = arguments[1].into_int_value();

    context.write_abi_data(offset, size);
    long_return(context, function)?;

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
    let function = context.function().to_owned();

    let offset = arguments[0].into_int_value();
    let size = arguments[1].into_int_value();

    context.write_abi_data(offset, size);

    context.build_unconditional_branch(function.throw_block);
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
    let function = context.function().to_owned();

    context.write_abi_data(context.field_const(0), context.field_const(0));
    long_return(context, function)?;

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
    let function = context.function().to_owned();

    context.write_abi_data(context.field_const(0), context.field_const(0));

    context.build_unconditional_branch(function.throw_block);
    Ok(None)
}

///
/// Generates the long return sequence.
///
fn long_return<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    function: Function<'ctx>,
) -> anyhow::Result<()>
where
    D: Dependency,
{
    if context.function().name == Runtime::FUNCTION_SELECTOR
        || context.function().name == Runtime::FUNCTION_CONSTRUCTOR
    {
        context.build_unconditional_branch(function.return_block);
    } else {
        let long_return_flag_pointer = context.access_memory(
            context.long_return_offset(),
            AddressSpace::Heap,
            "long_return_flag_pointer",
        );
        context.build_store(long_return_flag_pointer, context.field_const(1));
        context.build_unconditional_branch(function.throw_block);
    }

    Ok(())
}
