//!
//! The LLVM context library.
//!

pub(crate) mod context;
pub(crate) mod dump_flag;

pub use self::context::address_space::AddressSpace;
pub use self::context::argument::Argument;
pub use self::context::function::intrinsic::Intrinsic as IntrinsicFunction;
pub use self::context::function::r#return as FunctionReturn;
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
    fn compile(&mut self, name: &str);
}

// ///
// /// Compiles the dependency object.
// ///
// pub fn compile_dependency(&mut self, module_name: &str) -> Option<String> {
//     let contract_path = self
//         .project
//         .contracts
//         .iter()
//         .find_map(|(path, contract)| {
//             if contract.object.identifier.as_str() == module_name {
//                 Some(path.to_owned())
//             } else {
//                 None
//             }
//         })
//         .unwrap_or_else(|| panic!("Dependency `{}` not found", module_name));
//
//     let hash = self
//         .project
//         .compile(
//             contract_path.as_str(),
//             self.optimizer.level(),
//             self.dump_flags.as_slice(),
//         )
//         .unwrap_or_else(|error| {
//             panic!("Dependency `{}` compiling error: {:?}", module_name, error)
//         });
//
//     self.project
//         .contracts
//         .iter_mut()
//         .find_map(|(_path, contract)| {
//             if contract.object.identifier == self.module().get_name() {
//                 Some(contract)
//             } else {
//                 None
//             }
//         })
//         .as_mut()?
//         .insert_factory_dependency(hash.clone(), contract_path);
//
//     Some(hash)
// }

// ///
// /// Gets a deployed library address.
// ///
// pub fn get_library_address(&self, path: &str) -> Option<inkwell::values::IntValue<'ctx>> {
//     for (file_path, contracts) in self.project.libraries.iter() {
//         for (contract_name, address) in contracts.iter() {
//             let key = format!("{}:{}", file_path, contract_name);
//             if key.as_str() == path {
//                 return Some(self.field_const_str(&address["0x".len()..]));
//             }
//         }
//     }
//
//     None
// }
