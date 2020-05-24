use super::branch::*;

use std::mem;
use std::sync::*;

///
/// Describes a node index
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RopeNodeIndex(pub usize);

impl RopeNodeIndex {
    pub fn idx(self) -> usize {
        self.into()
    }
}

impl Into<usize> for RopeNodeIndex {
    fn into(self) -> usize {
        let RopeNodeIndex(idx) = self;
        idx
    }
}

///
/// A node in a rope
///
#[derive(Clone, PartialEq, Debug)]
pub enum RopeNode<Cell, Attribute> {
    /// An empty rope node
    Empty,

    /// A leaf node represents a substring of cells. The node index indicates the parent node
    Leaf(Option<RopeNodeIndex>, Vec<Cell>, Arc<Attribute>),

    /// A rope branch represents a point where a rope is split into two substrings
    Branch(RopeBranch)
}

impl<Cell, Attribute> RopeNode<Cell, Attribute> {
    ///
    /// Takes the value from this node and sets it to empty
    ///
    pub fn take(&mut self) -> RopeNode<Cell, Attribute> {
        let mut result = RopeNode::Empty;
        mem::swap(self, &mut result);
        result
    }

    ///
    /// Retrieves the length of the substring in this node and its decendents
    ///
    pub fn len(&self) -> usize {
        match self {
            RopeNode::Empty             => 0,
            RopeNode::Leaf(_, cells, _) => cells.len(),
            RopeNode::Branch(branch)    => branch.length
        }
    }
}
