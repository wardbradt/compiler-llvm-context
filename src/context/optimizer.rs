//!
//! The LLVM optimizing tools.
//!

///
/// The LLVM optimizing tools.
///
#[derive(Debug)]
pub struct Optimizer<'ctx> {
    /// The LLVM target machine.
    target_machine: inkwell::targets::TargetMachine,
    /// The middle-end optimization level.
    level_middle_end: inkwell::OptimizationLevel,
    /// The back-end optimization level.
    level_back_end: inkwell::OptimizationLevel,
    /// Whether to run the inliner.
    is_inliner_enabled: bool,

    /// The module optimization pass manager.
    pass_manager_module: Option<inkwell::passes::PassManager<inkwell::module::Module<'ctx>>>,
    /// The function optimization pass manager.
    pass_manager_function:
        Option<inkwell::passes::PassManager<inkwell::values::FunctionValue<'ctx>>>,
}

impl<'ctx> Optimizer<'ctx> {
    /// The LLVM target name.
    pub const VM_TARGET_NAME: &'static str = "syncvm";

    /// The actual production VM name.
    pub const VM_PRODUCTION_NAME: &'static str = "zkEVM";

    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        level_middle_end: inkwell::OptimizationLevel,
        level_back_end: inkwell::OptimizationLevel,
        run_inliner: bool,
    ) -> anyhow::Result<Self> {
        let target_machine = inkwell::targets::Target::from_name(Self::VM_TARGET_NAME)
            .ok_or_else(|| {
                anyhow::anyhow!("LLVM target machine `{}` not found", Self::VM_TARGET_NAME)
            })?
            .create_target_machine(
                &inkwell::targets::TargetTriple::create(Self::VM_TARGET_NAME),
                "",
                "",
                level_back_end,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "LLVM target machine `{}` initialization error",
                    Self::VM_TARGET_NAME
                )
            })?;

        Ok(Self {
            target_machine,
            level_middle_end,
            level_back_end,
            is_inliner_enabled: run_inliner,
            pass_manager_module: None,
            pass_manager_function: None,
        })
    }

    ///
    /// Sets the module which is to be optimized.
    ///
    pub fn set_module(&mut self, module: &inkwell::module::Module<'ctx>) {
        module.set_triple(&self.target_machine.get_triple());
        module.set_data_layout(&self.target_machine.get_target_data().get_data_layout());

        let pass_manager_builder = inkwell::passes::PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(self.level_middle_end);
        pass_manager_builder.set_disable_unroll_loops(matches!(
            self.level_middle_end,
            inkwell::OptimizationLevel::Aggressive
        ));

        let pass_manager_module = inkwell::passes::PassManager::create(());
        pass_manager_builder.populate_lto_pass_manager(
            &pass_manager_module,
            true,
            self.is_inliner_enabled,
        );
        pass_manager_builder.populate_module_pass_manager(&pass_manager_module);

        let pass_manager_function = inkwell::passes::PassManager::create(module);
        pass_manager_builder.populate_function_pass_manager(&pass_manager_function);

        self.pass_manager_module = Some(pass_manager_module);
        self.pass_manager_function = Some(pass_manager_function);
    }

    ///
    /// Returns the middle-end optimization level.
    ///
    pub fn level_middle_end(&self) -> inkwell::OptimizationLevel {
        self.level_middle_end
    }

    ///
    /// Returns the back-end optimization level.
    ///
    pub fn level_back_end(&self) -> inkwell::OptimizationLevel {
        self.level_back_end
    }

    ///
    /// Whether the inliner is enabled.
    ///
    pub fn is_inliner_enabled(&self) -> bool {
        self.is_inliner_enabled
    }

    ///
    /// Runs the optimizations on `module`.
    ///
    /// Only returns `true` if any of the passes modified the module.
    ///
    pub fn run_on_module(&self, module: &inkwell::module::Module<'ctx>) -> bool {
        self.pass_manager_module
            .as_ref()
            .expect("The module has not been set")
            .run_on(module)
    }

    ///
    /// Runs the optimizations on `function`.
    ///
    /// Only returns `true` if any of the passes modified the function.
    ///
    pub fn run_on_function(&self, function: inkwell::values::FunctionValue<'ctx>) -> bool {
        self.pass_manager_function
            .as_ref()
            .expect("The module has not been set")
            .run_on(&function)
    }

    ///
    /// Returns the target machine reference.
    ///
    pub fn target_machine(&self) -> &inkwell::targets::TargetMachine {
        &self.target_machine
    }
}
