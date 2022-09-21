//!
//! Translates the hash instruction.
//!

use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the `keccak256` instruction.
///
pub fn keccak256<'ctx, D>(
    context: &mut Context<'ctx, D>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let success_block = context.append_basic_block("keccak256_success_block");
    let failure_block = context.append_basic_block("keccak256_failure_block");
    let join_block = context.append_basic_block("keccak256_failure_block");

    let abi_data = crate::evm::contract::abi_data(
        context,
        input_offset,
        input_length,
        context.field_const(0),
        AddressSpace::Heap,
        true,
    )?;
    let address = context.field_const(compiler_common::ADDRESS_KECCAK256.into());

    let result_pointer = context
        .build_invoke_far_call(
            context.runtime.static_call,
            vec![
                abi_data.as_basic_value_enum(),
                address.as_basic_value_enum(),
            ],
            "keccak256_call_external",
        )
        .expect("Always returns a value");
    let result_abi_data_pointer = context
        .builder()
        .build_struct_gep(
            result_pointer.into_pointer_value(),
            0,
            "keccak256_call_external_result_abi_data_pointer",
        )
        .expect("Always valid");
    let result_abi_data = context.build_load(
        result_abi_data_pointer,
        "keccak256_call_external_result_abi_data",
    );
    let result_abi_data_casted = context.builder().build_pointer_cast(
        result_abi_data.into_pointer_value(),
        context.field_type().ptr_type(AddressSpace::Generic.into()),
        "keccak256_call_external_result_abi_data_casted",
    );

    let result_status_code_pointer = unsafe {
        context.builder().build_gep(
            result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_int(1, false),
            ],
            "keccak256_external_result_status_code_pointer",
        )
    };
    let result_status_code_boolean = context.build_load(
        result_status_code_pointer,
        "keccak256_external_result_status_code_boolean",
    );
    let result_pointer = context.build_alloca(context.field_type(), "keccak256_result_pointer");
    context.build_store(result_pointer, context.field_const(0));
    context.build_conditional_branch(
        result_status_code_boolean.into_int_value(),
        success_block,
        failure_block,
    );

    context.set_basic_block(success_block);
    let child_data = context.build_load(result_abi_data_casted, "keccak256_child_data");
    context.build_store(result_pointer, child_data);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(failure_block);
    let result_abi_data_value = context.builder().build_ptr_to_int(
        result_abi_data.into_pointer_value(),
        context.field_type(),
        "keccak256_child_data_pointer_value",
    );
    let child_data_length_shifted = context.builder().build_right_shift(
        result_abi_data_value,
        context.field_const((compiler_common::BITLENGTH_X32 * 3) as u64),
        false,
        "keccak256_child_data_length_shifted",
    );
    let child_data_length = context.builder().build_and(
        child_data_length_shifted,
        context.field_const(u64::MAX as u64),
        "keccak256_child_data_length",
    );
    let source = result_abi_data_casted;
    let destination = context.access_memory(
        context.field_const(0),
        AddressSpace::Heap,
        "keccak256_destination",
    );
    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromGeneric,
        destination,
        source,
        child_data_length,
        "keccak256_memcpy_from_child",
    );
    context.build_exit(
        IntrinsicFunction::Revert,
        context.field_const(0),
        child_data_length,
    );

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "keccak256_result");
    Ok(Some(result))
}
