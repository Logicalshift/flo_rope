//!
//! `flo_rope` is an implementation of the rope data structure designed for FlowBetween.
//! 
//! This is a Rust library containing an implementation of the rope data structure.
//! Ropes are an extension of the string type that support efficient manipulation of
//! very large amounts of data.
//! 
//! `flo_rope` adds a couple of extensions to the usual features of the data structure:
//! 
//!  * It allows for ropes of any 'cell' type so it's not just a replacement for 
//!    character strings but also vector types with the same editing properties
//!  * It supports attaching attributes to regions in the rope. This makes it suitable
//!    for tasks like representing text with attached formatting information.
//!  * It supplies a 'push' and 'pull' model for mirroring changes to a rope elsewhere.
//!    (for example, to update a user interface in response to changes to the rope)
//! 
//! Ropes are quite typically used for text editors for their performance 
//! characteristics: `flo_rope` provides a way to represent text colours and set up a 
//! pipeline to use those colours to perform syntax highlighting. The 'pull' model is
//! highly suited to representing text regions (and other collection data structures) 
//! in a reactive user interface.
//! 
//! A companion library, `flo_binding` will provide support for the reactive model of 
//! ropes, suitable for integrating into a user interface.
//! 
//! ## Examples
//! 
//! Replacing a range in a rope
//! 
//! ```
//! use flo_rope::*;
//! 
//! let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
//! 
//! rope.replace(1..3, vec![42]);
//! assert!(rope.read_cells(0..rope.len()).cloned().collect::<Vec<_>>() == vec![1,42,4,5,6,7,8]);
//! ```

pub mod api;
pub mod rope;
pub mod stream_rope;

pub use crate::api::*;
pub use crate::rope::*;
pub use crate::stream_rope::*;
