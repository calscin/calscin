//! Trait definitions for type convertions

/// The trait responsible for type transmutation convertions
pub trait TypeTransmutation {
    /// Can the given type transmute into the provided [`Self`] variant.
    fn can_transmute(&self, into: &Self) -> bool;

    /// Can the given type transmute into the provided [`Self`] variant taking
    /// into account that self is weakly typed
    fn can_transmute_weakly(&self, into: &Self) -> bool;
}

/// The trait responsible for type casting convertions.
pub trait TypeCasting {
    /// Can the given type cast into the provided [`Self`] variant.
    fn can_cast(&self, into: &Self) -> bool;
}
