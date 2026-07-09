/// A potentially empty mutable reference.
/// This is used to share mutable references safely to things that might clone.
/// This doesn't use any UB
#[derive(Debug)]
pub enum MutableReference<'a, T> {
    EmptyRef,
    HasRef(&'a mut T),
}

impl<'a, T> MutableReference<'a, T> {
    pub fn new(r: &'a mut T) -> Self {
        MutableReference::HasRef(r)
    }

    pub fn empty() -> Self {
        MutableReference::EmptyRef
    }

    pub fn get(&'a mut self) -> &'a mut T {
        match self {
            Self::EmptyRef => panic!("MutableReference called get but doesn't contain reference"),
            Self::HasRef(r) => *r,
        }
    }
}

impl<'a, T> Clone for MutableReference<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Self::EmptyRef => Self::EmptyRef,
            _ => panic!("Cannot clone an MutableReference::HasRef"),
        }
    }
}
