use crate::api::*;

use std::ops::{Range};

///
/// Indicates a range of values that has been updated since the last pull from a rope
///
struct RopePendingChange {
    /// Where these values were originally in the rope
    original_range: Range<usize>,

    /// Where the replacement values appear in the updated rope
    new_range: Range<usize>
}

///
/// A pull rope will notify its function when changes are available and will gather changes into
/// a single batch when they're 'pulled' from the rope. This is useful in circumstances where
/// updates are scheduled but not performed immediately, for example when updating a UI. Pulling
/// changes only when the UI is ready to redraw will reduce the number of updates required to
/// end up with a representation of the most recent state of the rope.
///
pub struct PullRope<BaseRope, PullFn> 
where 
BaseRope:   RopeMut, 
PullFn:     Fn() -> () {
    /// The rope that this will pull changes from
    rope: BaseRope,

    /// A function that is called whenever the state of this rope changes from 'no changes' to 'changes waiting to be pulled'
    pull_fn: PullFn,

    /// The changes that have ocurred since the last time this rope was pulled from (kept in ascending order)
    changes: Vec<RopePendingChange>
}

impl<BaseRope, PullFn> PullRope<BaseRope, PullFn>
where 
BaseRope:   RopeMut, 
PullFn:     Fn() -> () {
    ///
    /// Creates a new pull rope from a base rope and a pull function
    /// 
    /// The base rope is used as storage for this pull rope, and the pull function is called whenever the state of
    /// the rope changes from 'no changes' to 'changes waiting to be pulled'
    ///
    pub fn from(rope: BaseRope, pull_fn: PullFn) -> PullRope<BaseRope, PullFn> {
        PullRope {
            rope:       rope,
            pull_fn:    pull_fn,
            changes:    vec![]
        }
    }

    ///
    /// Returns the index in the changes list that is either before or just after the specified position,
    /// along with the difference in position from the original at that point
    ///
    fn find_change(&self, pos: usize) -> (usize, i64) {
        let mut diff = 0;

        // Changes can only replace or insert, they can't move things around: this means that both
        // the 'old' and 'new' ranges will always be in order.
        for idx in 0..self.changes.len() {
            let change = &self.changes[idx];

            if change.new_range.start <= pos && change.new_range.end > pos {
                // Position is in the range of this change
                return (idx, diff);
            } else if change.new_range.start > pos {
                // We've passed the change
                return (idx, diff);
            }

            // Update the difference in position from this point
            let old_len = change.original_range.len() as i64;
            let new_len = change.new_range.len() as i64;

            diff += old_len - new_len;
        }

        // Change not found: must be beyond the end of the change range
        (self.changes.len(), diff)
    }
}

impl<BaseRope, PushFn> Rope for PullRope<BaseRope, PushFn>
where 
BaseRope:   RopeMut, 
PushFn:     Fn() -> () {
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
