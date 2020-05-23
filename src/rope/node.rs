use super::branch::*;

use std::sync::*;

///
/// Describes a node index
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RopeNodeIndex(pub usize);

///
/// A node in a rope
///
#[derive(Clone, PartialEq, Debug)]
pub enum RopeNode<Cell, Attribute> {
    /// An empty rope node
    Empty,

    /// A leaf node represents a substring of cells. The node index indicates the parent node
    Leaf(Option<RopeNodeIndex>, Arc<Vec<Cell>>, Arc<Attribute>),

    /// A rope branch represents a point where a rope is split into two substrings
    Branch(Arc<RopeBranch>)
}
