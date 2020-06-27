use crate::api::*;

use std::mem;
use std::ops::{Range};

///
/// Indicates a range of values that has been updated since the last pull from a rope
///
struct RopePendingChange {
    /// Where these values were originally in the rope
    original_range: Range<usize>,

    /// Where the replacement values appear in the updated rope
    new_range: Range<usize>,

    /// True if the attributes for this range have changed
    changed_attributes: bool
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

    ///
    /// Marks a region as changed for the next pull request
    ///
    fn mark_change(&mut self, original_range: Range<usize>, new_length: usize, attribute_change: bool) {
        // Find the existing change corresponding to the start of the range
        let (mut change_idx, mut diff)  = self.find_change(original_range.start);
        let mut remaining_range         = original_range;
        let mut remaining_length        = new_length;

        loop {
            // If the index is beyond the end of the existing changes, then just add the edit range to the end
            if change_idx >= self.changes.len() {
                // Adjust the original range to match the new range
                let original_start  = (remaining_range.start as i64) + diff;
                let original_end    = (remaining_range.end as i64) + diff;
                let original_start  = original_start as usize;
                let original_end    = original_end as usize;

                self.changes.push(RopePendingChange {
                    original_range:     original_start..original_end,
                    new_range:          remaining_range.start..(remaining_range.start+remaining_length),
                    changed_attributes: attribute_change
                });

                break;
            } else if self.changes[change_idx].new_range.start <= remaining_range.start {
                // We overlap with an existing range
                self.changes[change_idx].changed_attributes = self.changes[change_idx].changed_attributes || attribute_change;
                let change = &self.changes[change_idx];

                let new_end = remaining_range.start + remaining_length;
                if new_end < change.new_range.end {
                    // New change is entirely within the existing change
                    let max_diff        = change.new_range.len() as i64;
                    let length_diff     = (remaining_range.len() as i64) - (remaining_length as i64);
                    let length_change   = max_diff.min(length_diff);

                    if length_diff != 0 {
                        // Adjust the length of the changed range
                        self.changes[change_idx].new_range.end = (self.changes[change_idx].new_range.end as i64 - length_change) as usize;

                        // Adjust the position of the following ranges
                        for move_idx in (change_idx+1)..self.changes.len() {
                            self.changes[move_idx].new_range.start  = (self.changes[move_idx].new_range.start as i64 - length_change) as usize;
                            self.changes[move_idx].new_range.end    = (self.changes[move_idx].new_range.end as i64 - length_change) as usize;
                        }
                    }

                    if length_change == length_diff {
                        // Entire length change was contained within the current change
                        break;
                    } else {
                        // Current range was too short to incorporate the entire change (trying to shrink a range by more than its overall size)
                        let change              = &self.changes[change_idx];
                        remaining_range.end     -= length_change as usize;

                        let old_len             = change.original_range.len() as i64;
                        let new_len             = change.new_range.len() as i64;

                        diff                    += old_len - new_len;
                        change_idx              += 1;
                    }
                } else {
                    // Continue with the following range
                    let used_length = change.new_range.end - remaining_range.start;

                    remaining_range.start   += used_length;
                    remaining_length        -= used_length;

                    // New range will be overlapping or before the next change
                    let old_len = change.original_range.len() as i64;
                    let new_len = change.new_range.len() as i64;

                    diff        += old_len - new_len;
                    change_idx  += 1;
                }
            } else {
                // The range does not overlap an existing range
                let next_range_start = if change_idx < self.changes.len() {
                    self.changes[change_idx].new_range.start
                } else {
                    usize::MAX
                };

                if next_range_start >= remaining_range.end {
                    // If the next range fits within the gap, then insert it and stop
                    let original_start  = (remaining_range.start as i64) + diff;
                    let original_end    = (remaining_range.end as i64) + diff;
                    let original_start  = original_start as usize;
                    let original_end    = original_end as usize;

                    self.changes.insert(change_idx, RopePendingChange {
                        original_range:     original_start..original_end,
                        new_range:          remaining_range.start..(remaining_range.start+remaining_length),
                        changed_attributes: false
                    });

                    // New change is entirely within the existing gap
                    let length_diff = (remaining_range.len() as i64) - (remaining_length as i64);

                    if length_diff != 0 {
                        // Adjust the position of the following ranges
                        for move_idx in (change_idx+1)..self.changes.len() {
                            self.changes[move_idx].new_range.start  = (self.changes[move_idx].new_range.start as i64 - length_diff) as usize;
                            self.changes[move_idx].new_range.end    = (self.changes[move_idx].new_range.end as i64 - length_diff) as usize;
                        }
                    }

                    break;
                } else {
                    // Fill in as much as possible from the gap and continue from here
                    let gap_length      = next_range_start - remaining_range.start;
                    let original_start  = (remaining_range.start as i64) + diff;
                    let original_start  = original_start as usize;
                    let gap_end         = original_start + gap_length;

                    if gap_length <= remaining_length {
                        // The new range covers the entire gap
                        self.changes.insert(change_idx, RopePendingChange {
                            original_range:     original_start..gap_end,
                            new_range:          remaining_range.start..(remaining_range.start+gap_length),
                            changed_attributes: attribute_change
                        });

                        remaining_range.start   += gap_length;
                        remaining_length        -= gap_length;
                        change_idx              += 1;
                    } else {
                        // The new range needs to shrink the gap
                        self.changes.insert(change_idx, RopePendingChange {
                            original_range:     original_start..gap_end,
                            new_range:          remaining_range.start..(remaining_range.start+remaining_length),
                            changed_attributes: attribute_change
                        });

                        // Shrink the future changes
                        let length_diff = gap_length - remaining_length;

                        // Adjust the position of the following ranges
                        for move_idx in (change_idx+1)..self.changes.len() {
                            self.changes[move_idx].new_range.start  = self.changes[move_idx].new_range.start - length_diff;
                            self.changes[move_idx].new_range.end    = self.changes[move_idx].new_range.end - length_diff;
                        }

                        remaining_range.start   += remaining_length;
                        remaining_range.end     -= gap_length-remaining_length;
                        remaining_length        = 0;

                        // Move to the next range
                        let change              = &self.changes[change_idx];

                        let old_len             = change.original_range.len() as i64;
                        let new_len             = change.new_range.len() as i64;

                        diff                    += old_len - new_len;
                        change_idx              += 1;
                    }
                }
            }
        }
    }

