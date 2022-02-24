//!
//! Translates a contract call.
//!

use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::argument::Argument;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates a contract call.
///
#[allow(clippy::too_many_arguments)]
pub fn call<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    call_type: IntrinsicFunction,
    address: inkwell::values::IntValue<'ctx>,
    value: Option<inkwell::values::IntValue<'ctx>>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_size: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    if let Some(value) = value {
        crate::evm::check_value_zero(context, value);
    }

    let identity_block = context.append_basic_block("contract_call_identity_block");
    let ordinary_block = context.append_basic_block("contract_call_ordinary_block");
    let join_block = context.append_basic_block("contract_call_join_block");

    let result_pointer = context.build_alloca(context.field_type(), "contract_call_result_pointer");
    context.build_store(result_pointer, context.field_const(0));

    let is_address_identity = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        address,
        context.field_const_str(compiler_common::ABI_ADDRESS_IDENTITY),
        "contract_call_is_address_identity",
    );
    context.build_conditional_branch(is_address_identity, identity_block, ordinary_block);

    context.set_basic_block(identity_block);
    let result = call_identity(context, output_offset, input_offset, output_size)?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(ordinary_block);
    let result = call_ordinary(
        context,
        call_type,
        address,
        input_offset,
        input_size,
        output_offset,
        output_size,
    )?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "contract_call_result");

    Ok(Some(result))
}

///
/// Translates a linker symbol.
///
pub fn linker_symbol<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
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
/// Generates an ordinary contract call
///
fn call_ordinary<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    call_type: IntrinsicFunction,
    address: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_size: inkwell::values::IntValue<'ctx>,
    output_offset: inkwell::values::IntValue<'ctx>,
    output_size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let intrinsic = context.get_intrinsic_function(IntrinsicFunction::SwitchContext);
    context.build_call(intrinsic, &[], "contract_call_switch_context");

    let intrinsic = context.get_intrinsic_function(call_type);
    let call_definition = context.builder().build_left_shift(
        address,
        context.field_const((compiler_common::BITLENGTH_X32) as u64),
        "",
    );
    let result = context
        .build_call(
            intrinsic,
            &[
                call_definition.as_basic_value_enum(),
                input_offset.as_basic_value_enum(),
                input_size.as_basic_value_enum(),
                context.field_const(0).as_basic_value_enum(),
            ],
            "contract_call_external",
        )
        .expect("IntrinsicFunction always returns a flag");

    let child_data_offset = context.builder().build_and(
        result.into_int_value(),
        context.field_const(compiler_common::BITLENGTH_X32 as u64),
        "contract_call_child_data_offset",
    );
    let source = context.access_memory(
        child_data_offset,
        AddressSpace::Child,
        "contract_call_source",
    );

    let destination = context.access_memory(
        output_offset,
        AddressSpace::Heap,
        "contract_call_destination",
    );

    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromChild,
        destination,
        source,
        output_size,
        "contract_call_memcpy_from_child",
    );

    Ok(context.field_const(1).as_basic_value_enum())
}

///
/// Generates a memcopy call for the Identity precompile.
///
fn call_identity<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
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
