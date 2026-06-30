use calsc_utils::alloc::arena::ArenaHandle;

pub enum TreeEntry {
    Type,
    Function,
    Module(ArenaHandle),
}
