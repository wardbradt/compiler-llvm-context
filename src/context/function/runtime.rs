//!
//! The LLVM runtime functions.
//!

use inkwell::types::BasicType;

use crate::context::address_space::AddressSpace;

///
/// The LLVM runtime functions.
///
#[derive(Debug)]
pub struct Runtime<'ctx> {
    /// The personality function, used for exception handling.
    pub personality: inkwell::values::FunctionValue<'ctx>,
    /// The exception throwing function.
    pub cxa_throw: inkwell::values::FunctionValue<'ctx>,

    /// The `__addmod` runtime function.
    pub add_mod: inkwell::values::FunctionValue<'ctx>,
    /// The `__mulmod` runtime function.
    pub mul_mod: inkwell::values::FunctionValue<'ctx>,
    /// The `__signextend` runtime function.
    pub sign_extend: inkwell::values::FunctionValue<'ctx>,

    /// The `__sload` runtime function.
    pub storage_load: inkwell::values::FunctionValue<'ctx>,
    /// The `__sstore` runtime function.
    pub storage_store: inkwell::values::FunctionValue<'ctx>,

    /// The `__farcall` runtime function.
    pub far_call: inkwell::values::FunctionValue<'ctx>,
    /// The `__staticcall` runtime function.
    pub static_call: inkwell::values::FunctionValue<'ctx>,
    /// The `__delegatecall` runtime function.
    pub delegate_call: inkwell::values::FunctionValue<'ctx>,
    /// The `__mimiccall` runtime function.
    pub mimic_call: inkwell::values::FunctionValue<'ctx>,

    /// The `__nearcall` runtime function.
    pub near_call: inkwell::values::FunctionValue<'ctx>,
}

impl<'ctx> Runtime<'ctx> {
    /// The LLVM contract entry function name.
    pub const FUNCTION_ENTRY: &'static str = "__entry";

    /// The LLVM contract constructor function name.
    pub const FUNCTION_CONSTRUCTOR: &'static str = "__constructor";

    /// The LLVM contract selector function name.
    pub const FUNCTION_SELECTOR: &'static str = "__selector";

    /// The LLVM personality function name.
    pub const FUNCTION_PERSONALITY: &'static str = "__personality";

    /// The LLVM exception throwing function name.
    pub const FUNCTION_CXA_THROW: &'static str = "__cxa_throw";

    /// The `addmod` runtime function name.
    pub const FUNCTION_ADDMOD: &'static str = "__addmod";

    /// The `mulmod` runtime function name.
    pub const FUNCTION_MULMOD: &'static str = "__mulmod";

    /// The `signextend` runtime function name.
    pub const FUNCTION_SIGNEXTEND: &'static str = "__signextend";

    /// The `sload` runtime function name.
    pub const FUNCTION_SLOAD: &'static str = "__sload";

    /// The `sstore` runtime function name.
    pub const FUNCTION_SSTORE: &'static str = "__sstore";

    /// The `farcall` runtime function name.
    pub const FUNCTION_FARCALL: &'static str = "__farcall";

    /// The `staticcall` runtime function name.
    pub const FUNCTION_STATICCALL: &'static str = "__staticcall";

    /// The `delegatecall` runtime function name.
    pub const FUNCTION_DELEGATECALL: &'static str = "__delegatecall";

    /// The `mimiccall` runtime function name.
    pub const FUNCTION_MIMICCALL: &'static str = "__mimiccall";

    /// The `nearcall` runtime function name.
    pub const FUNCTION_NEARCALL: &'static str = "__nearcall";

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
                        .as_basic_type_enum();
                    3
                ]
                .as_slice(),
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        cxa_throw.add_attribute(
            inkwell::attributes::AttributeLoc::Function,
            llvm.create_enum_attribute(inkwell::LLVMAttributeKindCode::NoProfile, 0),
        );

        let add_mod = module.add_function(
            Self::FUNCTION_ADDMOD,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum();
                        3
                    ]
                    .as_slice(),
                    false,
                ),
            Some(inkwell::module::Linkage::External),
        );
        Self::apply_default_math(llvm, add_mod);
        let mul_mod = module.add_function(
            Self::FUNCTION_MULMOD,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum();
                        3
                    ]
                    .as_slice(),
                    false,
                ),
            Some(inkwell::module::Linkage::External),
        );
        Self::apply_default_math(llvm, mul_mod);
        let sign_extend = module.add_function(
            Self::FUNCTION_SIGNEXTEND,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum();
                        2
                    ]
                    .as_slice(),
                    false,
                ),
            Some(inkwell::module::Linkage::External),
        );
        Self::apply_default_math(llvm, sign_extend);

        let storage_load = module.add_function(
            Self::FUNCTION_SLOAD,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    vec![
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum();
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
                            .as_basic_type_enum();
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
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
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
                        .as_basic_type_enum(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum(),
                    external_call_result_type,
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
                        .as_basic_type_enum(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum(),
                    external_call_result_type,
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
                        .as_basic_type_enum(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum(),
                    external_call_result_type,
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
                        .as_basic_type_enum(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum(),
                    llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                        .as_basic_type_enum(),
                    external_call_result_type,
                ],
                false,
            ),
            Some(inkwell::module::Linkage::External),
        );
        let near_call = module.add_function(
            Self::FUNCTION_NEARCALL,
            llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                .fn_type(
                    &[
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .ptr_type(AddressSpace::Stack.into())
                            .as_basic_type_enum(),
                        llvm.custom_width_int_type(compiler_common::BITLENGTH_FIELD as u32)
                            .as_basic_type_enum(),
                    ],
                    true,
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
            static_call,
            delegate_call,
            mimic_call,

            near_call,
        }
    }

    ///
    /// Applies the default attribute set for the math function.
    ///
    fn apply_default_math(
        llvm: &'ctx inkwell::context::Context,
        function: inkwell::values::FunctionValue<'ctx>,
    ) {
        for attribute_kind in [
            inkwell::LLVMAttributeKindCode::MustProgress,
            inkwell::LLVMAttributeKindCode::NoUnwind,
            inkwell::LLVMAttributeKindCode::ReadNone,
            inkwell::LLVMAttributeKindCode::WillReturn,
        ]
        .into_iter()
        {
            function.add_attribute(
                inkwell::attributes::AttributeLoc::Function,
                llvm.create_enum_attribute(attribute_kind, 0),
            );
        }
    }
}
