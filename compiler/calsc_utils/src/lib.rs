//! Utilities to use inside of the Calscin compiler.

pub mod cmp;
pub mod fs;
pub mod hash;
pub mod math;
pub mod pos;
pub mod str;

/// Stores either a value of type [`A`] or a value of type [`B`]
pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> Either<A, B> {
    /// Creates a new [`Either`] value of type [`A`]
    ///
    /// # Example
    /// ```
    /// use calsc_utils::Either;
    ///
    /// let either: Either<String, i128> = Either::new_a("test".to_string());
    /// ```
    ///
    pub fn new_a(val: A) -> Self {
        Self::A(val)
    }

    /// Creates a new [`Either`] value of type [`B`]
    ///
    /// # Example
    /// ```
    /// use calsc_utils::Either;
    ///
    /// let either: Either<String, i128> = Either::new_b(1);
    /// ```
    ///
    pub fn new_b(val: B) -> Self {
        Self::B(val)
    }

    /// Unwraps the [`Either`] and consumes it and returns the [`A`] value.
    ///
    /// # Panics
    /// Panics if the [`Either`] is not of type [`A`]
    ///     
    /// # Example
    /// ```
    /// use calsc_utils::Either;
    ///
    /// let either: Either<String, i128> = Either::new_a("test".to_string());
    /// let val = either.unwrap_a();
    /// ```
    pub fn unwrap_a(self) -> A {
        if let Self::A(val) = self {
            val
        } else {
            panic!("Unwrapped A on a non A Either")
        }
    }

    /// Unwraps the [`Either`] and consumes it and returns the [`B`] value.
    ///
    /// # Panics
    /// Panics if the [`Either`] is not of type [`B`]
    ///     
    /// # Example
    /// ```
    /// use calsc_utils::Either;
    ///
    /// let either: Either<String, i128> = Either::new_b(1);
    /// let val = either.unwrap_b();
    /// ```
    pub fn unwrap_b(self) -> B {
        if let Self::B(val) = self {
            val
        } else {
            panic!("Unwrapped B on a non B Either")
        }
    }

    pub fn is_a(&self) -> bool {
        if let Self::A(_) = self { true } else { false }
    }

    pub fn is_b(&self) -> bool {
        !self.is_a()
    }
}

impl<A: Clone, B: Clone> Clone for Either<A, B> {
    fn clone(&self) -> Self {
        match self {
            Self::A(val) => Self::A(val.clone()),
            Self::B(val) => Self::B(val.clone()),
        }
    }
}
