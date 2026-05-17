//! Arena allocator definitions

/// An arena allocator. Handles storing elements and handing out a reference index
///
/// https://en.wikipedia.org/wiki/Region-based_memory_management
pub struct ArenaAllocator<K> {
    arena: Vec<K>,
}

pub type ArenaAllocatorReference = usize;

impl<K> ArenaAllocator<K> {
    /// Creates a new instance of an [`ArenaAllocator`]
    pub fn new() -> Self {
        Self { arena: vec![] }
    }

    /// Creates a new [`ArenaAllocator`] with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: Vec::with_capacity(capacity),
        }
    }

    /// Appends a new element of type `K` inside of the Arena allocator and hands out a given reference index.
    pub fn append(&mut self, item: K) -> ArenaAllocatorReference {
        let reference = self.arena.len();

        self.arena.push(item);

        reference
    }

    /// Gets the reference of the stored object of the given reference index as a reference
    pub fn get(&self, refer: ArenaAllocatorReference) -> &K {
        &self.arena[refer]
    }
}

impl<K: Clone> ArenaAllocator<K> {
    /// Clones the stored object of the given reference index
    pub fn get_cloned(&self, refer: ArenaAllocatorReference) -> K {
        self.arena[refer].clone()
    }
}
