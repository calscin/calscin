//! Convertion definitions for HIR nodes

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_expected_error, build_unexpected_error},
};
use calsc_typing::{TransmutableType, tree::Type};

use crate::{
    localctx::LocalContext,
    nodes::{HIRNode, HIRNodeKind},
};

impl HIRNode {
    /// Uses the node as a value of the given [`Type`]. Will potentially transmute the value if possible.
    ///
    /// # Error
    /// This function will error if the casting fails at any point or if the types cannot be transmuted.
    ///
    pub fn use_as<K: DiagnosticSource>(
        &self,
        ty: Type,
        local_ctx: Option<&LocalContext>,
    ) -> DiagResult<HIRNode> {
        if self.get_type(local_ctx)?.is_none() {
            return Err(build_unexpected_error(&"void".to_string(), self).into());
        }

        let self_type = unsafe { self.get_type(local_ctx)?.unwrap_unchecked() };

        if self_type == ty {
            return Ok(self.clone());
        }

        if let HIRNodeKind::StructuredInit { .. } = self.kind.clone() {
            return convert_structured_init_into(self.clone(), ty, self);
        }

        if !self_type.can_transmute(ty.clone()) {
            return Err(build_expected_error(&ty, &self_type, self).into());
        }

        let node = HIRNode::new(
            HIRNodeKind::CastNode {
                original: self.clone().push(),
                into: ty,
            },
            self.start.clone(),
            self.end.clone(),
        );

        Ok(node)
    }
}

pub fn convert_structured_init_into<K: DiagnosticSource>(
    structured_init: HIRNode,
    ty: Type,
    origin: &K,
) -> DiagResult<HIRNode> {
    todo!()
}
