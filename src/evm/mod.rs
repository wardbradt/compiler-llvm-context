//!
//! The common code generation utils.
//!

pub mod arithmetic;
pub mod bitwise;
pub mod calldata;
pub mod comparison;
pub mod contract;
pub mod create;
pub mod ether_gas;
pub mod event;
pub mod hash;
pub mod immutable;
pub mod math;
pub mod memory;
pub mod r#return;
pub mod return_data;
pub mod storage;
pub mod verbatim;

use std::str::FromStr;

use num::BigUint;
use num::Num;

use crate::context::address_space::AddressSpace;
use crate::context::Context;
use crate::Dependency;

///
/// Stores the temporarily rewritten values in a safe heap space.
///
pub fn save_and_write_extra_data<'ctx, D>(
    context: &mut Context<'ctx, D>,
    source_offset: inkwell::values::IntValue<'ctx>,
    value: inkwell::values::IntValue<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    let value_pointer = context.access_memory(
        source_offset,
        AddressSpace::Heap,
        "save_and_write_extra_data_value_pointer",
    );
    let value_saved_data =
        context.build_load(value_pointer, "save_and_write_extra_data_value_saved_data");
    context.build_store(value_pointer, value);

    let address_offset = context.builder().build_int_add(
        source_offset,
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "save_and_write_extra_data_address_offset",
    );
    let address_pointer = context.access_memory(
        address_offset,
        AddressSpace::Heap,
        "save_and_write_extra_data_address_pointer",
    );
    let address_saved_data = context.build_load(
        address_pointer,
        "save_and_write_extra_data_address_saved_data",
    );
    context.build_store(address_pointer, address);

    let value_saved_data_pointer = context.access_memory(
        context.field_const(compiler_common::ABI_MEMORY_OFFSET_ABI_VALUE_TEMPORARY_DATA as u64),
        AddressSpace::Heap,
        "save_and_write_extra_data_value_saved_data_pointer",
    );
    context.build_store(value_saved_data_pointer, value_saved_data);
    let address_saved_data_pointer = context.access_memory(
        context.field_const(compiler_common::ABI_MEMORY_OFFSET_ABI_ADDRESS_TEMPORARY_DATA as u64),
        AddressSpace::Heap,
        "save_and_write_extra_data_address_saved_data_pointer",
    );
    context.build_store(address_saved_data_pointer, address_saved_data);

    Ok(None)
}

///
/// Parses an address and returns its BigUint representation.
///
/// # Panics
/// If the `address` is invalid
///
pub fn parse_address(address: &str) -> BigUint {
    let address = address.strip_prefix("0x").unwrap_or(address);
    BigUint::from_str_radix(address, compiler_common::BASE_HEXADECIMAL as u32)
        .expect("Always valid")
}

///
/// Parses an LLVM constant and returns its BigUint representation.
///
/// # Panics
/// If the LLVM notation is invalid
///
pub fn parse_llvm_constant(value: inkwell::values::IntValue) -> Option<BigUint> {
    let debug_string = format!("{:?}", value);
    let value_match = regex::Regex::new(r#"i256\s[0-9]+"#)
        .expect("Always valid")
        .captures(debug_string.as_str())?
        .get(0)?;
    let value_string = value_match.as_str().split(' ').last()?;
    Some(BigUint::from_str(value_string).expect("Always valid"))
}
