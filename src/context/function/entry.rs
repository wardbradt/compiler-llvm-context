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
/// The function is a wrapper managing the constructor and selector calling logic.
///
#[derive(Debug, Default)]
pub struct Entry {}

impl Entry {
    /// The calldata offset argument index.
    pub const ARGUMENT_INDEX_CALLDATA_OFFSET: usize = 0;

    /// The calldata length argument index.
    pub const ARGUMENT_INDEX_CALLDATA_LENGTH: usize = 1;

    /// The constructor call flag argument index.
    pub const ARGUMENT_INDEX_IS_CONSTRUCTOR_CALL: usize = 2;
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

        let constructor_call_block = context.append_basic_block("constructor_call_block");
        let selector_call_block = context.append_basic_block("selector_call_block");

        let constructor = context
            .functions
            .get(Runtime::FUNCTION_CONSTRUCTOR)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract constructor not found"))?;
        let selector = context
            .functions
            .get(Runtime::FUNCTION_SELECTOR)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract selector not found"))?;

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
        let is_constructor_call = context
            .function()
            .value
            .get_nth_param(Self::ARGUMENT_INDEX_IS_CONSTRUCTOR_CALL as u32)
            .expect("Always exists")
            .into_int_value();
        context.build_conditional_branch(
            is_constructor_call,
            constructor_call_block,
            selector_call_block,
        );

        context.set_basic_block(constructor_call_block);
        context.build_call(constructor.value, &[], "constructor_call");
        context.build_unconditional_branch(context.function().return_block);

        context.set_basic_block(selector_call_block);
        context.build_call(selector.value, &[], "selector_call");
        context.build_unconditional_branch(context.function().return_block);

        context.build_throw_block();
        context.build_catch_block();

        context.set_basic_block(context.function().return_block);
        let return_value = context
            .read_abi_data(AddressSpace::Parent)
            .as_basic_value_enum();
        context.build_return(Some(&return_value));

        Ok(())
    }
}
