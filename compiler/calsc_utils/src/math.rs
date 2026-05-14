//! Math related utilities

/// Represents every possible math operation possible in Quickfall
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
pub struct MathOperator {
    pub operation: MathOperation,
    pub fast: bool,
}

impl MathOperator {
    pub fn new(operation: MathOperation, fast: bool) -> Self {
        Self { operation, fast }
    }
}
