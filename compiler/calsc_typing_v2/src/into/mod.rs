//! The code that allows for convertions of types like A -> B.
//! There are two kinds of convertions of types in Calscin:
//! - Transmutations: near no runtime impact and are fully safe
//! - Casts: Potentially has a runtime impact or is unsafe.
//!
//! We seperate both to allow for users to explicitly know what the effects of the convertion will be.
