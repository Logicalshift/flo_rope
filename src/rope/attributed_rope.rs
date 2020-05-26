use super::node::*;
use super::branch::*;
use super::attributed_rope_iterator::*;

use crate::api::*;

use std::mem;
use std::iter;
use std::sync::*;
use std::ops::{Range};

/// The number of cells where we would rather split the rope than splice an existing cell
///
/// (We don't need to always append at the end of a string as inserting in the middle will still be
/// fast enough: depending on the application it could potentially be valid to allow for quite long 
/// cell sizes)
///
/// Attributes are attached to cells, so setting an attribute on a range will always generate a
/// split in the event it doesn't always cover a whole cell.
const SPLIT_LENGTH: usize = 32;

///
/// The attributed rope struct provides the simplest implementation of a generic rope with attributes.
///
/// This struct is suitable for data storage of bulk vectors of data where frequent and arbitrary editing
/// is needed. Using a `u8` cell to represent UTF-8 makes this a suitable data type for building something
/// like a text editor around, although for interactive applications, the streaming rope classes might be
/// more suitable as they can dynamically notify about their updates.
///
#[derive(Clone)]
pub struct AttributedRope<Cell, Attribute> {
    /// The nodes that make up this rope
    pub (super) nodes: Vec<RopeNode<Cell, Attribute>>,

    /// The index of the root node
    root_node_idx: RopeNodeIndex,

