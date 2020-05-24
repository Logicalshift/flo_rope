use super::rope_trait::*;
use super::rope_action::*;

use std::ops::{Range};

///
/// A rope that can be edited by the user
///
pub trait RopeMut : Rope {
    ///
    /// Performs the specified editing action to this rope
    ///
    fn edit(&mut self, action: RopeAction<Self::Cell, Self::Attribute>);

    ///
    /// Replaces a range of cells. The attributes applied to the new cells will be the same
    /// as the attributes that were applied to the first cell in the replacement range
    ///
    fn replace<NewCells: IntoIterator<Item=Self::Cell>>(&mut self, range: Range<usize>, new_cells: NewCells) {
        self.edit(RopeAction::Replace(range, new_cells.into_iter().collect()));
    }

    ///
    /// Sets the attributes for a range of cells
    ///
    fn set_attributes(&mut self, range: Range<usize>, new_attributes: Self::Attribute) {
        self.edit(RopeAction::SetAttributes(range, new_attributes));
    }

    ///
    /// Replaces a range of cells and sets the attributes for them.
    ///
    fn replace_attributes<NewCells: IntoIterator<Item=Self::Cell>>(&mut self, range: Range<usize>, new_cells: NewCells, new_attributes: Self::Attribute) {
        self.edit(RopeAction::ReplaceAttributes(range, new_cells.into_iter().collect(), new_attributes));
    }
}
