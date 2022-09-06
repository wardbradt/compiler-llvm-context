//!
//! The LLVM entry function.
//!

use inkwell::types::BasicType;

use crate::context::address_space::AddressSpace;
use crate::context::function::runtime::Runtime;
use crate::context::Context;
use crate::Dependency;
use crate::WriteLLVM;

///
/// The LLVM entry function.
///
/// The function is a wrapper managing the runtime and deploy code calling logic.
///
#[derive(Debug, Default)]
pub struct Entry {}

impl Entry {
    /// The calldata ABI argument index.
    pub const ARGUMENT_INDEX_CALLDATA_ABI: usize = 0;

    /// The deploy code call flag argument index.
    pub const ARGUMENT_INDEX_IS_DEPLOY_CALL: usize = 1;

    ///
    /// Initializes the global variables.
    ///
    /// The pointers are not initialized, because it's not possible to create a null pointer.
    ///
    pub fn initialize_globals<D>(context: &mut Context<D>) -> anyhow::Result<()>
    where
        D: Dependency,
    {
        context.set_global(crate::GLOBAL_CALLDATA_SIZE, context.field_const(0));
        context.set_global(crate::GLOBAL_RETURN_DATA_SIZE, context.field_const(0));
        context.set_global(
            crate::GLOBAL_TEMP_SIMULATOR_MSG_VALUE,
            context.field_const(0),
        );
        context.set_global(crate::GLOBAL_TEMP_SIMULATOR_ADDRESS, context.field_const(0));
        Ok(())
    }
}

impl<D> WriteLLVM<D> for Entry
where
    D: Dependency,
{
    fn declare(&mut self, context: &mut Context<D>) -> anyhow::Result<()> {
        let function_type = context.function_type(
            1,
            vec![
                context
                    .integer_type(compiler_common::BITLENGTH_BYTE)
                    .ptr_type(AddressSpace::Generic.into())
                    .as_basic_type_enum(),
                context
                    .integer_type(compiler_common::BITLENGTH_BOOLEAN)
                    .as_basic_type_enum(),
            ],
        );
        context.add_function(
            Runtime::FUNCTION_ENTRY,
            function_type,
            Some(inkwell::module::Linkage::External),
        );

        Ok(())
    }

    fn into_llvm(self, context: &mut Context<D>) -> anyhow::Result<()> {
        let function = context
            .functions
            .get(Runtime::FUNCTION_ENTRY)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract entry not found"))?;
        context.set_function(function);

        let deploy_code_call_block = context.append_basic_block("deploy_code_call_block");
        let runtime_code_call_block = context.append_basic_block("runtime_code_call_block");

        let deploy_code = context
            .functions
            .get(Runtime::FUNCTION_DEPLOY_CODE)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract deploy code not found"))?;
        let runtime_code = context
            .functions
            .get(Runtime::FUNCTION_RUNTIME_CODE)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract runtime code not found"))?;

        context.set_basic_block(context.function().entry_block);
        Self::initialize_globals(context)?;

        let calldata_abi = context
            .function()
            .value
            .get_nth_param(Self::ARGUMENT_INDEX_CALLDATA_ABI as u32)
            .expect("Always exists")
            .into_pointer_value();
        context.write_abi_calldata(calldata_abi);
        let calldata_length = context.get_global(crate::r#const::GLOBAL_CALLDATA_SIZE)?;
        let return_data_abi = unsafe {
            context.builder().build_gep(
                calldata_abi,
                &[calldata_length.into_int_value()],
                "return_data_abi_initializer",
            )
        };
        context.write_abi_return_data(return_data_abi);

        let is_deploy_code_call = context
            .function()
            .value
            .get_nth_param(Self::ARGUMENT_INDEX_IS_DEPLOY_CALL as u32)
            .expect("Always exists")
            .into_int_value();
        context.build_conditional_branch(
            is_deploy_code_call,
            deploy_code_call_block,
            runtime_code_call_block,
        );

        context.set_basic_block(deploy_code_call_block);
        context.build_invoke(deploy_code.value, &[], "deploy_code_call");
        context.build_unconditional_branch(context.function().return_block);

        context.set_basic_block(runtime_code_call_block);
        context.build_invoke(runtime_code.value, &[], "runtime_code_call");
        context.build_unconditional_branch(context.function().return_block);

        context.set_basic_block(context.function().return_block);
        context.build_return(Some(&context.field_const(0)));

        Ok(())
    }
}
