//! Comparing utilities

#[derive(Debug, Clone, PartialEq)]
pub enum ComparePredicate {
    Equal,       // a == b
    NotEqual,    // a != b
    GreaterThan, // a > b
    LowerThan,   // a < b
}

/// Represents an operator used for comparing
#[derive(Debug, Clone, PartialEq)]
pub struct CompareOperator {
    pub predicate: ComparePredicate,
    pub also_equal: bool,
}

impl CompareOperator {
    pub fn new_equal() -> Self {
        Self {
            predicate: ComparePredicate::Equal,
            also_equal: false,
        }
    }

    pub fn new_not_equal() -> Self {
        Self {
            predicate: ComparePredicate::NotEqual,
            also_equal: false,
        }
    }

    pub fn new(predicate: ComparePredicate, also_equal: bool) -> Self {
        Self {
            predicate,
            also_equal,
        }
    }
}
