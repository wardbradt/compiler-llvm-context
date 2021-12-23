//!
//! The LLVM context library.
//!

pub(crate) mod context;
pub(crate) mod dump_flag;

pub use self::context::address_space::AddressSpace;
pub use self::context::argument::Argument;
pub use self::context::function::intrinsic::Intrinsic as IntrinsicFunction;
pub use self::context::function::r#return::Return as FunctionReturn;
pub use self::context::function::runtime::Runtime;
pub use self::context::function::Function;
pub use self::context::optimizer::Optimizer;
pub use self::context::r#loop::Loop;
pub use self::context::Context;
pub use self::dump_flag::DumpFlag;

///
/// Implemented by items which are translated into LLVM IR.
///
#[allow(clippy::upper_case_acronyms)]
pub trait WriteLLVM<D>
where
    D: Dependency,
{
    ///
    /// Translates the entity into LLVM IR.
    ///
    fn into_llvm(self, context: &mut Context<D>) -> anyhow::Result<()>;
}

///
/// Implemented by items managing project dependencies.
///
pub trait Dependency {
    ///
    /// Compiles a project dependency.
    ///
    fn compile(
        &mut self,
        name: &str,
        parent_name: &str,
        optimization_level_middle: inkwell::OptimizationLevel,
        optimization_level_back: inkwell::OptimizationLevel,
        dump_flags: Vec<DumpFlag>,
    ) -> anyhow::Result<String>;

    ///
    /// Resolves a library address.
    ///
    fn resolve_library(&self, path: &str) -> anyhow::Result<String>;
}
