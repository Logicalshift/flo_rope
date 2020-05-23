use super::node::*;
use super::branch::*;

use crate::api::*;

use std::sync::*;
use std::ops::{Range};

///
///
///
#[derive(Clone)]
pub struct AttributedRope<Cell, Attribute> {
    /// The root of this rope
    root_node: RopeNode<Cell, Attribute>
}

impl<Cell, Attribute> AttributedRope<Cell, Attribute> 
where   
    Cell:       Clone, 
    Attribute:  PartialEq+Clone+Default {
    ///
    /// Creates a new, empty rope
    ///
    pub fn new() -> AttributedRope<Cell, Attribute> {
        AttributedRope {
            root_node: RopeNode::Leaf(Arc::new(vec![]), Arc::new(Attribute::default()))
        }
    }
}

impl<Cell, Attribute> Rope for AttributedRope<Cell, Attribute> 
where   
    Cell:       Clone, 
    Attribute:  PartialEq+Clone+Default {

    type Cell           = Cell;
    type Attribute      = Attribute;
    type CellIterator   = Box<dyn Iterator<Item=Cell>>;


    ///
    /// Returns the number of cells in this rope
    ///
    fn len(&self) -> usize {
        match &self.root_node {
            RopeNode::Leaf(cells, _)    => cells.len(),
            RopeNode::Branch(branch)    => branch.len()
        }
    }

    ///
    /// Reads the cell values for a range in this rope
    ///
    fn read_cells(&self, range: Range<usize>) -> Self::CellIterator {
        unimplemented!()
    }
}

impl<Cell, Attribute> RopeMut for AttributedRope<Cell, Attribute> 
where   
    Cell:       Clone, 
    Attribute:  PartialEq+Clone+Default {
    ///
    /// Performs the specified editing action to this rope
    ///
    fn edit(&mut self, action: RopeAction<Self::Cell, Self::Attribute>) {
        unimplemented!()
    }
}