    ///
    /// Pulls the pending changes from this rope
    ///
    /// There will be no pending changes after this function returns
    ///
    pub fn pull_changes<'a>(&'a mut self) -> impl 'a+Iterator<Item=RopeAction<BaseRope::Cell, BaseRope::Attribute>> {
        // Remove the pending changes from the rope
        let mut pending_changes = vec![];
        mem::swap(&mut self.changes, &mut pending_changes);

        // Create an iterator to return the actions for these changes
        // Changes are returned in reverse so these edits can be applied directly to another rope in the original state
        pending_changes.into_iter()
            .rev()
            .filter(|change| change.original_range.len() > 0 || change.new_range.len() > 0)
            .flat_map(move |change| {
                // Read the cells for this change
                if change.changed_attributes && change.new_range.len() > 0 {
                    // Replace the cells and attributes in this range

                    // Usually the attribute will cover the whole range but it's possible to create multiple attributes in a range via several updates: we work backwards until we've covered the entire range
                    let mut original_range  = change.original_range;
                    let new_range           = change.new_range;
                    let mut end_pos         = new_range.end;
                    let mut changes         = vec![];

                    loop {
                        // Read the attributes at the current end position
                        let (attribute, attribute_range)    = self.rope.read_attributes(end_pos-1);
                        let start_pos                       = new_range.start.max(attribute_range.start);
                        let valid_range                     = start_pos..end_pos;
                        let new_cells                       = self.read_cells(valid_range).cloned().collect();

                        changes.push(RopeAction::ReplaceAttributes(original_range.clone(), new_cells, attribute.clone()));

                        // Stop once we reach the start of the changed range
                        if start_pos <= new_range.start { break; }

                        // Make the next change an insertion at the beginning of the range
                        original_range                      = original_range.start..original_range.start;

                        // Continue searching for attributes from the point we reached
                        end_pos                             = start_pos;
                    }

                    changes
                } else {
                    // Just replace the cells in this range
                    let new_cells = self.rope.read_cells(change.new_range.clone()).cloned().collect::<Vec<_>>();

                    vec![RopeAction::Replace(change.original_range, new_cells)]
                }
            })
    }
}

