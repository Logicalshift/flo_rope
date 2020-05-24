use std::ops::{Range};

///
/// The editing action that can be performed on a rope
///
#[derive(Clone, PartialEq, Debug)]
pub enum RopeAction<Cell, Attribute> {
    /// Replaces a range of text in this rope. The attributes applied to 
    /// the new text will be the same as those applied to the first character 
    /// in the replaced range.
    Replace(Range<usize>, Vec<Cell>),

    /// Sets the attributes for a range of text in this rope
    SetAttributes(Range<usize>, Attribute),

    /// Sets both the attributes and the value for a range of cells
    ReplaceAttributes(Range<usize>, Vec<Cell>, Attribute)
}
