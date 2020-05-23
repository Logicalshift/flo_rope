use super::node::*;

///
/// Represents a branch in a rope
///
#[derive(Clone, Debug, PartialEq)]
pub struct RopeBranch<Cell, Attribute> {
    /// The left-hand side of the rope (first part of the string)
    left: RopeNode<Cell, Attribute>,

    /// The right-hand side of the rope
    right: RopeNode<Cell, Attribute>,

    /// The total length of all the substrings under this branch
    length: usize
}