impl<BaseRope, PullFn> Rope for PullRope<BaseRope, PullFn>
where 
BaseRope:   RopeMut, 
PullFn:     Fn() -> () {
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

impl<BaseRope, PullFn> RopeMut for PullRope<BaseRope, PullFn>
where 
BaseRope:   RopeMut, 
PullFn:     Fn() -> () {
    ///
    /// Performs the specified editing action to this rope
    ///
    fn edit(&mut self, action: RopeAction<Self::Cell, Self::Attribute>) {
        let need_pull = self.changes.len() == 0;

        // Store the change
        match &action {
            RopeAction::Replace(range, new_values)                  => self.mark_change(range.clone(), new_values.len(), false),
            RopeAction::SetAttributes(range, _attr)                 => self.mark_change(range.clone(), range.len(), true),
            RopeAction::ReplaceAttributes(range, new_values, _attr) => self.mark_change(range.clone(), new_values.len(), true)
        }

        // Pass on to the base rope
        self.rope.edit(action);

        // Indicate that there are pending changes
        if need_pull && self.changes.len() > 0 {
            (self.pull_fn)();
        }
    }

    ///
    /// Replaces a range of cells. The attributes applied to the new cells will be the same
    /// as the attributes that were applied to the first cell in the replacement range
    ///
    fn replace<NewCells: IntoIterator<Item=Self::Cell>>(&mut self, range: Range<usize>, new_cells: NewCells) {
        let need_pull = self.changes.len() == 0;

        let new_cells = new_cells.into_iter().collect::<Vec<_>>();

        self.mark_change(range.clone(), new_cells.len(), false);
        self.rope.replace(range, new_cells);

        // Indicate that there are pending changes
        if need_pull && self.changes.len() > 0 {
            (self.pull_fn)();
        }
    }

    ///
    /// Sets the attributes for a range of cells
    ///
    fn set_attributes(&mut self, range: Range<usize>, new_attributes: Self::Attribute) {
        let need_pull = self.changes.len() == 0;

        self.mark_change(range.clone(), range.len(), true);
        self.rope.set_attributes(range, new_attributes);

        // Indicate that there are pending changes
        if need_pull && self.changes.len() > 0 {
            (self.pull_fn)();
        }
    }

    ///
    /// Replaces a range of cells and sets the attributes for them.
    ///
    fn replace_attributes<NewCells: IntoIterator<Item=Self::Cell>>(&mut self, range: Range<usize>, new_cells: NewCells, new_attributes: Self::Attribute) {
        let need_pull = self.changes.len() == 0;

        let new_cells = new_cells.into_iter().collect::<Vec<_>>();

        self.mark_change(range.clone(), new_cells.len(), true);
        self.rope.replace_attributes(range, new_cells, new_attributes);

        // Indicate that there are pending changes
        if need_pull && self.changes.len() > 0 {
            (self.pull_fn)();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use super::*;

    #[test]
    fn add_initial_change_range() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(4..10, 15, false);

        assert!(rope.changes[0].original_range == (4..10));
        assert!(rope.changes[0].new_range == (4..19));
        assert!(rope.changes.len() == 1);
    }

    #[test]
    fn shrink_range() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..45, 1, false);

        assert!(rope.changes[0].original_range == (5..45));
        assert!(rope.changes[0].new_range == (5..6));
        assert!(rope.changes.len() == 1);
    }

    #[test]
    fn add_multiple_changes_at_end() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(4..10, 15, false);
        rope.mark_change(20..25, 5, false);

        assert!(rope.changes[1].original_range == (11..16));
        assert!(rope.changes[1].new_range == (20..25));

        assert!(rope.changes[0].original_range == (4..10));
        assert!(rope.changes[0].new_range == (4..19));
        assert!(rope.changes.len() == 2);
    }

    #[test]
    fn add_overlapping_range_with_no_size_change() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(4..10, 15, false);
        rope.mark_change(20..25, 5, false);
        rope.mark_change(6..11, 5, false);

        assert!(rope.changes[1].original_range == (11..16));
        assert!(rope.changes[1].new_range == (20..25));

        assert!(rope.changes[0].original_range == (4..10));
        assert!(rope.changes[0].new_range == (4..19));
        assert!(rope.changes.len() == 2);
    }

    #[test]
    fn add_overlapping_range_with_size_change() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(4..10, 15, false);
        rope.mark_change(20..25, 5, false);

        rope.mark_change(6..12, 5, false);

        assert!(rope.changes[0].original_range == (4..10));
        assert!(rope.changes[0].new_range == (4..18));

        assert!(rope.changes[1].original_range == (11..16));
        assert!(rope.changes[1].new_range == (19..24));

        assert!(rope.changes.len() == 2);
    }

    #[test]
    fn add_overlapping_range_partially_at_end() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(4..10, 15, false);
        rope.mark_change(4..30, 20, false);

        assert!(rope.changes[0].original_range == (4..10));
        assert!(rope.changes[0].new_range == (4..19));

        assert!(rope.changes[1].original_range == (10..21));
        assert!(rope.changes[1].new_range == (19..24));

        assert!(rope.changes.len() == 2);
    }

    #[test]
    fn add_range_covering_existing_ranges() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 15, false);
        rope.mark_change(20..25, 5, false);
        rope.mark_change(5..30, 40, false);

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (25..45));

        assert!(rope.changes[1].original_range == (10..15));
        assert!(rope.changes[1].new_range == (20..25));

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..20));
        assert!(rope.changes.len() == 3);
    }

    #[test]
    fn add_range_in_gap() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 10, false);
        rope.mark_change(15..20, 10, false);

        assert!(rope.changes[1].original_range == (10..15));
        assert!(rope.changes[1].new_range == (15..25));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (25..35));

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..15));
        assert!(rope.changes.len() == 3);
    }

    #[test]
    fn add_range_partially_in_gap() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 10, false);
        rope.mark_change(15..18, 8, false);

        assert!(rope.changes[1].original_range == (10..13));
        assert!(rope.changes[1].new_range == (15..23));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (25..35));

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..15));
        assert!(rope.changes.len() == 3);
    }

    #[test]
    fn shrink_gap() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 10, false);
        rope.mark_change(15..20, 1, false);

        assert!(rope.changes[1].original_range == (10..15));
        assert!(rope.changes[1].new_range == (15..16));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (16..26));

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..15));
        assert!(rope.changes.len() == 3);
    }

    #[test]
    fn add_range_overlapping_gap() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 10, false);
        rope.mark_change(5..20, 20, false);

        assert!(rope.changes[1].original_range == (10..15));
        assert!(rope.changes[1].new_range == (15..25));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (25..35));

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..15));
        assert!(rope.changes.len() == 3);
    }

    #[test]
    fn add_range_partially_overlapping_gap() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 10, false);
        rope.mark_change(5..18, 18, false);

        assert!(rope.changes[1].original_range == (10..13));
        assert!(rope.changes[1].new_range == (15..23));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (25..35));

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..15));
        assert!(rope.changes.len() == 3);
    }

    #[test]
    fn add_range_with_gap_between_existing_ranges() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 5, false);
        rope.mark_change(5..30, 40, false);

        assert!(rope.changes[1].original_range == (10..15));
        assert!(rope.changes[1].new_range == (15..20));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (20..25));

        assert!(rope.changes[3].original_range == (20..25));
        assert!(rope.changes[3].new_range == (25..45));

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..15));
        assert!(rope.changes.len() == 4);
    }

    #[test]
    fn add_and_shrink_range() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 5, false);
        rope.mark_change(5..30, 40, false);
        rope.mark_change(5..45, 1, false);

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..5));

        assert!(rope.changes[1].original_range == (10..15));
        assert!(rope.changes[1].new_range == (5..5));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (5..5));

        assert!(rope.changes[3].original_range == (20..25));
        assert!(rope.changes[3].new_range == (5..6));

        assert!(rope.changes.len() == 4);
    }

    #[test]
    fn add_and_shrink_at_end() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(5..45, 1, false);

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..5));

        assert!(rope.changes[1].original_range == (10..40));
        assert!(rope.changes[1].new_range == (5..6));

        assert!(rope.changes.len() == 2);
    }

    #[test]
    fn add_and_shrink_range_across_gap_1() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 5, false);
        rope.mark_change(5..45, 1, false);

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..5));

        assert!(rope.changes[1].original_range == (10..15));
        assert!(rope.changes[1].new_range == (5..6));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (6..6));

        assert!(rope.changes[3].original_range == (20..40));
        assert!(rope.changes[3].new_range == (6..6));

        assert!(rope.changes.len() == 4);
    }

    #[test]
    fn add_and_shrink_range_across_gap_2() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 5, false);
        rope.mark_change(20..25, 5, false);
        rope.mark_change(5..45, 1, false);

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..5));

        assert!(rope.changes[1].original_range == (10..20));
        assert!(rope.changes[1].new_range == (5..6));

        assert!(rope.changes[2].original_range == (20..25));
        assert!(rope.changes[2].new_range == (6..6));

        assert!(rope.changes[3].original_range == (25..45));
        assert!(rope.changes[3].new_range == (6..6));

        assert!(rope.changes.len() == 4);
    }

    #[test]
    fn add_and_shrink_range_into_gap() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 5, false);
        rope.mark_change(5..18, 1, false);

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..5));

        assert!(rope.changes[1].original_range == (10..13));
        assert!(rope.changes[1].new_range == (5..6));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (8..13));

        assert!(rope.changes.len() == 3);
    }

    #[test]
    fn add_and_erase_range() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.mark_change(5..10, 10, false);
        rope.mark_change(20..25, 5, false);
        rope.mark_change(5..30, 40, false);
        rope.mark_change(5..45, 0, false);

        assert!(rope.changes[0].original_range == (5..10));
        assert!(rope.changes[0].new_range == (5..5));

        assert!(rope.changes[1].original_range == (10..15));
        assert!(rope.changes[1].new_range == (5..5));

        assert!(rope.changes[2].original_range == (15..20));
        assert!(rope.changes[2].new_range == (5..5));

        assert!(rope.changes[3].original_range == (20..25));
        assert!(rope.changes[3].new_range == (5..5));

        assert!(rope.changes.len() == 4);
    }

    #[test]
    fn pull_basic_change() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.replace(0..0, vec![1, 2, 3]);

        let pulled = rope.pull_changes().collect::<Vec<_>>();
        assert!(pulled == vec![RopeAction::Replace(0..0, vec![1, 2, 3])]);
    }

    #[test]
    fn clear_after_pull() {
        let mut rope = PullRope::from(AttributedRope::<u8, ()>::new(), || {});

        rope.replace(0..0, vec![1, 2, 3]);

        let _       = rope.pull_changes();
        let pulled  = rope.pull_changes().collect::<Vec<_>>();
        assert!(pulled == vec![]);
    }
}
