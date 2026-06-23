//! The code that allows for convertions of types like A -> B.
//! There are two kinds of convertions of types in Calscin:
//! - Transmutations: near no runtime impact and are fully safe
//! - Casts: Potentially has a runtime impact or is unsafe.
//!
//! We seperate both to allow for users to explicitly know what the effects of the convertion will be.
//! Please note that type transmutations are not chained and will most likely not be as that can potentially make them
//! have an impact on runtime. Most of the time, only one transmutation is needed anyway

pub mod primitives;
pub mod traits;

pub use traits::*;

/// The kind of type convertion
pub enum ConvertionKind {
    Transmutation,
    Cast,
}
