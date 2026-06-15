#[cfg(feature = "debug")]
use std::fmt::Debug;

use std::ops::Deref;

use crate::nodes::HIRNode;

#[must_use]
#[derive(Clone)]
pub struct HIRArenaReference {
    pub ind: usize,
    pub refer: &'static HIRNode,
}

impl From<(&'static HIRNode, usize)> for HIRArenaReference {
    fn from(value: (&'static HIRNode, usize)) -> Self {
        Self {
            refer: value.0,
            ind: value.1,
        }
    }
}

impl Deref for HIRArenaReference {
    type Target = HIRNode;

    fn deref(&self) -> &Self::Target {
        self.refer
    }
}

impl Debug for HIRArenaReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.refer.fmt(f)
    }
}
