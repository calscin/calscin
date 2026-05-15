//! Math related utilities

/// Represents every possible math operation possible in Calscin
#[derive(Clone, Debug, PartialEq)]
pub enum MathOperation {
    /// +
    Add,

    /// -
    Sub,

    /// *
    Mul,

    /// /
    Div,

    /// \
    Mod,

    /// --
    Or,

    /// ++
    And,

    /// !!
    Xor,

    /// ??
    Nor,

    /// **
    ShiftLeft,

    /// //
    ShiftRight,
}

/// Represents a full math operator
#[derive(Clone, Debug, PartialEq)]
pub struct MathOperator {
    pub operation: MathOperation,
    pub fast: bool,
    pub assigns: bool,
}

impl MathOperator {
    pub fn new(operation: MathOperation, fast: bool, assigns: bool) -> Self {
        Self {
            operation,
            fast,
            assigns,
        }
    }
}
