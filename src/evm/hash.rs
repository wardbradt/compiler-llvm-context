//!
//! Translates the hash instruction.
//!

use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the hash instruction.
///
pub fn keccak256<'ctx, 'dep, D>(
    context: &mut Context<'ctx, 'dep, D>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let intrinsic = context.get_intrinsic_function(IntrinsicFunction::SwitchContext);
    context.build_call(intrinsic, &[], "keccak256_switch_context");

    let intrinsic = context.get_intrinsic_function(IntrinsicFunction::StaticCall);
    let call_definition = context.builder().build_left_shift(
        context.field_const_str(compiler_common::ABI_ADDRESS_KECCAK256),
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
            "keccak256_call_external",
        )
        .expect("Always returns a value");

    let child_offset = context.builder().build_and(
        result.into_int_value(),
        context.field_const(compiler_common::BITLENGTH_X32 as u64),
        "keccak256_child_offset",
    );
    let child_pointer =
        context.access_memory(child_offset, AddressSpace::Child, "keccak256_child_pointer");

    let result = context.build_load(child_pointer, "keccak256_result");

    Ok(Some(result))
}
