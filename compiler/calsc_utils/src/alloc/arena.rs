pub struct ArenaAllocator<K> {
    arena: Vec<K>,
}

pub type ArenaAllocatorReference = usize;

impl<K> ArenaAllocator<K> {
    pub fn new() -> Self {
        Self { arena: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: Vec::with_capacity(capacity),
        }
    }

    pub fn append(&mut self, item: K) -> ArenaAllocatorReference {
        let reference = self.arena.len();

        self.arena.push(item);

        reference
    }

    pub fn get(&self, refer: ArenaAllocatorReference) -> &K {
        &self.arena[refer]
    }
}

impl<K: Clone> ArenaAllocator<K> {
    pub fn get_cloned(&self, refer: ArenaAllocatorReference) -> K {
        self.arena[refer].clone()
    }
}
