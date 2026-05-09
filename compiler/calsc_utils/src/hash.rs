use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

#[macro_export]
macro_rules! fnvhash {
    ($lit:expr) => {
        $lit.hash_fnv_1a()
    };
}

pub trait FNV1aHash {
    /// Hashes the given object using the FNV-1a algorithm at compile time
    ///
    /// # Example
    /// let hash = "test".hash_fnv_1a();
    fn hash_fnv_1a(&self) -> u64;
}

impl<S: AsRef<OsStr>> FNV1aHash for S {
    fn hash_fnv_1a(&self) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        let mut i = 0;

        let bytes = self.as_ref().as_bytes();

        while i < bytes.len() {
            hash ^= bytes[i] as u64;
            hash = hash.wrapping_mul(0x100000001b3);
            i += 1;
        }

        hash
    }
}
