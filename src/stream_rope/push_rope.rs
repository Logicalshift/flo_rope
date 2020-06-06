use crate::api::*;

use std::ops::{Range};

///
/// A push rope is a rope with a callback function where updates will be sent.
/// It can be used as an event source for cases where updates need to be
/// immediately reflected somewhere else.
///
/// In order to reduce the amount of data that's copied, the PushBeforeRope
/// will only send updates before they're applied to the rope.
///
#[derive(Clone)]
pub struct PushBeforeRope<BaseRope, PushFn> 
where 
BaseRope:   RopeMut, 
PushFn:     Fn(&RopeAction<BaseRope::Cell, BaseRope::Attribute>) -> () {
    /// The rope that this will push updates for
    rope:       BaseRope,

    /// The function that updates will be pushed to
    push_fn:    PushFn
}

impl<BaseRope, PushFn> Rope for PushBeforeRope<BaseRope, PushFn>
where 
BaseRope:   RopeMut, 
PushFn:     Fn(&RopeAction<BaseRope::Cell, BaseRope::Attribute>) -> () {
    /// A 'cell' or character in the rope. For a UTF-8 rope this could be `u8`, for xample
    type Cell = BaseRope::Cell;

    /// The type of an attribute in the rope. Every cell range has an attribute attached to it
    type Attribute = BaseRope::Attribute;

    ///
    /// Returns the number of cells in this rope
    ///
    #[inline]
    fn len(&self) -> usize {
        self.rope.len()
    }

    ///
    /// Reads the cell values for a range in this rope
    ///
    #[inline]
    fn read_cells<'a>(&'a self, range: Range<usize>) -> Box<dyn 'a+Iterator<Item=&Self::Cell>> {
        self.rope.read_cells(range)
    }

    ///
    /// Returns the attributes set at the specified location and their extent
    ///
    #[inline]
    fn read_attributes<'a>(&'a self, pos: usize) -> (&'a Self::Attribute, Range<usize>) {
        self.rope.read_attributes(pos)
    }
}

impl<BaseRope, PushFn> RopeMut for PushBeforeRope<BaseRope, PushFn>
where 
BaseRope:   RopeMut, 
PushFn:     Fn(&RopeAction<BaseRope::Cell, BaseRope::Attribute>) -> () {
    ///
    /// Performs the specified editing action to this rope
    ///
    #[inline]
    fn edit(&mut self, action: RopeAction<Self::Cell, Self::Attribute>) {
        (self.push_fn)(&action);
        self.rope.edit(action);
    }
}

///
/// A push rope is a rope with a callback function where updates will be sent.
/// It can be used as an event source for cases where updates need to be
/// immediately reflected somewhere else.
///
/// The PushAfterRope generates its events after the base has been modified,
/// which can be more convenient but does require cloning the data that's
/// passed in.
///
#[derive(Clone)]
pub struct PushAfterRope<BaseRope, PushFn> 
where 
BaseRope:   RopeMut, 
PushFn:     Fn(RopeAction<BaseRope::Cell, BaseRope::Attribute>) -> () {
    /// The rope that this will push updates for
    rope:       BaseRope,

    /// The function that updates will be pushed to
    push_fn:    PushFn
}

impl<BaseRope, PushFn> Rope for PushAfterRope<BaseRope, PushFn>
where 
BaseRope:   RopeMut, 
PushFn:     Fn(RopeAction<BaseRope::Cell, BaseRope::Attribute>) -> () {
    /// A 'cell' or character in the rope. For a UTF-8 rope this could be `u8`, for xample
    type Cell = BaseRope::Cell;

    /// The type of an attribute in the rope. Every cell range has an attribute attached to it
    type Attribute = BaseRope::Attribute;

    ///
    /// Returns the number of cells in this rope
    ///
    #[inline]
    fn len(&self) -> usize {
        self.rope.len()
    }

    ///
    /// Reads the cell values for a range in this rope
    ///
    #[inline]
    fn read_cells<'a>(&'a self, range: Range<usize>) -> Box<dyn 'a+Iterator<Item=&Self::Cell>> {
        self.rope.read_cells(range)
    }

    ///
    /// Returns the attributes set at the specified location and their extent
    ///
    #[inline]
    fn read_attributes<'a>(&'a self, pos: usize) -> (&'a Self::Attribute, Range<usize>) {
        self.rope.read_attributes(pos)
    }
}

impl<BaseRope, PushFn> RopeMut for PushAfterRope<BaseRope, PushFn>
where 
BaseRope:   RopeMut, 
PushFn:     Fn(RopeAction<BaseRope::Cell, BaseRope::Attribute>) -> () {
    ///
    /// Performs the specified editing action to this rope
    ///
    #[inline]
    fn edit(&mut self, action: RopeAction<Self::Cell, Self::Attribute>) {
        self.rope.edit(action.clone());
        (self.push_fn)(action);
    }

    ///
    /// Replaces a range of cells. The attributes applied to the new cells will be the same
    /// as the attributes that were applied to the first cell in the replacement range
    ///
    fn replace<NewCells: IntoIterator<Item=Self::Cell>>(&mut self, range: Range<usize>, new_cells: NewCells) {
        let new_cells = new_cells.into_iter().collect::<Vec<_>>();

        self.rope.replace(range.clone(), new_cells.clone());
        (self.push_fn)(RopeAction::Replace(range, new_cells));
    }

    ///
    /// Sets the attributes for a range of cells
    ///
    fn set_attributes(&mut self, range: Range<usize>, new_attributes: Self::Attribute) {
        self.rope.set_attributes(range.clone(), new_attributes.clone());
        (self.push_fn)(RopeAction::SetAttributes(range, new_attributes));
    }

    ///
    /// Replaces a range of cells and sets the attributes for them.
    ///
    fn replace_attributes<NewCells: IntoIterator<Item=Self::Cell>>(&mut self, range: Range<usize>, new_cells: NewCells, new_attributes: Self::Attribute) {
        let new_cells = new_cells.into_iter().collect::<Vec<_>>();

        self.rope.replace_attributes(range.clone(), new_cells.clone(), new_attributes.clone());
        (self.push_fn)(RopeAction::ReplaceAttributes(range, new_cells, new_attributes));
    }
}
