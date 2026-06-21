//! The implementation of every diagnostic code inside of Calscin

pub mod errors;
pub mod warnings;

#[macro_export]
macro_rules! declare_diagnostic {
    ($name: ident, $code: literal) => {
        pub const $name: usize = $code;
    };
}
