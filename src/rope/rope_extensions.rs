use super::attributed_rope::*;

use crate::api::*;

use std::iter;
use std::ops::{AddAssign};

// These extensions will work for any implementation of Rope, but Rust doesn't let us a provide a universal implementation

impl<Cell, Attribute> AddAssign<Cell> for AttributedRope<Cell, Attribute>
where
Cell:       Clone, 
Attribute:  PartialEq+Clone+Default {
    fn add_assign(&mut self, other: Cell) {
        let len = self.len();
        self.replace(len..len, iter::once(other));
    }
}

impl<Cell, Attribute> Extend<Cell> for AttributedRope<Cell, Attribute> 
where
Cell:       Clone, 
Attribute:  PartialEq+Clone+Default {
    fn extend<I: IntoIterator<Item=Cell>>(&mut self, iter: I) {
        let len = self.len();
        self.replace(len..len, iter);
    }
}

impl<'a, Cell, Attribute> Extend<&'a Cell> for AttributedRope<Cell, Attribute> 
where
Cell:       'a+Clone, 
Attribute:  PartialEq+Clone+Default {
    fn extend<I: IntoIterator<Item=&'a Cell>>(&mut self, iter: I) {
        let len = self.len();
        self.replace(len..len, iter.into_iter().cloned());
    }
}

impl<Cell, Attribute> PartialEq for AttributedRope<Cell, Attribute>
where
Cell:       Clone+PartialEq, 
Attribute:  PartialEq+Clone+Default {
    fn eq(&self, other: &AttributedRope<Cell, Attribute>) -> bool {
        // Compare lengths
        if self.len() != other.len() {
            return false;
        }

        // Compare cells
        let mut cells_a = self.read_cells(0..self.len());
        let mut cells_b = other.read_cells(0..other.len());

        while let (Some(a), Some(b)) = (cells_a.next(), cells_b.next()) {
            if a != b {
                return false;
            }
        }

        // Compare attributes (coverage of each attribute may vary between the two styles)
        let len                         = self.len();
        let (mut attr_a, mut range_a)   = self.read_attributes(0);
        let (mut attr_b, mut range_b)   = self.read_attributes(0);

        loop {
            // Ranges should never be 0-length
            debug_assert!(range_a.len() != 0 && range_b.len() != 0);

            // Move the 'a' range on if range_b starts after it, similarly for range_b
            if range_b.start >= range_a.end || range_a.start == range_a.end {
                let (new_attr, new_range) = self.read_attributes(range_b.start);
                attr_a  = new_attr;
                range_a = new_range;
            }

            if range_a.start >= range_b.end || range_b.start == range_b.end {
                let (new_attr, new_range) = self.read_attributes(range_a.start);
                attr_b  = new_attr;
                range_b = new_range;
            }

            // range_a and range_b should now overlap
            if attr_a != attr_b {
                return false;
            }

            // Move range_a or range_b onwards
            if range_a.end >= len && range_b.end >= len {
                // All attributes compared
                break;
            }

            if range_a.end <= range_b.end {
                let (new_attr, new_range) = self.read_attributes(range_a.end);
                attr_a  = new_attr;
                range_a = new_range;
            } else {
                let (new_attr, new_range) = self.read_attributes(range_b.end);
                attr_b  = new_attr;
                range_b = new_range;
            }
        }

        // All equality tests passed
        return true;
    }
}
