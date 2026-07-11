//! The context of the type system.

use crate::params::TypeParamCtx;

/// The typing context. Holds temporary / permanent information (eg: allocators).
/// This context should be passed instead of individual references as it doesn't add additional cost to pass.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct TypeCtx {
    pub type_params: TypeParamCtx,
}

impl TypeCtx {
    pub fn new() -> Self {
        Self {
            type_params: TypeParamCtx::new(),
        }
    }
}
