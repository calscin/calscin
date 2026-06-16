//! Arena allocator definitions

use std::fmt::Debug;

/// An arena allocator. Handles storing elements and handing out a reference index
///
/// https://en.wikipedia.org/wiki/Region-based_memory_management
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct ArenaAllocator<T> {
    pub arena: Vec<T>,
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct ArenaHandle {
    index: usize,
}

impl<T> ArenaAllocator<T> {
    pub fn new() -> Self {
        Self { arena: vec![] }
    }

    pub fn append(&mut self, value: T) -> ArenaHandle {
        let idx = self.arena.len();

        self.arena.push(value);

        ArenaHandle { index: idx }
    }

    pub fn get(&self, handle: &ArenaHandle) -> &T {
        &self.arena[handle.index]
    }

    pub fn get_mut(&mut self, handle: &ArenaHandle) -> &mut T {
        &mut self.arena[handle.index]
    }
}
