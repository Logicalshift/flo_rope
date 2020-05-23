//!
//! `flo_rope` is an implementation of the rope data structure designed for FlowBetween.
//!
//! This implementation of the data structure supports strings of arbitrary `character` data
//! types (not just UTF-8 strings). It also supports attributes so it's suited for representing
//! text with formatting supplied. Finally, it has both a function and a streaming/command-based
//! API which makes it highly composable and suited for reactive-style applications.
//!

pub mod api;

pub use crate::api::*;
