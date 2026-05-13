use crate::container::dump_diagnostics;

pub trait CalscinResult<K, V> {
    /// Cleanly unwraps if [`Ok`].
    ///
    /// # Panics
    /// If [`Err`], then the function will cleanly dump all diagnostics then kill the program.
    fn unwrap_cleanly(self) -> K;
}

impl<K, V> CalscinResult<K, V> for Result<K, V> {
    fn unwrap_cleanly(self) -> K {
        if let Ok(v) = self {
            return v;
        }

        dump_diagnostics();

        panic!()
    }
}
