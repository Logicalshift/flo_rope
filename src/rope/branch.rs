use super::node::*;

///
/// Represents a branch in a rope
///
#[derive(Clone, Debug, PartialEq)]
pub struct RopeBranch {
    /// The left-hand side of the rope (first part of the string)
    pub left: usize,

    /// The right-hand side of the rope
    pub right: usize,

    /// The total length of all the substrings under this branch
    pub length: usize,

    /// The parent of this branch, or None if this is the root node
    pub parent: RopeNodeIndex
}
