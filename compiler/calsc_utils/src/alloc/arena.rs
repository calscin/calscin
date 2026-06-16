//! Arena allocator definitions

use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

/// An arena allocator. Handles storing elements and handing out a reference index
///
/// https://en.wikipedia.org/wiki/Region-based_memory_management
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct ArenaAllocator<T> {
    pub arena: RefCell<Vec<T>>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ArenaHandle {
    pub index: usize,
}

pub struct ArenaRef<'a, T> {
    inner: Ref<'a, T>,
}

pub struct ArenaRefMut<'a, T> {
    inner: RefMut<'a, T>,
}

impl<T> ArenaAllocator<T> {
    pub fn new() -> Self {
        Self {
            arena: RefCell::new(vec![]),
        }
    }

    pub fn append(&self, value: T) -> ArenaHandle {
        let mut data = self.arena.borrow_mut();
        let idx = data.len();

        data.push(value);

        ArenaHandle { index: idx }
    }

    fn borrow(&self, idx: usize) -> Ref<T> {
        Ref::map(self.arena.borrow(), |v| &v[idx])
    }

    fn borrow_mut(&self, idx: usize) -> RefMut<T> {
        RefMut::map(self.arena.borrow_mut(), |v| &mut v[idx])
    }

    pub fn get(&self, handle: &ArenaHandle) -> ArenaRef<'_, T> {
        ArenaRef {
            inner: self.borrow(handle.index),
        }
    }

    pub fn get_mut(&self, handle: &ArenaHandle) -> ArenaRefMut<'_, T> {
        ArenaRefMut {
            inner: self.borrow_mut(handle.index),
        }
    }
}

impl<'a, T> Deref for ArenaRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> Deref for ArenaRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for ArenaRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Debug for ArenaHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node located at {}", self.index)
    }
}
