//!
//! The LLVM context constants.
//!

/// The calldata ABI pointer global variable name.
pub static GLOBAL_CALLDATA_ABI: &str = "ptr_calldata";

/// The calldata size pointer global variable name.
pub static GLOBAL_CALLDATA_SIZE: &str = "calldatasize";

/// The return data ABI pointer global variable name.
pub static GLOBAL_RETURN_DATA_ABI: &str = "ptr_return_data";

/// The return data size pointer global variable name.
pub static GLOBAL_RETURN_DATA_SIZE: &str = "returndatasize";

/// The temporary ETH value simulator `msg.value` global variable name.
pub static GLOBAL_TEMP_SIMULATOR_MSG_VALUE: &str = "temp_msg_value";

/// The temporary ETH value simulator `address` global variable name.
pub static GLOBAL_TEMP_SIMULATOR_ADDRESS: &str = "temp_address";

/// The external call data offset in the auxiliary heap.
pub const HEAP_AUX_OFFSET_EXTERNAL_CALL: u64 = 0;

/// The constructor return data offset in the auxiliary heap.
pub const HEAP_AUX_OFFSET_CONSTRUCTOR_RETURN_DATA: u64 = 8 * (compiler_common::SIZE_FIELD as u64);
