//!
//! The address space aliases.
//!

///
/// The address space aliases.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressSpace {
    /// The stack memory.
    Stack,
    /// The heap memory.
    Heap,
    /// The auxiliary heap memory.
    HeapAuxiliary,
    /// The generic memory page.
    Generic,
}

impl From<AddressSpace> for inkwell::AddressSpace {
    fn from(value: AddressSpace) -> Self {
        match value {
            AddressSpace::Stack => Self::Zero,
            AddressSpace::Heap => Self::One,
            AddressSpace::HeapAuxiliary => Self::Two,
            AddressSpace::Generic => Self::Three,
        }
    }
}
