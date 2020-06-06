use std::ops::{Range};

///
/// Represents a read-only Rope data structure
///
pub trait Rope {
    /// A 'cell' or character in the rope. For a UTF-8 rope this could be `u8`, for xample
    type Cell: Clone;

    /// The type of an attribute in the rope. Every cell range has an attribute attached to it
    type Attribute: PartialEq+Clone+Default;

    ///
    /// Returns the number of cells in this rope
    ///
    fn len(&self) -> usize;

    ///
    /// Reads the cell values for a range in this rope
    ///
    fn read_cells<'a>(&'a self, range: Range<usize>) -> Box<dyn 'a+Iterator<Item=&Self::Cell>>;

    ///
    /// Returns the attributes set at the specified location and their extent
    ///
    fn read_attributes<'a>(&'a self, pos: usize) -> (&'a Self::Attribute, Range<usize>);
}
