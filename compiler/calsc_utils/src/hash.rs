use std::{hash::Hash, ops::Deref};

#[macro_export]
macro_rules! fnvhash {
    ($lit:expr) => {
        hash_fnv_1a($lit)
    };
}

/// A string that also contains it's hash to avoid recomputation.
/// Uses the **FNV-1a** algorithm to generate a hash
///
/// # Example
/// ```
/// use calsc_utils::hash;
///
/// let hash: hash::HashedString = hash::HashedString::new("test".to_string());
///
/// assert_eq!(hash.hash(), hash::hash_fnv_1a("test"))
/// ```
#[derive(Clone, Debug)]
pub struct HashedString {
    hash: u64,
    val: String,
}

pub const fn hash_fnv_1a(s: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    let mut i = 0;

    let bytes = s.as_bytes();

    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }

    hash
}

impl HashedString {
    /// Creates a new HashedString with the given value.
    /// Automatically calculates the hash at the creation.
    ///
    /// # Example
    /// ```
    /// use calsc_utils::hash;
    ///
    /// let hash: hash::HashedString = hash::HashedString::new("test".to_string());
    ///
    /// assert_eq!(hash.hash(), hash::hash_fnv_1a("test"))
    /// ```
    pub fn new(val: String) -> Self {
        Self {
            val: val.clone(),
            hash: hash_fnv_1a(&val),
        }
    }

    /// Modifies the held string object and automatically recomputes the hash inside based on the new value.
    ///
    /// # Example
    /// ```
    /// use calsc_utils::hash;
    ///
    /// let mut hash: hash::HashedString = hash::HashedString::new("test".to_string());
    ///
    /// assert_eq!(hash.hash(), hash::hash_fnv_1a("test"));
    /// hash.set("test2".to_string());
    /// assert_eq!(hash.hash(), hash::hash_fnv_1a("test2"));
    /// ```
    pub fn set(&mut self, new: String) {
        self.val = new;
        self.hash = fnvhash!(&self.val);
    }

    /// Returns the held hash of the string.
    ///
    /// # Example
    /// ```
    /// use calsc_utils::hash;
    ///
    /// let hash: hash::HashedString = hash::HashedString::new("test".to_string());
    ///
    /// assert_eq!(hash.hash(), hash::hash_fnv_1a("test"))
    /// ```
    pub fn hash(&self) -> u64 {
        self.hash
    }
}

impl Deref for HashedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl Hash for HashedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl Eq for HashedString {}

impl PartialEq for HashedString {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl From<&str> for HashedString {
    fn from(value: &str) -> Self {
        HashedString::new(value.to_string())
    }
}

impl From<String> for HashedString {
    fn from(value: String) -> Self {
        HashedString::new(value)
    }
}
