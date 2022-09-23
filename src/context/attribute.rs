//!
//! The LLVM attribute.
//!

///
/// The LLVM attribute.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute {
    /// The eponymous LLVM attribute.
    AlwaysInline = 1,
    /// The eponymous LLVM attribute.
    ArgMemOnly = 2,
    /// The eponymous LLVM attribute.
    Builtin = 3,
    /// The eponymous LLVM attribute.
    Cold = 4,
    /// The eponymous LLVM attribute.
    Convergent = 5,
    /// The eponymous LLVM attribute.
    Hot = 6,
    /// The eponymous LLVM attribute.
    ImmArg = 7,
    /// The eponymous LLVM attribute.
    InReg = 8,
    /// The eponymous LLVM attribute.
    InaccessibleMemOnly = 9,
    /// The eponymous LLVM attribute.
    InaccessibleMemOrArgMemOnly = 10,
    /// The eponymous LLVM attribute.
    InlineHint = 11,
    /// The eponymous LLVM attribute.
    JumpTable = 12,
    /// The eponymous LLVM attribute.
    MinSize = 13,
    /// The eponymous LLVM attribute.
    MustProgress = 14,
    /// The eponymous LLVM attribute.
    Naked = 15,
    /// The eponymous LLVM attribute.
    Nest = 16,
    /// The eponymous LLVM attribute.
    NoAlias = 17,
    /// The eponymous LLVM attribute.
    NoBuiltin = 18,
    /// The eponymous LLVM attribute.
    NoCallback = 19,
    /// The eponymous LLVM attribute.
    NoCapture = 20,
    /// The eponymous LLVM attribute.
    NoCfCheck = 21,
    /// The eponymous LLVM attribute.
    NoDuplicate = 22,
    /// The eponymous LLVM attribute.
    NoFree = 23,
    /// The eponymous LLVM attribute.
    NoImplicitFloat = 24,
    /// The eponymous LLVM attribute.
    NoInline = 25,
    /// The eponymous LLVM attribute.
    NoMerge = 26,
    /// The eponymous LLVM attribute.
    NoProfile = 27,
    /// The eponymous LLVM attribute.
    NoRecurse = 28,
    /// The eponymous LLVM attribute.
    NoRedZone = 29,
    /// The eponymous LLVM attribute.
    NoReturn = 30,
    /// The eponymous LLVM attribute.
    NoSanitizeCoverage = 31,
    /// The eponymous LLVM attribute.
    NoSync = 32,
    /// The eponymous LLVM attribute.
    NoUndef = 33,
    /// The eponymous LLVM attribute.
    NoUnwind = 34,
    /// The eponymous LLVM attribute.
    NonLazyBind = 35,
    /// The eponymous LLVM attribute.
    NonNull = 36,
    /// The eponymous LLVM attribute.
    NullPointerIsValid = 37,
    /// The eponymous LLVM attribute.
    OptForFuzzing = 38,
    /// The eponymous LLVM attribute.
    OptimizeForSize = 39,
    /// The eponymous LLVM attribute.
    OptimizeNone = 40,
    /// The eponymous LLVM attribute.
    ReadNone = 41,
    /// The eponymous LLVM attribute.
    ReadOnly = 42,
    /// The eponymous LLVM attribute.
    Returned = 43,
    /// The eponymous LLVM attribute.
    ReturnsTwice = 44,
    /// The eponymous LLVM attribute.
    SExt = 45,
    /// The eponymous LLVM attribute.
    SafeStack = 46,
    /// The eponymous LLVM attribute.
    SanitizeAddress = 47,
    /// The eponymous LLVM attribute.
    SanitizeHWAddress = 48,
    /// The eponymous LLVM attribute.
    SanitizeMemTag = 49,
    /// The eponymous LLVM attribute.
    SanitizeMemory = 50,
    /// The eponymous LLVM attribute.
    SanitizeThread = 51,
    /// The eponymous LLVM attribute.
    ShadowCallStack = 52,
    /// The eponymous LLVM attribute.
    Speculatable = 53,
    /// The eponymous LLVM attribute.
    SpeculativeLoadHardening = 54,
    /// The eponymous LLVM attribute.
    StackProtect = 55,
    /// The eponymous LLVM attribute.
    StackProtectReq = 56,
    /// The eponymous LLVM attribute.
    StackProtectStrong = 57,
    /// The eponymous LLVM attribute.
    StrictFP = 58,
    /// The eponymous LLVM attribute.
    SwiftAsync = 59,
    /// The eponymous LLVM attribute.
    SwiftError = 60,
    /// The eponymous LLVM attribute.
    SwiftSelf = 61,
    /// The eponymous LLVM attribute.
    UWTable = 62,
    /// The eponymous LLVM attribute.
    WillReturn = 63,
    /// The eponymous LLVM attribute.
    WriteOnly = 64,
    /// The eponymous LLVM attribute.
    ZExt = 65,
    /// The eponymous LLVM attribute.
    ByRef = 66,
    /// The eponymous LLVM attribute.
    ByVal = 67,
    /// The eponymous LLVM attribute.
    ElementType = 68,
    /// The eponymous LLVM attribute.
    InAlloca = 69,
    /// The eponymous LLVM attribute.
    Preallocated = 70,
    /// The eponymous LLVM attribute.
    StructRet = 71,
    /// The eponymous LLVM attribute.
    Alignment = 72,
    /// The eponymous LLVM attribute.
    AllocSize = 73,
    /// The eponymous LLVM attribute.
    Dereferenceable = 74,
    /// The eponymous LLVM attribute.
    DereferenceableOrNull = 75,
    /// The eponymous LLVM attribute.
    StackAlignment = 76,
    /// The eponymous LLVM attribute.
    VScaleRange = 77,
}
