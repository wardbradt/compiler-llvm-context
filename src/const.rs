//!
//! The LLVM context constants.
//!

/// The calldata pointer global variable name.
pub static GLOBAL_CALLDATA_POINTER: &str = "ptr_calldata";

/// The calldata size pointer global variable name.
pub static GLOBAL_CALLDATA_SIZE: &str = "calldatasize";

/// The return data pointer global variable name.
pub static GLOBAL_RETURN_DATA_POINTER: &str = "ptr_return_data";

/// The return data size pointer global variable name.
pub static GLOBAL_RETURN_DATA_SIZE: &str = "returndatasize";

/// The call flags global variable name.
pub static GLOBAL_CALL_FLAGS: &str = "call_flags";

/// The extra ABI data global variable name.
pub static GLOBAL_EXTRA_ABI_DATA: &str = "extra_abi_data";

/// The active pointer global variable name.
pub static GLOBAL_ACTIVE_POINTER: &str = "ptr_active";

/// The external call data offset in the auxiliary heap.
pub const HEAP_AUX_OFFSET_EXTERNAL_CALL: u64 = 0;

/// The constructor return data offset in the auxiliary heap.
pub const HEAP_AUX_OFFSET_CONSTRUCTOR_RETURN_DATA: u64 = 8 * (compiler_common::SIZE_FIELD as u64);

/// The number of the extra ABI data arguments.
pub const EXTRA_ABI_DATA_SIZE: usize = 2;

/// The `ptr_calldata` global access index.
pub const GLOBAL_INDEX_CALLDATA_ABI: usize = 0;

/// The `call_flags` global access index.
pub const GLOBAL_INDEX_CALL_FLAGS: usize = 1;

/// The `extra_abi_data_1` global access index.
pub const GLOBAL_INDEX_EXTRA_ABI_DATA_1: usize = 2;

/// The `extra_abi_data_2` global access index.
pub const GLOBAL_INDEX_EXTRA_ABI_DATA_2: usize = 3;

/// The `ptr_return_data` global access index.
pub const GLOBAL_INDEX_RETURN_DATA_ABI: usize = 4;
