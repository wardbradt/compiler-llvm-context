//!
//! The LLVM entry function.
//!

use inkwell::types::BasicType;
use inkwell::values::BasicValue;

use crate::context::Context;
use crate::Dependency;
use crate::WriteLLVM;

///
/// The LLVM entry function.
///
/// The function is a wrapper managing the constructor and selector calling logic.
///
#[derive(Debug, Default)]
pub struct Entry {}

impl<D> WriteLLVM<D> for Entry
where
    D: Dependency,
{
    fn prepare(context: &mut Context<D>) -> anyhow::Result<()> {
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
            compiler_common::LLVM_FUNCTION_ENTRY,
            function_type,
            Some(inkwell::module::Linkage::External),
        );

        Ok(())
    }

    fn into_llvm(self, context: &mut Context<D>) -> anyhow::Result<()> {
        let function = context
            .functions
            .get(compiler_common::LLVM_FUNCTION_ENTRY)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract entry not found"))?;
        context.set_function(function);

        let constructor_call_block = context.append_basic_block("constructor_call_block");
        let selector_call_block = context.append_basic_block("selector_call_block");

        let constructor = context
            .functions
            .get(compiler_common::LLVM_FUNCTION_CONSTRUCTOR)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract constructor not found"))?;
        let selector = context
            .functions
            .get(compiler_common::LLVM_FUNCTION_SELECTOR)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract selector not found"))?;

        context.set_basic_block(context.function().entry_block);
        let calldata_offset = context
            .function()
            .value
            .get_nth_param(compiler_common::ABI_ENTRY_ARGUMENT_INDEX_CALLDATA_OFFSET as u32)
            .expect("Always exists")
            .into_int_value();
        let calldata_length = context
            .function()
            .value
            .get_nth_param(compiler_common::ABI_ENTRY_ARGUMENT_INDEX_CALLDATA_LENGTH as u32)
            .expect("Always exists")
            .into_int_value();
        context.write_abi_data(calldata_offset, calldata_length);
        let is_constructor_call = context
            .function()
            .value
            .get_nth_param(compiler_common::ABI_ENTRY_ARGUMENT_INDEX_IS_CONSTRUCTOR_CALL as u32)
            .expect("Always exists")
            .into_int_value();
        context.build_conditional_branch(
            is_constructor_call,
            constructor_call_block,
            selector_call_block,
        );

        context.set_basic_block(constructor_call_block);
        context.build_invoke(constructor.value, &[], "constructor_call");
        context.build_unconditional_branch(context.function().return_block);

        context.set_basic_block(selector_call_block);
        context.build_invoke(selector.value, &[], "selector_call");
        context.build_unconditional_branch(context.function().return_block);

        context.build_throw_block(true);
        context.build_catch_block(true);

        context.set_basic_block(context.function().return_block);
        let return_value = context.read_abi_data().as_basic_value_enum();
        context.build_return(Some(&return_value));

        Ok(())
    }
}
