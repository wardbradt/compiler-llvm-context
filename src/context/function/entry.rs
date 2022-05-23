//!
//! The LLVM entry function.
//!

use inkwell::types::BasicType;
use inkwell::values::BasicValue;

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
    /// The calldata offset argument index.
    pub const ARGUMENT_INDEX_CALLDATA_OFFSET: usize = 0;

    /// The calldata length argument index.
    pub const ARGUMENT_INDEX_CALLDATA_LENGTH: usize = 1;

    /// The deploy code call flag argument index.
    pub const ARGUMENT_INDEX_IS_DEPLOY_CALL: usize = 2;
}

impl<D> WriteLLVM<D> for Entry
where
    D: Dependency,
{
    fn declare(&mut self, context: &mut Context<D>) -> anyhow::Result<()> {
        let function_type = context.function_type(
            1,
            vec![
                context.field_type().as_basic_type_enum(),
                context.field_type().as_basic_type_enum(),
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
        let calldata_offset = context
            .function()
            .value
            .get_nth_param(Self::ARGUMENT_INDEX_CALLDATA_OFFSET as u32)
            .expect("Always exists")
            .into_int_value();
        let calldata_offset = context.builder().build_and(
            calldata_offset,
            context.field_const(((1 << 24) - 1) as u64),
            "calldata_offset_masked",
        );
        let calldata_length = context
            .function()
            .value
            .get_nth_param(Self::ARGUMENT_INDEX_CALLDATA_LENGTH as u32)
            .expect("Always exists")
            .into_int_value();
        context.write_abi_data(calldata_offset, calldata_length, AddressSpace::Parent);
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

        context.build_catch_block();

        context.set_basic_block(context.function().return_block);
        let return_value = context
            .read_abi_data(AddressSpace::Parent)
            .as_basic_value_enum();
        context.build_return(Some(&return_value));

        Ok(())
    }
}
