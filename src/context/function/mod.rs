//!
//! The LLVM generator function.
//!

pub mod block;
pub mod deploy_code;
pub mod entry;
pub mod evm_data;
pub mod intrinsic;
pub mod r#return;
pub mod runtime;
pub mod runtime_code;

use std::collections::HashMap;

use self::evm_data::EVMData;
use self::r#return::Return;

///
/// The LLVM generator function.
///
#[derive(Debug, Clone)]
pub struct Function<'ctx> {
    /// The high-level source code name.
    pub name: String,
    /// The LLVM function value.
    pub value: inkwell::values::FunctionValue<'ctx>,

    /// The entry block. Each LLVM IR functions must have an entry block.
    pub entry_block: inkwell::basic_block::BasicBlock<'ctx>,
    /// The return/leave block. LLVM IR functions may have multiple returning blocks, but it is
    /// more reasonable to have a single returning block and other high-level language returns
    /// jumping to it. This way it is easier to implement some additional checks and clean-ups
    /// before the returning.
    pub return_block: inkwell::basic_block::BasicBlock<'ctx>,

    /// The return value entity.
    pub r#return: Option<Return<'ctx>>,
    /// The stack representation.
    pub stack: HashMap<String, inkwell::values::PointerValue<'ctx>>,
    /// The constants saved to variables. Used for peculiar cases like call simulation.
    /// It is a partial implementation of the constant propagation.
    pub constants: HashMap<String, num::BigUint>,
    /// The block-local variables. They are still allocated at the beginning of the function,
    /// but their parent block must be known in order to pass the implicit arguments thereto.
    /// Is only used by the Vyper LLL IR compiler.
    pub label_arguments: HashMap<String, Vec<String>>,

    /// The EVM compiler data.
    pub evm_data: Option<EVMData<'ctx>>,
}

impl<'ctx> Function<'ctx> {
    /// The near call ABI function prefix.
    pub const ZKSYNC_NEAR_CALL_ABI_PREFIX: &'static str = "ZKSYNC_NEAR_CALL";

    /// The near call ABI exception handler name.
    pub const ZKSYNC_NEAR_CALL_ABI_EXCEPTION_HANDLER: &'static str = "ZKSYNC_CATCH_NEAR_CALL";

    /// The stack hashmap default capacity.
    const STACK_HASHMAP_INITIAL_CAPACITY: usize = 64;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: String,
        value: inkwell::values::FunctionValue<'ctx>,

        entry_block: inkwell::basic_block::BasicBlock<'ctx>,
        return_block: inkwell::basic_block::BasicBlock<'ctx>,

        r#return: Option<Return<'ctx>>,
    ) -> Self {
        Self {
            name,
            value,

            entry_block,
            return_block,

            r#return,
            stack: HashMap::with_capacity(Self::STACK_HASHMAP_INITIAL_CAPACITY),
            constants: HashMap::new(),
            label_arguments: HashMap::new(),

            evm_data: None,
        }
    }

    ///
    /// Sets the function return data.
    ///
    pub fn set_return(&mut self, r#return: Return<'ctx>) {
        self.r#return = Some(r#return);
    }

    ///
    /// Returns the pointer to the function return value.
    ///
    /// # Panics
    /// If the pointer has not been set yet.
    ///
    pub fn return_pointer(&self) -> Option<inkwell::values::PointerValue<'ctx>> {
        self.r#return
            .as_ref()
            .expect("Always exists")
            .return_pointer()
    }

    ///
    /// Returns the return data size in bytes, based on the default stack alignment.
    ///
    /// # Panics
    /// If the pointer has not been set yet.
    ///
    pub fn return_data_size(&self) -> usize {
        self.r#return
            .as_ref()
            .map(|r#return| r#return.return_data_size())
            .unwrap_or_default()
    }

    ///
    /// Returns the EVM data reference.
    ///
    /// # Panics
    /// If the EVM data has not been initialized.
    ///
    pub fn evm(&self) -> &EVMData<'ctx> {
        self.evm_data
            .as_ref()
            .expect("The EVM data must have been initialized")
    }

    ///
    /// Returns the EVM data mutable reference.
    ///
    /// # Panics
    /// If the EVM data has not been initialized.
    ///
    pub fn evm_mut(&mut self) -> &mut EVMData<'ctx> {
        self.evm_data
            .as_mut()
            .expect("The EVM data must have been initialized")
    }
}
