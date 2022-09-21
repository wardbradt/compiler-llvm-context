//!
//! Translates the transaction return operations.
//!

use crate::context::address_space::AddressSpace;
use crate::context::code_type::CodeType;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates the `return` instruction.
///
/// Unlike in EVM, zkSync constructors return the array of contract immutables.
///
pub fn r#return<'ctx, D>(
    context: &mut Context<'ctx, D>,
    offset: inkwell::values::IntValue<'ctx>,
    length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    match context.code_type() {
        CodeType::Deploy => {
            let immutables_offset_pointer = context.access_memory(
                context.field_const(crate::r#const::HEAP_AUX_OFFSET_CONSTRUCTOR_RETURN_DATA),
                AddressSpace::HeapAuxiliary,
                "immutables_offset_pointer",
            );
            context.build_store(
                immutables_offset_pointer,
                context.field_const(compiler_common::SIZE_FIELD as u64),
            );

            let immutables_number_pointer = context.access_memory(
                context.field_const(
                    crate::r#const::HEAP_AUX_OFFSET_CONSTRUCTOR_RETURN_DATA
                        + (compiler_common::SIZE_FIELD as u64),
                ),
                AddressSpace::HeapAuxiliary,
                "immutables_number_pointer",
            );
            let immutable_values_size = context.immutable_size();
            context.build_store(
                immutables_number_pointer,
                context.field_const((immutable_values_size / compiler_common::SIZE_FIELD) as u64),
            );
            let immutables_size = context.builder().build_int_mul(
                context.field_const(immutable_values_size as u64),
                context.field_const(2),
                "immutables_size",
            );
            let return_data_length = context.builder().build_int_add(
                immutables_size,
                context.field_const((compiler_common::SIZE_FIELD * 2) as u64),
                "return_data_length",
            );

            context.build_exit(
                IntrinsicFunction::Return,
                context.field_const(crate::r#const::HEAP_AUX_OFFSET_CONSTRUCTOR_RETURN_DATA),
                return_data_length,
            );
        }
        CodeType::Runtime => {
            context.build_exit(IntrinsicFunction::Return, offset, length);
        }
    }

    Ok(None)
}

///
/// Translates the `revert` instruction.
///
pub fn revert<'ctx, D>(
    context: &mut Context<'ctx, D>,
    offset: inkwell::values::IntValue<'ctx>,
    length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    context.build_exit(IntrinsicFunction::Revert, offset, length);
    Ok(None)
}

///
/// Translates the `stop` instruction.
///
/// Is the same as `return(0, 0)`.
///
pub fn stop<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    self::r#return(context, context.field_const(0), context.field_const(0))
}

///
/// Translates the `invalid` instruction.
///
/// Is the same as `revert(0, 0)`.
///
pub fn invalid<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    self::revert(context, context.field_const(0), context.field_const(0))
}
