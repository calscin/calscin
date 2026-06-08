use rand::{RngExt, distr::Alphanumeric};

pub mod body;
pub mod control;
pub mod funcs;
pub mod result;
pub mod types;
pub mod values;
pub mod vars;

pub fn generate_block_seed() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
