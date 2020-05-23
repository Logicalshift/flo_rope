use super::branch::*;

use std::sync::*;

///
/// A node in a rope
///
#[derive(Clone, PartialEq, Debug)]
pub enum RopeNode<Cell, Attribute> {
    /// A leaf node represents a substring of cells
    Leaf(Arc<Vec<Cell>>, Arc<Attribute>),

    /// A rope branch represents a point where a rope is split into two substrings
    Branch(Arc<RopeBranch<Cell, Attribute>>)
}
