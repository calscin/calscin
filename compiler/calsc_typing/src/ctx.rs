//! The context of the type system.

use crate::{TypingInterner, params::TypeParamCtx};

/// The typing context. Holds temporary / permanent information (eg: allocators).
/// This context should be passed instead of individual references as it doesn't add additional cost to pass.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct TypeCtx<'a> {
    pub type_params: TypeParamCtx,
    pub interner: &'a mut TypingInterner,
}

impl<'a> TypeCtx<'a> {
    pub fn new(interner: &'a mut TypingInterner) -> Self {
        Self {
            interner,
            type_params: TypeParamCtx::new(),
        }
    }
}
