use std::{fmt::Debug, mem};

pub enum NoCloneContainer<K> {
    Has(K),
    NoHas,
}

impl<K> NoCloneContainer<K> {
    pub fn new(val: K) -> Self {
        Self::Has(val)
    }

    pub fn empty() -> Self {
        Self::NoHas
    }

    pub fn get<'a>(&'a self) -> &'a K {
        match self {
            Self::Has(k) => k,
            _ => panic!(),
        }
    }

    pub fn get_mut<'a>(&'a mut self) -> &'a mut K {
        match self {
            Self::Has(k) => k,
            _ => panic!(),
        }
    }
}

impl<K: Default> NoCloneContainer<K> {
    pub fn take(&mut self) -> K {
        match self {
            Self::Has(k) => mem::replace(k, K::default()),
            Self::NoHas => panic!(),
        }
    }
}

impl<K> Clone for NoCloneContainer<K> {
    fn clone(&self) -> Self {
        match self {
            Self::NoHas => Self::NoHas,
            _ => panic!(),
        }
    }
}

impl<K: Debug> Debug for NoCloneContainer<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoHas => write!(f, "NoHas"),
            Self::Has(k) => write!(f, "Has({:#?})", k),
        }
    }
}
