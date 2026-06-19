//! The fill pass of the stage 0
//! The goal of this pass is to fill the "empty" entries inside of the module tree for types and functions.
//! The fill phase also manages tracking of which modules are imported or not
//!
//! We use an append pass first in order to allow for all
