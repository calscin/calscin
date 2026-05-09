#[macro_export]
macro_rules! fnvhash {
    ($lit:expr) => {
        hash_fnv_1a($lit)
    };
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
