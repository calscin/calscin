use std::{
    backtrace::Backtrace,
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    thread::LocalKey,
};

/// A variant of [`RefCell`] that tracks active borrows
pub struct TrackedRefCell<T> {
    pub inner: RefCell<T>,
}

pub struct TrackedRef<'a, T> {
    origin: String,
    inner: Ref<'a, T>,
}

pub struct TrackedRefMut<'a, T> {
    origin: String,
    inner: RefMut<'a, T>,
}

impl<T> TrackedRefCell<T> {
    pub fn provide_imm_borrow<'a>(&self, re: Ref<'a, T>) -> TrackedRef<'a, T> {
        let origin = Backtrace::capture()
            .to_string()
            .lines()
            .take(10)
            .collect::<Vec<_>>()
            .join("\n");

        println!("+ Created immutable borrow at:\n {}", origin);

        TrackedRef { origin, inner: re }
    }

    pub fn provide_mut_borrow<'a>(&self, re: RefMut<'a, T>) -> TrackedRefMut<'a, T> {
        let origin = Backtrace::capture()
            .to_string()
            .lines()
            .take(10)
            .collect::<Vec<_>>()
            .join("\n");

        println!("+ Created mutable borrow at:\n {}", origin);

        TrackedRefMut { origin, inner: re }
    }
}

impl<T> TrackedRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    pub fn borrow(&self) -> TrackedRef<'_, T> {
        match self.inner.try_borrow() {
            Ok(v) => self.provide_imm_borrow(v),
            Err(_) => panic!("Tried attempting a borrow"),
        }
    }

    pub fn borrow_mut(&self) -> TrackedRefMut<'_, T> {
        match self.inner.try_borrow_mut() {
            Ok(v) => self.provide_mut_borrow(v),
            Err(_) => panic!("Tried attempting a mutable borrow"),
        }
    }
}

impl<T> Deref for TrackedRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Drop for TrackedRef<'_, T> {
    fn drop(&mut self) {
        println!("- Borrow at dropped at: \n{}", self.origin)
    }
}

impl<T> Drop for TrackedRefMut<'_, T> {
    fn drop(&mut self) {
        println!("- Mut Borrow at dropped at: \n{}", self.origin)
    }
}

impl<T> Deref for TrackedRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TrackedRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
