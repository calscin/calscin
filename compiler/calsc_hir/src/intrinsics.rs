//! The intrinsics in Calscin.

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub enum Intrinsics {
    /// Allocates a specific amount of bytes in the stack
    /// `std::intrinsics::alloca`
    Alloca,

    /// Allocates a specific amount of bytes in the heap
    /// `std::intrinsics::heap_alloca`
    HeapAlloca,

    /// Frees a heap allocated pointer
    /// `std::intrinsics::free`
    Free,

    /// Loads a pointer atomically
    /// `std::intrinsics::load_atomic`
    LoadAtomic,

    /// Stores into a pointer atomically
    /// `std::intrinsics::store_atomic`
    StoreAtomic,

    /// Selects a value based on the output of the condition
    /// `std::intrinsics::select`
    Select,

    /// Assumes a boolean value is true
    /// `std::intrinsics::assume`
    Assume,

    /// Assumes a place is unreachable
    /// `std::intrinsics::unreachable`
    Unreachable,
}
