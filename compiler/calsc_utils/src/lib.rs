//! Utilities to use inside of the Calscin compiler.

use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};

pub mod alloc;
pub mod cmp;
pub mod fs;
pub mod hash;
pub mod math;
pub mod path;
pub mod pos;
pub mod refcell;
pub mod str;
pub mod unsafes;

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

pub struct DisplayWithWrapper<'a, K: Clone, T: DisplayWith<K>> {
    k: K,
    t: &'a T,
    marker: PhantomData<T>,
}

impl<K: Clone, T: DisplayWith<K>> Display for DisplayWithWrapper<'_, K, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.t.fmt(self.k.clone(), f)
    }
}

pub trait DisplayWith<K: Clone> {
    fn fmt(&self, k: K, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub fn display_with_to_string<K: Clone, T: DisplayWith<K>>(dw: &T, k: K) -> String {
    let wrapper = DisplayWithWrapper {
        k,
        t: dw,
        marker: PhantomData,
    };

    format!("{}", wrapper)
}

pub fn display_with_list<K: Clone, T: DisplayWith<K>>(dw: &[T], k: K) -> String {
    let mut str = display_with_to_string(&dw[0], k.clone());

    for i in 1..dw.len() {
        str += ",";
        str += &display_with_to_string(&dw[i], k.clone());
    }

    str
}

pub fn vec_contains<K: PartialEq>(vec: &[K], elem: &K) -> Option<usize> {
    for i in 0..vec.len() {
        if &vec[i] == elem {
            return Some(i);
        }
    }

    None
}
