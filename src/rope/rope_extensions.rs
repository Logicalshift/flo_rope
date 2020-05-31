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
