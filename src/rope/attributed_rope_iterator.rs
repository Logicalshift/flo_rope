use super::node::*;
use super::attributed_rope::*;

///
/// Iterator that reads a range of cells in an attributed rope
///
pub struct AttributedRopeIterator<'a, Cell, Attribute> {
    /// The rope that's being read
    pub (super) rope: &'a AttributedRope<Cell, Attribute>,

    /// The node that's being read
    pub (super) node_idx: RopeNodeIndex,

    /// The offset of the node
    pub (super) node_offset: usize,

    /// The remaining number of cells to read from this iterator
    pub (super) remaining_cells: usize
}

impl<'a, Cell, Attribute> Iterator for AttributedRopeIterator<'a, Cell, Attribute>
where   
Cell:       Clone, 
Attribute:  PartialEq+Clone+Default {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        if self.remaining_cells == 0 {
            // No more cells to read
            None
        } else {
            // Try to read the current cell from the current node
            let node = &self.rope.nodes[self.node_idx.idx()];

            if let RopeNode::Leaf(_, cells, _) = node {
                if self.node_offset < cells.len() {
                    // Fetch the cell
                    let cell = &cells[self.node_offset];

                    // Move to the next item
                    self.node_offset        += 1;
                    self.remaining_cells    -= 1;

                    // Fetched the item
                    Some(cell)
                } else {
                    // Passed over the end of the node
                    if let Some(next_node) = self.rope.next_leaf_to_the_right(self.node_idx) {
                        // Move on to the next node, and try again
                        self.node_idx       = next_node;
                        self.node_offset    = 0;

                        self.next()
                    } else {
                        // Overran the end of the rope
                        None
                    }
                }
            } else {
                // Not a leaf node
                debug_assert!(false, "Rope iterator expects to only encounter leaf nodes");
                None
            }
        }
    }
}
