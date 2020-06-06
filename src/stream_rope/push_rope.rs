use crate::api::*;

use std::ops::{Range};

///
/// A push rope is a rope with a callback function where updates will be sent.
/// It can be used as an event source for cases where updates need to be
/// immediately reflected somewhere else.
///
#[derive(Clone)]
pub struct PushRope<BaseRope, PushFn> 
where 
BaseRope:   RopeMut, 
PushFn:     Clone+Fn(RopeAction<BaseRope::Cell, BaseRope::Attribute>) -> () {
    /// The rope that this will push updates for
    rope:       BaseRope,

    /// The function that updates will be pushed to
    push_fn:    PushFn
}

impl<BaseRope, PushFn> Rope for PushRope<BaseRope, PushFn>
where 
BaseRope:   RopeMut, 
PushFn:     Clone+Fn(RopeAction<BaseRope::Cell, BaseRope::Attribute>) -> () {
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

