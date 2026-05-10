//! The implementation of every diagnostic code inside of Calscin

pub mod errors;

#[macro_export]
macro_rules! declare_diagnostic {
    ($name: ident, $code: literal) => {
        pub const $name: usize = $code;
    };
}
