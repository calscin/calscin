//! Math related utilities

/// Represents every possible math operation possible in Calscin
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq)]
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
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq)]
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
