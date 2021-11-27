use crate::api::*;

use std::marker::{PhantomData};

///
/// Given two streams of actions that represent the changes to the two halfs of a single
/// concatenated rope, generates a single stream of actions representing the combined
/// rope.
///
pub struct RopeConcatenator<Cell, Attribute> {
    cell: PhantomData<Cell>,
    attribute: PhantomData<Attribute>,

    /// The length of the left-hand side of the concatenated rope
    left_len: usize
}

impl<Cell, Attribute> RopeConcatenator<Cell, Attribute> {
    ///
    /// Creates a new concatenating rope. Initially both the left and the right-hand sides of
    /// the concatenated rope are considered to be empty.
    ///
    pub fn new() -> RopeConcatenator<Cell, Attribute>{
        RopeConcatenator {
            cell:       PhantomData,
            attribute:  PhantomData,
            left_len:   0
        }
    }

    ///
    /// Processes actions intended for the left-hand side of the rope, returning an iterator of the new actions
    ///
    pub fn send_left<'a, ActionIter: 'a+IntoIterator<Item=RopeAction<Cell, Attribute>>>(&'a mut self, items: ActionIter) -> impl 'a+Iterator<Item=RopeAction<Cell, Attribute>> {
        items.into_iter()
            .map(|item| {
                use RopeAction::*;

                // Get the range that was edited and its new length
                let (range, new_len) = match &item {
                    Replace(range, cells)                           => (range, cells.len()),
                    ReplaceAttributes(range, cells, _attributes)    => (range, cells.len()),
                    SetAttributes(range, _attributes)               => (range, range.len()),
                };

                // In debug builds, assert the actions are in the LHS of the rope
                debug_assert!(range.start <= self.left_len);
                debug_assert!(range.end <= self.left_len);

                // Update the length of the left-hand side of this rope
                if new_len > range.len() {
                    // Added extra items to the range
                    self.left_len += new_len - range.len();
                } else if new_len < range.len() {
                    // Removed items from the range
                    debug_assert!(range.len() - new_len <= self.left_len);
                    self.left_len -= range.len() - new_len;
                }

                // Item is passed through unchanged
                item
            })
    }

    ///
    /// Processes actions intended for the right-hand side of the rope, returning an iterator of the new actions
    ///
    pub fn send_right<'a, ActionIter: 'a+IntoIterator<Item=RopeAction<Cell, Attribute>>>(&'a mut self, items: ActionIter) -> impl 'a+Iterator<Item=RopeAction<Cell, Attribute>> {
        let left_len = self.left_len;

        // Adjust all of the actions by the length of the RHS of the rope
        items.into_iter()
            .map(move |item| {
                use RopeAction::*;

                match item {
                    Replace(range, cells)                       => Replace((range.start+left_len)..(range.end+left_len), cells),
                    ReplaceAttributes(range, cells, attributes) => ReplaceAttributes((range.start+left_len)..(range.end+left_len), cells, attributes),
                    SetAttributes(range, attributes)            => SetAttributes((range.start+left_len)..(range.end+left_len), attributes),
                }
            })
    }
}
