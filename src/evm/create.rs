//!
//! Translates the contract creation instructions.
//!

use inkwell::types::BasicType;
use inkwell::values::BasicValue;

use crate::context::Context;
use crate::AddressSpace;
use crate::Dependency;

///
/// The deployer call header size, which consists of:
/// - selector (4 bytes)
/// - salt (32 bytes)
/// - bytecode hash (32 bytes)
/// - ether value (32 bytes)
/// - constructor arguments offset (32 bytes)
/// - constructor arguments length (32 bytes)
///
pub const HEADER_SIZE: usize = compiler_common::SIZE_X32 + (compiler_common::SIZE_FIELD * 4);

///
/// Translates the contract `create` instruction.
///
pub fn create<'ctx, D>(
    context: &mut Context<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    address_space: AddressSpace,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let address = call_deployer(
        context,
        value,
        input_offset,
        input_length,
        "create(bytes32,bytes32,bytes)",
        None,
        address_space,
    )?;

    Ok(Some(address.as_basic_value_enum()))
}

///
/// Translates the contract `create2` instruction.
///
pub fn create2<'ctx, D>(
    context: &mut Context<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    salt: Option<inkwell::values::IntValue<'ctx>>,
    address_space: AddressSpace,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let address = call_deployer(
        context,
        value,
        input_offset,
        input_length,
        "create2(bytes32,bytes32,bytes)",
        salt,
        address_space,
    )?;

    Ok(Some(address.as_basic_value_enum()))
}

///
/// Translates the contract hash instruction, which is actually used to set the hash of the contract
/// being created, or other related auxiliary data.
///
/// `dataoffset` in Yul, `PUSH [$]` in legacy assembly.
///
pub fn contract_hash<'ctx, D>(
    context: &mut Context<'ctx, D>,
    identifier: String,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let parent = context.module().get_name().to_str().expect("Always valid");

    let contract_path = context.resolve_path(identifier.as_str())?;
    if identifier.ends_with("_deployed") || contract_path.as_str() == parent {
        return Ok(Some(context.field_const(0).as_basic_value_enum()));
    }

    let hash_value = context
        .compile_dependency(identifier.as_str())
        .map(|hash| context.field_const_str(hash.as_str()))
        .map(inkwell::values::BasicValueEnum::IntValue)?;

    Ok(Some(hash_value))
}

///
/// Translates the deployer call header size instruction, Usually, the header consists of:
/// - the deployer contract method signature
/// - the salt if the call is `create2`, or zero if the call is `create1`
/// - the hash of the bytecode of the contract whose instance is being created
/// - the passed Ether value
/// - the offset of the constructor arguments
/// - the length of the constructor arguments
///
/// If the call is `create1`, the space for the salt is still allocated, because the memory for the
/// header is allocated before it is known which version of `create` is going to be used.
///
/// Concerning AST, it looks like `datasize` in Yul, and `PUSH #[$]` in the EVM legacy assembly.
///
pub fn header_size<'ctx, D>(
    context: &mut Context<'ctx, D>,
    identifier: String,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let parent = context.module().get_name().to_str().expect("Always valid");

    let contract_path = context.resolve_path(identifier.as_str())?;
    if identifier.ends_with("_deployed") || contract_path.as_str() == parent {
        return Ok(Some(context.field_const(0).as_basic_value_enum()));
    }

    Ok(Some(
        context
            .field_const(HEADER_SIZE as u64)
            .as_basic_value_enum(),
    ))
}

