//! Arena allocator definitions

use std::marker::PhantomData;

/// An arena allocator. Handles storing elements and handing out a reference index
///
/// https://en.wikipedia.org/wiki/Region-based_memory_management
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct ArenaAllocator<V> {
    pub arena: Vec<V>,
}

pub type ArenaAllocatorReference = usize;

impl<V> ArenaAllocator<V> {
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
    pub fn append(&mut self, item: V) -> &'static V {
        let reference = self.arena.len();

        self.arena.push(item);

        unsafe { std::mem::transmute::<&V, &'static V>(&self.arena[reference]) }
    }
}

impl<V: Clone> ArenaAllocator<V> {
    /// Clones the stored object of the given reference index
    pub fn get_cloned(&self, refer: ArenaAllocatorReference) -> V {
        self.arena[refer].clone()
    }
}
