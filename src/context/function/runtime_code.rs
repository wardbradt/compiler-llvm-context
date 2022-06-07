//!
//! The LLVM runtime code function.
//!

use std::marker::PhantomData;

use crate::context::code_type::CodeType;
use crate::context::function::runtime::Runtime;
use crate::context::Context;
use crate::Dependency;
use crate::WriteLLVM;

///
/// The LLVM runtime code function.
///
#[derive(Debug, Default)]
pub struct RuntimeCode<B, D>
where
    B: WriteLLVM<D>,
    D: Dependency,
{
    /// The runtime code AST representation.
    inner: B,
    /// The `D` phantom data.
    _pd: PhantomData<D>,
}

impl<B, D> RuntimeCode<B, D>
where
    B: WriteLLVM<D>,
    D: Dependency,
{
    ///
    /// A shortcut constructor.
    ///
    pub fn new(inner: B) -> Self {
        Self {
            inner,
            _pd: PhantomData::default(),
        }
    }
}

impl<B, D> WriteLLVM<D> for RuntimeCode<B, D>
where
    B: WriteLLVM<D>,
    D: Dependency,
{
    fn declare(&mut self, context: &mut Context<D>) -> anyhow::Result<()> {
        let function_type = context.function_type(0, vec![]);
        context.add_function(
            Runtime::FUNCTION_RUNTIME_CODE,
            function_type,
            Some(inkwell::module::Linkage::Private),
        );

        self.inner.declare(context)
    }

    fn into_llvm(self, context: &mut Context<D>) -> anyhow::Result<()> {
        let function = context
            .functions
            .get(Runtime::FUNCTION_RUNTIME_CODE)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Contract runtime code not found"))?;
        context.set_function(function);

        context.set_basic_block(context.function().entry_block);
        context.set_code_type(CodeType::Runtime);
        self.inner.into_llvm(context)?;
        match context
            .basic_block()
            .get_last_instruction()
            .map(|instruction| instruction.get_opcode())
        {
            Some(inkwell::values::InstructionOpcode::Br) => {}
            Some(inkwell::values::InstructionOpcode::Switch) => {}
            _ => context.build_unconditional_branch(context.function().return_block),
        }

        context.set_basic_block(context.function().return_block);
        context.build_return(None);

        Ok(())
    }
}
