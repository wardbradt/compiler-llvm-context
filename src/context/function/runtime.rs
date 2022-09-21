//!
//! The LLVM runtime functions.
//!

use inkwell::types::BasicType;

use crate::context::address_space::AddressSpace;
use crate::context::attribute::Attribute;

///
/// The LLVM runtime functions, implemented in the LLVM back-end.
/// The functions are automatically linked to the LLVM implementations if the signatures match.
///
#[derive(Debug)]
pub struct Runtime<'ctx> {
    /// The personality function, used for exception handling.
    pub personality: inkwell::values::FunctionValue<'ctx>,
    /// The exception throwing function.
    pub cxa_throw: inkwell::values::FunctionValue<'ctx>,

    /// The corresponding runtime function.
    pub add_mod: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub mul_mod: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub sign_extend: inkwell::values::FunctionValue<'ctx>,

    /// The corresponding runtime function.
    pub storage_load: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub storage_store: inkwell::values::FunctionValue<'ctx>,

    /// The corresponding runtime function.
    pub far_call: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub far_call_byref: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub system_far_call: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub system_far_call_byref: inkwell::values::FunctionValue<'ctx>,

    /// The corresponding runtime function.
    pub static_call: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub static_call_byref: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub system_static_call: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub system_static_call_byref: inkwell::values::FunctionValue<'ctx>,

    /// The corresponding runtime function.
    pub delegate_call: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub delegate_call_byref: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub system_delegate_call: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub system_delegate_call_byref: inkwell::values::FunctionValue<'ctx>,

    /// The corresponding runtime function.
    pub mimic_call: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub mimic_call_byref: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub system_mimic_call: inkwell::values::FunctionValue<'ctx>,
    /// The corresponding runtime function.
    pub system_mimic_call_byref: inkwell::values::FunctionValue<'ctx>,
}

impl<'ctx> Runtime<'ctx> {
    /// The LLVM contract main entry function name.
    pub const FUNCTION_ENTRY: &'static str = "__entry";

    /// The LLVM contract deploy code function name.
    pub const FUNCTION_DEPLOY_CODE: &'static str = "__deploy";

    /// The LLVM contract runtime code function name.
    pub const FUNCTION_RUNTIME_CODE: &'static str = "__runtime";

    /// The LLVM personality function name.
    pub const FUNCTION_PERSONALITY: &'static str = "__personality";

    /// The LLVM exception throwing function name.
    pub const FUNCTION_CXA_THROW: &'static str = "__cxa_throw";

    /// The corresponding runtime function name.
    pub const FUNCTION_ADDMOD: &'static str = "__addmod";

    /// The corresponding runtime function name.
    pub const FUNCTION_MULMOD: &'static str = "__mulmod";

    /// The corresponding runtime function name.
    pub const FUNCTION_SIGNEXTEND: &'static str = "__signextend";

    /// The corresponding runtime function name.
    pub const FUNCTION_SLOAD: &'static str = "__sload";

    /// The corresponding runtime function name.
    pub const FUNCTION_SSTORE: &'static str = "__sstore";

    /// The corresponding runtime function name.
    pub const FUNCTION_FARCALL: &'static str = "__farcall";

    /// The corresponding runtime function name.
    pub const FUNCTION_FARCALL_BYREF: &'static str = "__farcall_byref";

    /// The corresponding runtime function name.
    pub const FUNCTION_SYSTEM_FARCALL: &'static str = "__system_call";

    /// The corresponding runtime function name.
    pub const FUNCTION_SYSTEM_FARCALL_BYREF: &'static str = "__system_call_byref";

    /// The corresponding runtime function name.
    pub const FUNCTION_STATICCALL: &'static str = "__staticcall";

    /// The corresponding runtime function name.
    pub const FUNCTION_STATICCALL_BYREF: &'static str = "__staticcall_byref";

    /// The corresponding runtime function name.
    pub const FUNCTION_SYSTEM_STATICCALL: &'static str = "__system_staticcall";

