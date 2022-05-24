//!
//! The LLVM optimizer settings.
//!

///
/// The LLVM optimizer settings.
///
#[derive(Debug, Clone)]
pub struct Settings {
    /// The middle-end optimization level.
    pub level_middle_end: inkwell::OptimizationLevel,
    /// The back-end optimization level.
    pub level_back_end: inkwell::OptimizationLevel,
    /// Whether to run the inliner.
    pub is_inliner_enabled: bool,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        level_middle_end: inkwell::OptimizationLevel,
        level_back_end: inkwell::OptimizationLevel,
        is_inliner_enabled: bool,
    ) -> Self {
        Self {
            level_middle_end,
            level_back_end,
            is_inliner_enabled,
        }
    }
}
