//! The append pass of stage 0.
//! The goal of this pass is to append "empty" entries inside of the module tree for types and functions.
//! The append phase also manages tracking of which modules are imported or not

pub mod types;
