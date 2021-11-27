//!
//! # Stream rope functions
//!
//! The functions in this module provide support functionality for streaming changes to
//! a rope to other parts of the application. They do not directly support the futures
//! stream (see the rope bindings in `flo_binding` for that functionality) but rather
//! provide the building blocks for implementing that functionality elsewhere.
//!

mod push_rope;
mod pull_rope;
#[cfg(test)] mod tests;

pub use self::push_rope::*;
pub use self::pull_rope::*;
