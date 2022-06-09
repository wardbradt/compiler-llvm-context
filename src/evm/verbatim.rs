//!
//! Translates the verbatim instructions.
//!

use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::function::runtime::Runtime;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the 0-in-0-out instruction.
///
pub fn i0_o0<'ctx, D>(
    context: &mut Context<'ctx, D>,
    identifier: String,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    match identifier.as_str() {
        "00000000" => {
            context.build_call(
                context.runtime.cxa_throw,
                &[context
                    .integer_type(compiler_common::BITLENGTH_BYTE)
                    .ptr_type(AddressSpace::Stack.into())
                    .const_null()
                    .as_basic_value_enum(); 3],
                Runtime::FUNCTION_CXA_THROW,
            );
            context.build_unreachable();
        }
        _ => {}
    }

    Ok(None)
}
