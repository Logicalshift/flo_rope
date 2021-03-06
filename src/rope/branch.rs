use super::node::*;

///
/// Represents a branch in a rope
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RopeBranch {
    /// The left-hand side of the rope (first part of the string)
    pub left: RopeNodeIndex,

    /// The right-hand side of the rope
    pub right: RopeNodeIndex,

    /// The total length of all the substrings under this branch
    pub length: usize,

    /// The parent of this branch, or None if this is the root node
    pub parent: Option<RopeNodeIndex>
}