    /// List of nodes that are not being used
    free_nodes: Vec<usize>
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
            nodes:          vec![RopeNode::Leaf(None, vec![], Arc::new(Attribute::default()))],
            root_node_idx:  RopeNodeIndex(0),
            free_nodes:     vec![]
        }
    }

    ///
    /// Creates a rope from a list of cells
    ///
    pub fn from<NewCells: IntoIterator<Item=Cell>>(cells: NewCells) -> AttributedRope<Cell, Attribute> {
        AttributedRope {
            nodes:          vec![RopeNode::Leaf(None, cells.into_iter().collect(), Arc::new(Attribute::default()))],
            root_node_idx:  RopeNodeIndex(0),
            free_nodes:     vec![]
        }
    }

    ///
    /// Allocates space for a new node, stores it and returns the index that it was written to
    ///
    fn store_new_node(&mut self, node: RopeNode<Cell, Attribute>) -> RopeNodeIndex {
        // Try to use an existing empty node if there is one
        if let Some(free_node) = self.free_nodes.pop() {
            // Store in this free node
            self.nodes[free_node] = node;
            RopeNodeIndex(free_node)
        } else {
            // Create a new node
            let free_node = self.nodes.len();
            self.nodes.push(node);
            RopeNodeIndex(free_node)
        }
    }

    ///
    /// Retrieves the root node for this rope
    ///
    fn root_node<'a>(&'a self) -> &'a RopeNode<Cell, Attribute> {
        &self.nodes[self.root_node_idx.idx()]
    }

    ///
    /// Used by tests to force a split at a particular position
    ///
    #[cfg(test)]
    pub (super) fn split_at(&mut self, pos: usize) {
        let (offset, node_idx) = self.find_leaf(pos);
        self.split(node_idx, pos-offset);
    }

    ///
    /// Divides a node into two (replacing a leaf node with a branch node). Returns the left-hand node of the split
    ///
    fn split(&mut self, leaf_node_idx: RopeNodeIndex, split_index: usize) -> RopeNodeIndex {
        // Take the leaf node (this leaves it empty)
        let leaf_node = self.nodes[leaf_node_idx.idx()].take();

        match leaf_node {
            RopeNode::Leaf(parent, cells, attribute) => {
                // Split the cells into two halves
                let mut cells       = cells;
                let right_cells     = cells.drain(split_index..cells.len()).collect::<Vec<_>>();
                let left_cells      = cells;
                let length          = left_cells.len() + right_cells.len();

                // Generate the left and right nodes (the current leaf node will become the branch node)
                let left_node       = RopeNode::Leaf(Some(leaf_node_idx), left_cells, attribute.clone());
                let right_node      = RopeNode::Leaf(Some(leaf_node_idx), right_cells, attribute.clone());

                let left_idx        = self.store_new_node(left_node);
                let right_idx       = self.store_new_node(right_node);

                // Replace the leaf node with the new node
                self.nodes[leaf_node_idx.idx()] = RopeNode::Branch(RopeBranch {
                    left:   left_idx,
                    right:  right_idx,
                    length: length,
                    parent: parent
                });

                left_idx
            }

            leaf_node => {
                debug_assert!(false, "Tried to split non-leaf nodes");

                // Not a leaf node: put the node back in the array
                self.nodes[leaf_node_idx.idx()] = leaf_node;

                leaf_node_idx
            }
        }
    }

    ///
    /// Inserts a blank node into an existing leaf node (useful when inserting cells with different attributes
    /// to their surroundings). Returns the index of the blank node.
    ///
    fn insert_blank_node(&mut self, leaf_node_idx: RopeNodeIndex, split_index: usize) -> RopeNodeIndex {
        // If the split is right at the start of the node, then this is just a normal split operation at index 0
        if split_index == 0 {
            // As the split is right at the start, we can just add a new item right there
            self.split(leaf_node_idx, 0)
        } else {
            // Take the leaf node (this leaves it empty)
            let leaf_node = &self.nodes[leaf_node_idx.idx()];

            match leaf_node {
                RopeNode::Leaf(_, cells, _) => {
                    if split_index >= cells.len() {
                        // Result is the RHS of the leaf node generated after a single split
                        let lhs_idx = self.split(leaf_node_idx, split_index);
                        let rhs_idx = self.next_leaf_to_the_right(lhs_idx).expect("Split failed to create RHS leaf node");

                        rhs_idx
                    } else {
                        // Need to make two splits to divide the existing node
                        let lhs_idx     = self.split(leaf_node_idx, split_index);
                        let rhs_idx     = self.next_leaf_to_the_right(lhs_idx).expect("Split failed to create RHS leaf node");
                        let empty_leaf  = self.split(rhs_idx, 0);

                        empty_leaf
                    }
                }

                _ => {
                    // Can only perform this operation on leaf nodes
                    panic!("Tried to split non-leaf nodes");
                }
            }
        }
    }

    ///
    /// Joins a leaf node to the node immediately to the right
    ///
    fn join_to_right(&mut self, leaf_node_idx: RopeNodeIndex) {
        // Fetch the node to the right (we do nothing if there's no node to the right)
        let right_node_idx = match self.next_leaf_to_the_right(leaf_node_idx) { Some(rhs) => rhs, None => { return; } };

        // Take the leaf node, leaving it empty
        let leaf_node = self.nodes[leaf_node_idx.idx()].take();

        // Remove the branch node for this leaf
        match leaf_node {
            RopeNode::Leaf(parent_node_idx, lhs_cells, _)   => {
                // Fetch the parent node
                let parent_node_idx = match parent_node_idx { Some(idx) => idx, None => { return; } };

                // Take the parent node too
                let parent_node = self.nodes[parent_node_idx.idx()].take();

                // Should be a branch with one branch being our leaf: pick the other side of the branch
                let (remaining_node_idx, grandparent_node_idx) = match parent_node {
                    RopeNode::Branch(branch) => {
                        if branch.left == leaf_node_idx {
                            (branch.right, branch.parent)
                        } else {
                            debug_assert!(branch.right == leaf_node_idx);
                            (branch.left, branch.parent)
                        }
                    }

                    _ => panic!("Parent node must be a branch node")
                };

                // Change the grandparent to point at the remaining node
                match grandparent_node_idx {
                    Some(grandparent_node_idx) => {
                        match &mut self.nodes[grandparent_node_idx.idx()] {
                            RopeNode::Branch(grandparent_branch) => {
                                // Replace the left/right node with the remaining node from the original branch
                                if grandparent_branch.left == parent_node_idx {
                                    grandparent_branch.left = remaining_node_idx;
                                } else {
                                    debug_assert!(grandparent_branch.right == parent_node_idx);
                                    grandparent_branch.right = remaining_node_idx;
                                }
                            }

                            _ => panic!("Grandparent node must be a branch node")
                        }
                    }

                    None => {
                        // Found a new root node
                        self.root_node_idx = remaining_node_idx;
                    }
                }

                // Change the remaining node so its parent is the grandparent node
                match &mut self.nodes[remaining_node_idx.idx()] {
                    RopeNode::Leaf(parent, _, _)    => { *parent = grandparent_node_idx; }
                    RopeNode::Branch(branch)        => { branch.parent = grandparent_node_idx; }
                    RopeNode::Empty                 => { panic!("Found an unexpected empty node"); }
                }

                // The parent and leaf node are no longer referenced
                self.free_nodes.push(leaf_node_idx.idx());
                self.free_nodes.push(parent_node_idx.idx());

                // Join the text if the original leaf node is non-empty
                if lhs_cells.len() > 0 {
                    match &mut self.nodes[right_node_idx.idx()] {
                        RopeNode::Leaf(_, rhs_cells, _) => {
                            // The LHS cells are at the start of the new node, so swap them into the existing node
                            let mut cells = lhs_cells;
                            mem::swap(&mut cells, rhs_cells);

                            // After the swap, lhs_cells contain the cells to append to the end
                            rhs_cells.extend(cells);
                        }

                        _ => {
                            panic!("RHS of a join operation was not a leaf node");
                        }
                    }
                }
            }

            leaf_node => {
                // Not a leaf node: replace it and stop
                debug_assert!(false, "Tried to join a non-leaf node");
                self.nodes[leaf_node_idx.idx()] = leaf_node;
            }
        }
    }

    ///
    /// Given a leaf-node, replaces a range of cells with some new values
    ///
    fn replace_cells<Cells: Iterator<Item=Cell>>(&mut self, leaf_node_idx: RopeNodeIndex, range: Range<usize>, new_cells: Cells) {
        if let RopeNode::Leaf(parent_idx, cells, _attributes) = &mut self.nodes[leaf_node_idx.idx()] {
            // Adjust the range to fit in the cell range
            let mut range = range;
            if range.start > cells.len()    { range.start = cells.len(); }
            if range.end > cells.len()      { range.end = cells.len(); }

            // Work out the length difference
            let length_diff = (range.len() as i64) - (cells.len() as i64);

            // Substitute in the new cells
            cells.splice(range, new_cells);

            // Update the lengths in the branches above this node
            let mut parent_idx = *parent_idx;

            while let Some(branch_idx) = parent_idx {
                if let RopeNode::Branch(branch) = &mut self.nodes[branch_idx.idx()] {
                    // Adjust the branch length according to this replacement
                    branch.length = ((branch.length as i64) + length_diff) as usize;

                    // Continue with the parent of this node
                    parent_idx = branch.parent;
                } else {
                    // The tree is malformed
                    debug_assert!(false, "Parent node is not a branch");
                    parent_idx = None;
                }
            }
        } else {
            debug_assert!(false, "Tried to replace text in a leaf node");
        }
    }

    ///
    /// Finds the leaf node containing the specified index. The value returned is the leaf node and the
    /// offset from the rope start of the node start
    ///
    fn find_leaf(&self, idx: usize) -> (usize, RopeNodeIndex) {
        // Start at the current node
        let mut current_node    = self.root_node_idx;
        let mut offset          = 0;

        // Hunt for the leaf node that will contain this index
        // For the purposes of this search, the last node contains all following indexes
        while let RopeNode::Branch(branch) = &self.nodes[current_node.idx()] {
            // Get the left and right-hand nodes
            let left_idx    = branch.left;
            let right_idx   = branch.right;

            // Decide whether or not to follow to the left or right-hand side. If the offset is between nodes, we choose the left-hand side.
            let left_len    = self.nodes[left_idx.idx()].len();

            if (idx - offset) <= left_len {
                current_node    = left_idx;
            } else {
                offset          += left_len;
                current_node    = right_idx;
            }
        }

        // Result is the leaf node we found
        (offset, current_node)
    }

    ///
    /// Finds the next leaf-node to the right of a particular node (None for the last node in the tree)
    ///
    pub (super) fn next_leaf_to_the_right(&self, node_idx: RopeNodeIndex) -> Option<RopeNodeIndex> {
        // The initial node is the 'left' node which we're trying to find the RHS for
        let mut left_node_idx           = node_idx;
        let mut maybe_parent_node_idx   = self.nodes[left_node_idx.idx()].parent();

        // Move up the tree until the left node is on the left-hand side
        let mut right_node_idx          = None;

        while let Some(parent_node_idx) = maybe_parent_node_idx {
            if let RopeNode::Branch(parent_branch) = &self.nodes[parent_node_idx.idx()] {
                if left_node_idx == parent_branch.left {
                    // We can follow the RHS of the parent node to find the neighboring element
                    right_node_idx = Some(parent_branch.right);
                    break;
                } else {
                    // Move up the tree
                    debug_assert!(left_node_idx == parent_branch.right);

                    maybe_parent_node_idx   = parent_branch.parent;
                    left_node_idx           = parent_node_idx;
                }
            } else {
                debug_assert!(false, "Parent node was not a branch");
                maybe_parent_node_idx = None;
            }
        }

        // Move right then down from the parent node until we reach a leaf node
        if let Some(right_node_idx) = right_node_idx {
            let mut next_node = right_node_idx;

            while let RopeNode::Branch(branch) = &self.nodes[next_node.idx()] {
                next_node = branch.left;
            }

            Some(next_node)
        } else {
            None
        }
    }

    ///
    /// Performs a replacement operation on a particular leaf node
    ///
    fn replace_leaf<NewCells: Iterator<Item=Cell>>(&mut self, absolute_range: Range<usize>, leaf_offset: usize, leaf_node_idx: RopeNodeIndex, new_cells: NewCells) {
        // Get the initial leaf length
        let leaf_len    = self.nodes[leaf_node_idx.idx()].len();
        let leaf_pos    = absolute_range.start - leaf_offset;
        let leaf_end    = leaf_offset + leaf_len;

        // Replace within the leaf node
        self.replace_cells(leaf_node_idx, (absolute_range.start-leaf_offset)..(absolute_range.end-leaf_offset), new_cells);

        // Move to the right in the tree to remove extra characters from the range in the event it overruns the leaf cell
        if leaf_pos + absolute_range.len() > leaf_end {
            // Work out how many characters are remaining
            let mut remaining_to_right  = absolute_range.len() - (leaf_len - leaf_pos);

            // Keep removing from the next node to the right until there is none left
            let mut last_node_idx   = leaf_node_idx;
            let mut empty_nodes     = vec![];
            while remaining_to_right > 0 {
                // Fetch the next node along
                let next_node_idx   = match self.next_leaf_to_the_right(last_node_idx) { Some(node) => node, None => { break; } };

                // Work out how many cells we can remove from this node
                let next_node       = &self.nodes[next_node_idx.idx()];
                let to_remove       = remaining_to_right.min(next_node.len());

                // Remove the cells
                self.replace_cells(next_node_idx, 0..to_remove, iter::empty());

                // Add to the list of empty nodes if this cell is empty
                if self.nodes[next_node_idx.idx()].len() == 0 {
                    empty_nodes.push(next_node_idx);
                }

                // Move on if there are still remaining nodes to process
                remaining_to_right  -= to_remove;
                last_node_idx       = next_node_idx;
            }

            // Join any empty nodes that are left after this operation
            empty_nodes.into_iter().for_each(|empty_cell_idx| self.join_to_right(empty_cell_idx));
        }

        // If the original target node is empty, join it to the right
        if self.nodes[leaf_node_idx.idx()].len() == 0 {
            self.join_to_right(leaf_node_idx);
        }
    }

    ///
    /// Reads the cell values for a range in this rope
    ///
    pub fn read_cells<'a>(&'a self, range: Range<usize>) -> AttributedRopeIterator<'a, Cell, Attribute> {
        // Find the first cell in the range
        let (node_offset, node_idx) = self.find_leaf(range.start);

        // Create an iterator for the remaining cells
        AttributedRopeIterator {
            rope:               self,
            node_idx:           node_idx,
            node_offset:        range.start-node_offset,
            remaining_cells:    range.end-range.start
        }
    }
}

