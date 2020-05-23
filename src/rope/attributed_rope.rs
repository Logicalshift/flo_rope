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
    /// The nodes that make up this rope
    nodes: Vec<RopeNode<Cell, Attribute>>,

    /// The index of the root node
    root_node_idx: RopeNodeIndex,

    /// The number of empty nodes in the nodes list
    free_nodes: usize
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
            nodes:          vec![RopeNode::Leaf(None, Arc::new(vec![]), Arc::new(Attribute::default()))],
            root_node_idx:  RopeNodeIndex(0),
            free_nodes:     0
        }
    }

    ///
    /// Retrieves the root node for this rope
    ///
    fn root_node<'a>(&'a self) -> &'a RopeNode<Cell, Attribute> {
        &self.nodes[self.root_node_idx.idx()]
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
        match self.root_node() {
            RopeNode::Empty                         => 0,
            RopeNode::Leaf(_parent, cells, _attr)   => cells.len(),
            RopeNode::Branch(branch)                => branch.length
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
