//!
//! Translates the hash instruction.
//!

use inkwell::types::BasicType;
use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the hash instruction.
///
pub fn keccak256<'ctx, D>(
    context: &mut Context<'ctx, D>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let success_block = context.append_basic_block("keccak256_success_block");
    let failure_block = context.append_basic_block("keccak256_failure_block");
    let join_block = context.append_basic_block("keccak256_failure_block");

    let input_size_shifted = context.builder().build_left_shift(
        input_size,
        context.field_const(compiler_common::BITLENGTH_X64 as u64),
        "keccak256_call_input_size_shifted",
    );
    let abi_data = context.builder().build_int_add(
        input_size_shifted,
        input_offset,
        "keccak256_call_abi_data",
    );

    let address = context.field_const_str(compiler_common::ABI_ADDRESS_KECCAK256);

    let result_type = context
        .structure_type(vec![
            context
                .integer_type(compiler_common::BITLENGTH_FIELD)
                .as_basic_type_enum(),
            context
                .integer_type(compiler_common::BITLENGTH_BOOLEAN)
                .as_basic_type_enum(),
        ])
        .as_basic_type_enum();
    let result_pointer = context.build_alloca(result_type, "keccak256_call_result_pointer");

    let result_pointer = context
        .build_call(
            context.runtime.static_call,
            &[
                address.as_basic_value_enum(),
                abi_data.as_basic_value_enum(),
                result_pointer.as_basic_value_enum(),
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
    let child_data_offset = context.builder().build_and(
        result_abi_data.into_int_value(),
        context.field_const(u64::MAX as u64),
        "keccak256_child_data_offset",
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
    let child_pointer = context.access_memory(
        child_data_offset,
        AddressSpace::Child,
        "keccak256_child_pointer",
    );
    let child_data = context.build_load(child_pointer, "keccak256_child_data");
    context.build_store(result_pointer, child_data);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(failure_block);
    let child_data_length_shifted = context.builder().build_right_shift(
        result_abi_data.into_int_value(),
        context.field_const(compiler_common::BITLENGTH_X64 as u64),
        false,
        "keccak256_child_data_length_shifted",
    );
    let child_data_length = context.builder().build_and(
        child_data_length_shifted,
        context.field_const(u64::MAX as u64),
        "keccak256_child_data_length",
    );
    let source = context.access_memory(child_data_offset, AddressSpace::Child, "keccak256_source");
    let destination = context.access_memory(
        context.field_const(0),
        AddressSpace::Heap,
        "keccak256_destination",
    );
    context.build_memcpy(
        IntrinsicFunction::MemoryCopyFromChild,
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
    let result = context.build_load(child_pointer, "keccak256_result");
    Ok(Some(result))
}
