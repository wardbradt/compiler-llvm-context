//!
//! The common code generation utils.
//!

pub mod arithmetic;
pub mod bitwise;
pub mod calldata;
pub mod comparison;
pub mod context;
pub mod contract;
pub mod create;
pub mod ether_gas;
pub mod event;
pub mod ext_code;
pub mod hash;
pub mod immutable;
pub mod math;
pub mod memory;
pub mod r#return;
pub mod return_data;
pub mod storage;
pub mod verbatim;

use num::BigUint;
use num::Num;

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
