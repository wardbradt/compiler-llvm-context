//!
//! Translates the verbatim instructions.
//!

use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::function::runtime::Runtime;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the `throw` instruction.
///
/// This instruction is a zkSync Yul extension. It allows to throw exceptions in the Yul mode.
///
pub fn throw<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
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

    Ok(None)
}

/// The global getter identifier prefix.
pub static GLOBAL_GETTER_PREFIX: &str = "get_global::";