impl<Cell, Attribute> Rope for AttributedRope<Cell, Attribute> 
where   
Cell:       Clone, 
Attribute:  PartialEq+Clone+Default {
    type Cell           = Cell;
    type Attribute      = Attribute;

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
    fn read_cells<'a>(&'a self, range: Range<usize>) -> Box<dyn 'a+Iterator<Item=&Self::Cell>> {
        Box::new(self.read_cells(range))
    }

    ///
    /// Returns the attributes set at the specified location and their extent
    ///
    fn read_attributes<'a>(&'a self, pos: usize) -> (&'a Attribute, Range<usize>) {
        // Retrieve the node at the requested location
        let (leaf_offset, leaf_node_idx)    = self.find_leaf(pos);
        let leaf_node                       = &self.nodes[leaf_node_idx.idx()];
        let attributes                      = match leaf_node {
            RopeNode::Leaf(_, _, attr)  => attr,
            _                           => panic!("Found node was not a leaf node")
        };
        let mut extent                      = leaf_offset..(leaf_offset+leaf_node.len());

        // Move to the right to find the full extent of the attributes
        let mut last_node_idx = leaf_node_idx;
        loop {
            // Move to the right
            let next_node_idx = match self.next_leaf_to_the_right(last_node_idx) {
                Some(idx)   => idx,
                None        => { break; }
            };

            // Check that the attributes match the previous node
            let next_node = &self.nodes[next_node_idx.idx()];
            match next_node {
                RopeNode::Leaf(_, _, next_attr)  => {
                    if !(**next_attr).eq(&**attributes) {
                        break;
                    }
                }
                _ => { debug_assert!(false, "Neighboring node was not a leaf node"); break; }
            }

            // Grow the extent if the attributes match
            extent.end += next_node.len();

            // Keep going from this next node
            last_node_idx = next_node_idx;
        }

        (&**attributes, extent)
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
        match action {
            RopeAction::Replace(range, new_cells)                               => { self.replace(range, new_cells); }
            RopeAction::SetAttributes(range, new_attributes)                    => { self.set_attributes(range, new_attributes); }
            RopeAction::ReplaceAttributes(range, new_cells, new_attributes)     => { self.replace_attributes(range, new_cells, new_attributes); }
        }
    }

    ///
    /// Replaces a range of cells. The attributes applied to the new cells will be the same
    /// as the attributes that were applied to the first cell in the replacement range
    ///
    fn replace<NewCells: IntoIterator<Item=Self::Cell>>(&mut self, range: Range<usize>, new_cells: NewCells) {
        // Find the replacement position
        let (mut leaf_offset, mut leaf_node) = self.find_leaf(range.start);

        // Split the leaf node if necessary (ie, if we need to insert cells more than SPLIT_LENGTH cells before the end)
        let position_in_leaf = range.start - leaf_offset;
        if self.nodes[leaf_node.idx()].len()-position_in_leaf > SPLIT_LENGTH {
            // Split the leaf node
            self.split(leaf_node, position_in_leaf);

            // Pick the new leaf node to add to (TODO: the leaf node becomes a branch, we can select the LHS as it'll have the same offset)
            let (new_leaf_offset, new_leaf_node) = self.find_leaf(range.start);
            debug_assert!(leaf_offset == new_leaf_offset);
            leaf_node   = new_leaf_node;
            leaf_offset = new_leaf_offset;
        }

        // Delegate the replacement to replace_leaf
        self.replace_leaf(range, leaf_offset, leaf_node, new_cells.into_iter());
    }

    ///
    /// Sets the attributes for a range of cells
    ///
    fn set_attributes(&mut self, range: Range<usize>, new_attributes: Self::Attribute) {
        let len                 = self.len();
        let mut remaining_range = range;
        let new_attributes      = Arc::new(new_attributes);

        // Algorithm won't teminate if we try to set attributes beyond the end of the rope
        if remaining_range.start > len  { remaining_range.start = len; }
        if remaining_range.end > len    { remaining_range.end = len; }

        // Get the current leaf node
        let (mut leaf_offset, mut leaf_node_idx) = self.find_leaf(remaining_range.start);

        // Iterate until we've covered the entire range
        while remaining_range.start < remaining_range.end {
            let leaf_node   = &self.nodes[leaf_node_idx.idx()];
            let leaf_len    = leaf_node.len();
            let leaf_attr   = match leaf_node { RopeNode::Leaf(_, _, leaf_attributes) => leaf_attributes, _ => { break; } };

            // remaining_range.start must be within the current leaf node
            if (**leaf_attr).eq(&*new_attributes) {
                // This region already has the correct attributes, so move to the right
                remaining_range.start += leaf_len;
                leaf_offset     += leaf_len;
                leaf_node_idx   = match self.next_leaf_to_the_right(leaf_node_idx) { Some(idx) => idx, None => { break; } };

            } else if remaining_range.start >= leaf_offset + leaf_len {
                // Range starts at the end of the current leaf node, so move to the right
                leaf_offset     += leaf_len;
                leaf_node_idx   = match self.next_leaf_to_the_right(leaf_node_idx) { Some(idx) => idx, None => { break; } };

            } else if remaining_range.start != leaf_offset {
                // The attributes start in the middle of the current leaf node, so split it and try again
                let split_pos   = remaining_range.start - leaf_offset;
                leaf_node_idx   = self.split(leaf_node_idx, split_pos);
                leaf_node_idx   = match self.next_leaf_to_the_right(leaf_node_idx) { Some(idx) => idx, None => { break; } };
                leaf_offset     += split_pos;

            } else if remaining_range.end <= leaf_offset + leaf_len {
                // The attributes end before the end of the current leaf node, so split it and try again
                let split_pos   = remaining_range.end - leaf_offset;
                leaf_node_idx   = self.split(leaf_node_idx, split_pos);

            } else {
                // The entire range is to be set with the new attribute
                match &mut self.nodes[leaf_node_idx.idx()] {
                    RopeNode::Leaf(_, _, leaf_attributes)   => { *leaf_attributes = Arc::clone(&new_attributes); }
                    _                                       => { debug_assert!(false, "Missing leaf node"); }
                }

                // Move to the right to continue setting attributes
                remaining_range.start += leaf_len;
                leaf_offset     += leaf_len;
                leaf_node_idx   = match self.next_leaf_to_the_right(leaf_node_idx) { Some(idx) => idx, None => { break; } };

            }
        }
    }

    ///
    /// Replaces a range of cells and sets the attributes for them.
    ///
    fn replace_attributes<NewCells: IntoIterator<Item=Self::Cell>>(&mut self, range: Range<usize>, new_cells: NewCells, new_attributes: Self::Attribute) {
        // There are three cases to deal with here:
        //   * range starts in an existing cell with different attributes (we insert a blank cell and set the attributes there)
        //   * range is in an existing cell with the same attributes (just add to the cell)
        //   * range is at the start of an existing cell with different attributes but covers the entire cell (change the attributes and replace the cell)

        let (leaf_offset, leaf_node_idx)    = self.find_leaf(range.start);
        let leaf_node                       = &self.nodes[leaf_node_idx.idx()];
        let leaf_attributes                 = match leaf_node {
            RopeNode::Leaf(_, _, attributes)    => attributes,
            _                                   => { debug_assert!(false, "Failed to find leafnode while replacing text"); return; }
        };

        if (&**leaf_attributes).eq(&new_attributes) {
            // Attributes are unchanged for this node
            // TODO: a small optimisation would be to avoid searching for the node again in this step
            self.replace(range, new_cells);
        } else if leaf_offset == range.start && leaf_node.len() < range.len() {
            // Leaf node has different attributes but the entire node is covered, so we can just replace the attributes of the existing node

            // Replace attributes
            match &mut self.nodes[leaf_node_idx.idx()] {
                RopeNode::Leaf(_, _, attributes)    => *attributes = Arc::new(new_attributes),
                _                                   => debug_assert!(false, "Failed to find a leaf node to set attributes on")
            }

            // Replace contents
            // TODO: same optimisation as before
            self.replace(range, new_cells);
        } else {
            // Create a blank node and insert the attributes there
            let empty_node_idx = self.insert_blank_node(leaf_node_idx, range.start - leaf_offset);

            // Replace attributes
            match &mut self.nodes[empty_node_idx.idx()] {
                RopeNode::Leaf(_, _, attributes)    => *attributes = Arc::new(new_attributes),
                _                                   => debug_assert!(false, "Failed to find a leaf node to set attributes on")
            }

            // Replace contents
            // TODO: same optimisation as before
            let range_start = range.start;
            self.replace_leaf(range, range_start, empty_node_idx, new_cells.into_iter());
        }
    }
}
