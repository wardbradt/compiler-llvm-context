//!
//! The LLVM optimizing tools.
//!

///
/// The LLVM optimizing tools.
///
#[derive(Debug)]
pub struct Optimizer<'ctx> {
    /// The optimization level.
    level: inkwell::OptimizationLevel,
    /// The module optimization pass manager.
    pass_manager_module: inkwell::passes::PassManager<inkwell::module::Module<'ctx>>,
    /// The function optimization pass manager.
    pass_manager_function: inkwell::passes::PassManager<inkwell::values::FunctionValue<'ctx>>,
}

impl<'ctx> Optimizer<'ctx> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        module: &inkwell::module::Module<'ctx>,
        optimization_level: inkwell::OptimizationLevel,
    ) -> Self {
        let internalize = matches!(optimization_level, inkwell::OptimizationLevel::Aggressive);
        let run_inliner = matches!(optimization_level, inkwell::OptimizationLevel::Aggressive);

        let pass_manager_builder = inkwell::passes::PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(optimization_level);
        pass_manager_builder.set_disable_unroll_loops(matches!(
            optimization_level,
            inkwell::OptimizationLevel::Aggressive
        ));

        let pass_manager_module = inkwell::passes::PassManager::create(());
        pass_manager_builder.populate_lto_pass_manager(
            &pass_manager_module,
            internalize,
            run_inliner,
        );
        pass_manager_builder.populate_module_pass_manager(&pass_manager_module);

        let pass_manager_function = inkwell::passes::PassManager::create(module);
        pass_manager_builder.populate_function_pass_manager(&pass_manager_function);

        Self {
            level: optimization_level,
            pass_manager_module,
            pass_manager_function,
        }
    }

    ///
    /// Returns the optimization level.
    ///
    pub fn level(&self) -> inkwell::OptimizationLevel {
        self.level
    }

    ///
    /// Runs the optimizations on `module`.
    ///
    pub fn run_on_module(&self, module: &inkwell::module::Module<'ctx>) {
        self.pass_manager_module.run_on(module);
    }

    ///
    /// Runs the optimizations on `function`.
    ///
    pub fn run_on_function(&self, function: inkwell::values::FunctionValue<'ctx>) {
        self.pass_manager_function.run_on(&function);
    }
}
