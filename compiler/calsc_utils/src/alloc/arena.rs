//! Arena allocator definitions

use std::marker::PhantomData;

/// An arena allocator. Handles storing elements and handing out a reference index
///
/// https://en.wikipedia.org/wiki/Region-based_memory_management
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct ArenaAllocator<V, Key> {
    pub arena: Vec<V>,
    pub dummy: PhantomData<Key>,
}

pub type ArenaAllocatorReference = usize;

impl<V, Key: From<ArenaAllocatorReference> + Into<ArenaAllocatorReference>> ArenaAllocator<V, Key> {
    /// Creates a new instance of an [`ArenaAllocator`]
    pub const fn new() -> Self {
        Self {
            arena: vec![],
            dummy: PhantomData,
        }
    }

    /// Creates a new [`ArenaAllocator`] with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: Vec::with_capacity(capacity),
            dummy: PhantomData,
        }
    }

    /// Appends a new element of type `K` inside of the Arena allocator and hands out a given reference index.
    pub fn append(&mut self, item: V) -> Key {
        let reference = self.arena.len();

        self.arena.push(item);

        reference.into()
    }

    /// Gets the reference of the stored object of the given reference index as a reference
    pub fn get_static(&self, refer: Key) -> &'static V {
        unsafe { std::mem::transmute::<&V, &'static V>(&self.arena[refer.into()]) }
    }

    /// Gets the reference of the stored object of the given reference index as a reference
    pub fn get(&self, refer: Key) -> &V {
        &self.arena[refer.into()]
    }
}

impl<V: Clone, Key> ArenaAllocator<V, Key> {
    /// Clones the stored object of the given reference index
    pub fn get_cloned(&self, refer: ArenaAllocatorReference) -> V {
        self.arena[refer].clone()
    }
}
