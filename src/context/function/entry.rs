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

impl<D> WriteLLVM<D> for Entry
where
    D: Dependency,
{
    fn declare(&mut self, context: &mut Context<D>) -> anyhow::Result<()> {
        let function_type =
            context.function_type(1, vec![context.field_type().as_basic_type_enum()]);
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
        let abi_data = context
            .function()
            .value
            .get_first_param()
            .expect("Always exists")
            .into_int_value();
        let calldata_offset =
            context
                .builder()
                .build_and(abi_data, context.field_const(u64::MAX), "calldata_offset");
        let calldata_length_shifted = context.builder().build_right_shift(
            abi_data,
            context.field_const(compiler_common::BITLENGTH_X64 as u64),
            false,
            "calldata_length_shifted",
        );
        let calldata_length = context.builder().build_and(
            calldata_length_shifted,
            context.field_const(u64::MAX),
            "calldata_length",
        );
        context.write_abi_data(calldata_offset, calldata_length, AddressSpace::Parent);
        let is_constructor_call = context.builder().build_right_shift(
            abi_data,
            context.field_const((compiler_common::BITLENGTH_X64 * 3) as u64),
            false,
            "is_constructor_call",
        );
        let is_constructor_call = context.builder().build_int_cast(
            is_constructor_call,
            context.integer_type(compiler_common::BITLENGTH_BOOLEAN),
            "is_constructor_call_boolean",
        );
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
