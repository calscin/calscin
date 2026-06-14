use std::fmt::Display;

use calsc_utils::hash::HashedString;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ElementPath {
    pub members: Vec<HashedString>,
}

impl Display for ElementPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for path in &self.members {
            write!(f, "::{}", path)?;
        }

        Ok(())
    }
}
