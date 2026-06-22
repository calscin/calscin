//! The intrinsics in Calscin.

pub enum Intrinsics {
    /// Allocates a specific amount of bytes in the stack
    /// `std::intrinsics::alloca`
    Alloca,

    /// Allocates a specific amount of bytes in the heap
    HeapAlloca,

    /// Frees a heap allocated pointer
    Free,
}