    /// The corresponding runtime function name.
    pub const FUNCTION_SYSTEM_STATICCALL_BYREF: &'static str = "__system_staticcall_byref";

    /// The corresponding runtime function name.
    pub const FUNCTION_DELEGATECALL: &'static str = "__delegatecall";

    /// The corresponding runtime function name.
    pub const FUNCTION_DELEGATECALL_BYREF: &'static str = "__delegatecall_byref";

    /// The corresponding runtime function name.
    pub const FUNCTION_SYSTEM_DELEGATECALL: &'static str = "__system_delegatecall";

    /// The corresponding runtime function name.
    pub const FUNCTION_SYSTEM_DELEGATECALL_BYREF: &'static str = "__system_delegatecall_byref";

    /// The corresponding runtime function name.
    pub const FUNCTION_MIMICCALL: &'static str = "__mimiccall";

    /// The corresponding runtime function name.
    pub const FUNCTION_MIMICCALL_BYREF: &'static str = "__mimiccall_byref";

    /// The corresponding runtime function name.
    pub const FUNCTION_SYSTEM_MIMICCALL: &'static str = "__system_mimiccall";

    /// The corresponding runtime function name.
    pub const FUNCTION_SYSTEM_MIMICCALL_BYREF: &'static str = "__system_mimiccall_byref";

    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        llvm: &'ctx inkwell::context::Context,
        module: &inkwell::module::Module<'ctx>,
    ) -> Self {
        let personality = module.add_function(
            Self::FUNCTION_PERSONALITY,
            llvm.i32_type().fn_type(&[], false),
            None,
        );

        let cxa_throw = module.add_function(
            Self::FUNCTION_CXA_THROW,
            llvm.void_type().fn_type(
                vec![
                    llvm.i8_type()
                        .ptr_type(AddressSpace::Stack.into())
                        .as_basic_type_enum()
                        .into();
                    3
                ]
                .as_slice(),
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        cxa_throw.add_attribute(
            inkwell::attributes::AttributeLoc::Function,
            llvm.create_enum_attribute(Attribute::NoProfile as u32, 0),
        );

        let add_mod = module.add_function(
            Self::FUNCTION_ADDMOD,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum()
                            .into();
                        3
                    ]
                    .as_slice(),
                    false,
                ),
            Some(inkwell::module::Linkage::External),
        );
        Self::apply_default_attributes(llvm, add_mod);
        let mul_mod = module.add_function(
            Self::FUNCTION_MULMOD,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum()
                            .into();
                        3
                    ]
                    .as_slice(),
                    false,
                ),
            Some(inkwell::module::Linkage::External),
        );
        Self::apply_default_attributes(llvm, mul_mod);
        let sign_extend = module.add_function(
            Self::FUNCTION_SIGNEXTEND,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum()
                            .into();
                        2
                    ]
                    .as_slice(),
                    false,
                ),
            Some(inkwell::module::Linkage::External),
        );
        Self::apply_default_attributes(llvm, sign_extend);

        let storage_load = module.add_function(
            Self::FUNCTION_SLOAD,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum()
                            .into();
                        1
                    ]
                    .as_slice(),
                    false,
                ),
            Some(inkwell::module::Linkage::External),
        );
        let storage_store = module.add_function(
            Self::FUNCTION_SSTORE,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum()
                            .into();
                        2
                    ]
                    .as_slice(),
                    false,
                ),
            Some(inkwell::module::Linkage::External),
        );

        let external_call_result_type = llvm
            .struct_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum(),
                    llvm.bool_type().as_basic_type_enum(),
                ],
                false,
            )
            .ptr_type(AddressSpace::Stack.into())
            .as_basic_type_enum();

        let far_call = module.add_function(
            Self::FUNCTION_FARCALL,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let far_call_byref = module.add_function(
            Self::FUNCTION_FARCALL_BYREF,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let system_far_call = module.add_function(
            Self::FUNCTION_SYSTEM_FARCALL,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let system_far_call_byref = module.add_function(
            Self::FUNCTION_SYSTEM_FARCALL_BYREF,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );

        let static_call = module.add_function(
            Self::FUNCTION_STATICCALL,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let static_call_byref = module.add_function(
            Self::FUNCTION_STATICCALL_BYREF,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let system_static_call = module.add_function(
            Self::FUNCTION_SYSTEM_STATICCALL,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let system_static_call_byref = module.add_function(
            Self::FUNCTION_SYSTEM_STATICCALL_BYREF,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );

        let delegate_call = module.add_function(
            Self::FUNCTION_DELEGATECALL,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let delegate_call_byref = module.add_function(
            Self::FUNCTION_DELEGATECALL_BYREF,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let system_delegate_call = module.add_function(
            Self::FUNCTION_SYSTEM_DELEGATECALL,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let system_delegate_call_byref = module.add_function(
            Self::FUNCTION_SYSTEM_DELEGATECALL_BYREF,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );

        let mimic_call = module.add_function(
            Self::FUNCTION_MIMICCALL,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let mimic_call_byref = module.add_function(
            Self::FUNCTION_MIMICCALL_BYREF,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let system_mimic_call = module.add_function(
            Self::FUNCTION_SYSTEM_MIMICCALL,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let system_mimic_call_byref = module.add_function(
            Self::FUNCTION_SYSTEM_MIMICCALL_BYREF,
            external_call_result_type.fn_type(
                &[
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_BYTE as u32)
                        .ptr_type(AddressSpace::Generic.into())
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum()
                        .into(),
                    external_call_result_type.into(),
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );

        Self {
            personality,
            cxa_throw,

            add_mod,
            mul_mod,

            sign_extend,

            storage_load,
            storage_store,

            far_call,
            far_call_byref,
            system_far_call,
            system_far_call_byref,

            static_call,
            static_call_byref,
            system_static_call,
            system_static_call_byref,

            delegate_call,
            delegate_call_byref,
            system_delegate_call,
            system_delegate_call_byref,

            mimic_call,
            mimic_call_byref,
            system_mimic_call,
            system_mimic_call_byref,
        }
    }

    ///
    /// Modifies the external call function with `with_ptr` and `system` modifiers.
    ///
    pub fn modify(
        &self,
        function: inkwell::values::FunctionValue<'ctx>,
        is_byref: bool,
        is_system: bool,
    ) -> anyhow::Result<inkwell::values::FunctionValue<'ctx>> {
        let modified = if function == self.far_call {
            match (is_byref, is_system) {
                (false, false) => self.far_call,
                (false, true) => self.system_far_call,
                (true, false) => self.far_call,
                (true, true) => self.system_far_call_byref,
            }
        } else if function == self.static_call {
            match (is_byref, is_system) {
                (false, false) => self.static_call,
                (false, true) => self.system_static_call,
                (true, false) => self.static_call,
                (true, true) => self.system_static_call_byref,
            }
        } else if function == self.delegate_call {
            match (is_byref, is_system) {
                (false, false) => self.delegate_call,
                (false, true) => self.system_delegate_call,
                (true, false) => self.delegate_call,
                (true, true) => self.system_delegate_call_byref,
            }
        } else if function == self.mimic_call {
            match (is_byref, is_system) {
                (false, false) => self.mimic_call,
                (false, true) => self.system_mimic_call,
                (true, false) => self.mimic_call,
                (true, true) => self.system_mimic_call_byref,
            }
        } else {
            anyhow::bail!(
                "Cannot modify an external call function `{}`",
                function.get_name().to_string_lossy()
            );
        };

        Ok(modified)
    }

    ///
    /// Applies the default attribute set for the math function.
    ///
    fn apply_default_attributes(
        llvm: &'ctx inkwell::context::Context,
        function: inkwell::values::FunctionValue<'ctx>,
    ) {
        for attribute_kind in [
            Attribute::MustProgress,
            Attribute::NoUnwind,
            Attribute::ReadNone,
            Attribute::WillReturn,
        ]
        .into_iter()
        {
            function.add_attribute(
                inkwell::attributes::AttributeLoc::Function,
                llvm.create_enum_attribute(attribute_kind as u32, 0),
            );
        }
    }
}
