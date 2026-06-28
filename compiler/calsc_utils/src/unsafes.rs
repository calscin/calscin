pub struct UnsafeMut<K> {
    val: K,
}

impl<K> UnsafeMut<K> {
    pub fn new(val: K) -> Self {
        Self { val }
    }

    pub fn borrow(&self) -> &K {
        &self.val
    }

    pub fn borrow_mut(&self) -> &mut K {
        unsafe { ((&self.val) as *const K).cast_mut().as_mut().unwrap() }
    }
}
