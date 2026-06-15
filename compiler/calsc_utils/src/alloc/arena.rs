//! Arena allocator definitions

use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    ops::Deref,
};

/// An arena allocator. Handles storing elements and handing out a reference index
///
/// https://en.wikipedia.org/wiki/Region-based_memory_management
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct ArenaAllocator<T> {
    pub arena: RefCell<Vec<T>>,
}

#[derive(Clone, Copy)]
pub struct ArenaHandle {
    index: usize,
}

pub struct ArenaRef<'a, T> {
    inner: Ref<'a, T>,
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

    pub fn get(&self, handle: ArenaHandle) -> ArenaRef<'_, T> {
        ArenaRef {
            inner: self.borrow(handle.index),
        }
    }
}

impl<'a, T> Deref for ArenaRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