///
/// Calls the deployer system contract, which returns the newly deployed contract address.
///
fn call_deployer<'ctx, D>(
    context: &mut Context<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    signature: &'static str,
    salt: Option<inkwell::values::IntValue<'ctx>>,
    address_space: AddressSpace,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency,
{
    let error_block = context.append_basic_block("deployer_call_error_block");
    let success_block = context.append_basic_block("deployer_call_success_block");
    let join_block = context.append_basic_block("deployer_call_join_block");
    let value_zero_block = context.append_basic_block("deployer_call_value_zero_block");
    let value_non_zero_block = context.append_basic_block("deployer_call_value_non_zero_block");
    let value_join_block = context.append_basic_block("deployer_call_value_join_block");

    let abi_data = crate::evm::contract::abi_data(
        context,
        input_offset,
        input_length,
        context.field_const(0),
        address_space,
        true,
    )?;

    let signature_hash = crate::hashes::keccak256(signature.as_bytes());
    let signature_pointer = context.access_memory(
        input_offset,
        address_space,
        "deployer_call_signature_pointer",
    );
    let signature_value = context.field_const_str(signature_hash.as_str());
    context.build_store(signature_pointer, signature_value);

    let salt_offset = context.builder().build_int_add(
        input_offset,
        context.field_const(compiler_common::SIZE_X32 as u64),
        "deployer_call_salt_offset",
    );
    let salt_pointer =
        context.access_memory(salt_offset, address_space, "deployer_call_salt_pointer");
    let salt_value = salt.unwrap_or_else(|| context.field_const(0));
    context.build_store(salt_pointer, salt_value);

    let arguments_offset_offset = context.builder().build_int_add(
        salt_offset,
        context.field_const((compiler_common::SIZE_FIELD * 2) as u64),
        "deployer_call_arguments_offset_offset",
    );
    let arguments_offset_pointer = context.access_memory(
        arguments_offset_offset,
        address_space,
        "deployer_call_arguments_offset_pointer",
    );
    context.build_store(
        arguments_offset_pointer,
        context.field_const(
            (HEADER_SIZE - (compiler_common::SIZE_X32 + compiler_common::SIZE_FIELD)) as u64,
        ),
    );

    let arguments_length_offset = context.builder().build_int_add(
        arguments_offset_offset,
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "deployer_call_arguments_length_offset",
    );
    let arguments_length_pointer = context.access_memory(
        arguments_length_offset,
        address_space,
        "deployer_call_arguments_length_pointer",
    );
    let arguments_length_value = context.builder().build_int_sub(
        input_length,
        context.field_const(HEADER_SIZE as u64),
        "deployer_call_arguments_length",
    );
    context.build_store(arguments_length_pointer, arguments_length_value);

    let result_pointer = context.build_alloca(context.field_type(), "deployer_call_result_pointer");
    context.build_store(result_pointer, context.field_const(0));
    let deployer_call_result_pointer_pointer_type = context
        .structure_type(vec![
            context
                .integer_type(compiler_common::BITLENGTH_BYTE)
                .ptr_type(AddressSpace::Generic.into())
                .as_basic_type_enum(),
            context
                .integer_type(compiler_common::BITLENGTH_BOOLEAN)
                .as_basic_type_enum(),
        ])
        .ptr_type(AddressSpace::Stack.into());
    let deployer_call_result_pointer_pointer = context.build_alloca(
        deployer_call_result_pointer_pointer_type,
        "deployer_call_result_pointer_pointer",
    );
    context.build_store(
        deployer_call_result_pointer_pointer,
        deployer_call_result_pointer_pointer_type.const_zero(),
    );
    let is_value_zero = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        value,
        context.field_const(0),
        "deployer_call_is_value_zero",
    );
    context.build_conditional_branch(is_value_zero, value_zero_block, value_non_zero_block);

    context.set_basic_block(value_zero_block);
    let deployer_call_result_pointer = context
        .build_invoke_far_call(
            context.runtime.far_call,
            vec![
                abi_data.as_basic_value_enum(),
                context
                    .field_const_str(compiler_common::ADDRESS_CONTRACT_DEPLOYER)
                    .as_basic_value_enum(),
            ],
            "deployer_call_ordinary",
        )
        .expect("Always returns a value");
    context.build_store(
        deployer_call_result_pointer_pointer,
        deployer_call_result_pointer,
    );
    context.build_unconditional_branch(value_join_block);

    context.set_basic_block(value_non_zero_block);
    let system_call_bit = context.builder().build_left_shift(
        context.field_const(1),
        context.field_const((compiler_common::BITLENGTH_X32 * 4) as u64),
        "deployer_call_system_call_bit",
    );
    let value_and_system_call_bit = context.builder().build_or(
        value,
        system_call_bit,
        "deployer_call_value_and_system_call_bit",
    );
    let deployer_call_result_pointer = context
        .build_invoke_far_call(
            context.runtime.system_call,
            vec![
                abi_data.as_basic_value_enum(),
                context
                    .field_const_str(compiler_common::ADDRESS_MSG_VALUE)
                    .as_basic_value_enum(),
                value_and_system_call_bit.as_basic_value_enum(),
                context
                    .field_const_str(compiler_common::ADDRESS_CONTRACT_DEPLOYER)
                    .as_basic_value_enum(),
            ],
            "deployer_call_system",
        )
        .expect("Always returns a value");
    context.build_store(
        deployer_call_result_pointer_pointer,
        deployer_call_result_pointer,
    );
    context.build_unconditional_branch(value_join_block);

    context.set_basic_block(value_join_block);
    let deployer_call_result_pointer = context.build_load(
        deployer_call_result_pointer_pointer,
        "deployer_call_result_pointer_join",
    );
    let result_abi_data_pointer = unsafe {
        context.builder().build_gep(
            deployer_call_result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_zero(),
            ],
            "deployer_call_result_abi_data_pointer",
        )
    };
    let result_abi_data =
        context.build_load(result_abi_data_pointer, "deployer_call_result_abi_data");
    let result_abi_data_casted = context.builder().build_pointer_cast(
        result_abi_data.into_pointer_value(),
        context.field_type().ptr_type(AddressSpace::Generic.into()),
        "deployer_call_result_abi_data_casted",
    );

    let result_status_code_pointer = unsafe {
        context.builder().build_gep(
            deployer_call_result_pointer.into_pointer_value(),
            &[
                context.field_const(0),
                context
                    .integer_type(compiler_common::BITLENGTH_X32)
                    .const_int(1, false),
            ],
            "contract_call_external_result_status_code_pointer",
        )
    };
    let result_status_code_boolean = context
        .build_load(
            result_status_code_pointer,
            "contract_call_external_result_status_code_boolean",
        )
        .into_int_value();

    let address_or_status_code = context.build_load(
        result_abi_data_casted,
        "deployer_call_address_or_status_code",
    );
    let is_address_or_status_code_non_zero = context.builder().build_int_compare(
        inkwell::IntPredicate::NE,
        address_or_status_code.into_int_value(),
        context.field_const(0),
        "deployer_call_is_address_or_status_code_non_zero",
    );
    let is_address_or_status_code_non_zero_with_exception = context.builder().build_and(
        is_address_or_status_code_non_zero,
        result_status_code_boolean,
        "deployer_call_is_address_or_status_code_non_zero_with_exception",
    );
    context.build_conditional_branch(
        is_address_or_status_code_non_zero_with_exception,
        success_block,
        error_block,
    );

    context.set_basic_block(success_block);
    context.build_store(result_pointer, address_or_status_code);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(error_block);
    context.write_abi_return_data_deployer(result_abi_data.into_pointer_value());
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "deployer_call_result");
    Ok(result)
}
