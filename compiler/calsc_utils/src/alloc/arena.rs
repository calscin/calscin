//! Arena allocator definitions

use std::{fmt::Debug, ops::Deref};

/// An arena allocator. Handles storing elements and handing out a reference index
///
/// https://en.wikipedia.org/wiki/Region-based_memory_management
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct ArenaAllocator<V> {
    pub arena: Vec<V>,
}

#[derive(Clone)]
pub struct ArenaAllocatorKey<V: 'static> {
    arena_ref: &'static ArenaAllocator<V>, // This is supposed to be okay since normally the arena allocator doesn't move
    pub ind: usize,                        // The index inside of the arena allocator
}

impl<V: 'static> ArenaAllocator<V> {
    /// Creates a new instance of an [`ArenaAllocator`]
    pub const fn new() -> Self {
        Self { arena: vec![] }
    }

    /// Creates a new [`ArenaAllocator`] with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: Vec::with_capacity(capacity),
        }
    }

    /// Appends a new element of type `K` inside of the Arena allocator and hands out a given reference index.
    pub fn append(&mut self, item: V) -> ArenaAllocatorKey<V> {
        let reference = self.arena.len();

        self.arena.push(item);

        ArenaAllocatorKey {
            arena_ref: unsafe { std::mem::transmute::<&Self, &'static Self>(self) }, // This is generally safe since the arena allocator doesn't move
            ind: reference,
        }
    }
}

impl<V: Clone> ArenaAllocator<V> {
    /// Clones the stored object of the given reference index
    pub fn get_cloned(&self, refer: usize) -> V {
        self.arena[refer].clone()
    }
}

impl<V: 'static> Deref for ArenaAllocatorKey<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.arena_ref.arena[self.ind]
    }
}

impl<V: 'static + Debug> Debug for ArenaAllocatorKey<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.arena_ref.arena[self.ind].fmt(f)
    }
}

impl<V: 'static + PartialEq> PartialEq for ArenaAllocatorKey<V> {
    fn eq(&self, other: &Self) -> bool {
        self.arena_ref.arena[self.ind].eq(&other.arena_ref.arena[other.ind])
    }
}
