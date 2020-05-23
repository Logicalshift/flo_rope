use std::ops::{Range};

///
/// The editing action that can be performed on a rope
///
#[derive(Clone, PartialEq, Debug)]
pub enum RopeAction<Cell, Attribute> {
    /// Removes a range of text from this rope
    Delete(Range<usize>),

    /// Replaces a range of text in this rope
    Replace(Range<usize>, Vec<Cell>),

    /// Sets the attributes for a range of text in this rope
    SetAttributes(Range<usize>, Attribute)
}
