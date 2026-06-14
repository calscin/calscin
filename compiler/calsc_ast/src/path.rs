use std::fmt::Display;

use calsc_utils::hash::HashedString;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ElementPath {
    pub members: Vec<HashedString>,
}

impl ElementPath {
    pub fn everything_but_last(&self) -> ElementPath {
        ElementPath {
            members: self.members[0..self.members.len() - 1].to_vec(),
        }
    }

    pub fn last(&self) -> HashedString {
        self.members[self.members.len() - 1].clone()
    }
}

impl Display for ElementPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for path in &self.members {
            write!(f, "::{}", path)?;
        }

        Ok(())
    }
}
