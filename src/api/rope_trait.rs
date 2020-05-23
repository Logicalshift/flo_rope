use std::ops::{Range};

///
/// Represents a read-only Rope data structure
///
pub trait Rope : Clone {
    /// A 'cell' or character in the rope. For a UTF-8 rope this could be `u8`, for xample
    type Cell: Clone;

    /// The type of an attribute in the rope. Every cell range has an attribute attached to it
    type Attribute: PartialEq+Clone+Default;

    /// The type used to iterate through ranges of cells in this rope
    type CellIterator: Iterator<Item=Self::Cell>;

    ///
    /// Returns the number of cells in this rope
    ///
    fn len(&self) -> usize;

    ///
    /// Reads the cell values for a range in this rope
    ///
    fn read_cells(&self, range: Range<usize>) -> Self::CellIterator;
}
