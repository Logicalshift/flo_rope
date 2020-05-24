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
    /// Divides a node into two (replacing a leaf node with a branch node)
    ///
    fn split(&mut self, leaf_node_idx: RopeNodeIndex, split_index: usize) {
        // Take the leaf node (this leaves it empty)
        let leaf_node = self.nodes[leaf_node_idx.idx()].take();

        match leaf_node {
            RopeNode::Leaf(parent, cells, attribute) => {
                // We split the node by cloning
                let (parent, cells, attribute) = (parent.clone(), cells.clone(), attribute.clone());

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
                self.nodes[leaf_node_idx.idx()] = RopeNode::Branch(Arc::new(RopeBranch {
                    left:   left_idx,
                    right:  right_idx,
                    length: length,
                    parent: parent
                }));
            }

            leaf_node => {
                // Not a leaf node: put the node back in the array
                self.nodes[leaf_node_idx.idx()] = leaf_node;
            }
        }
    }

    ///
    /// Joins a branch node into a leaf node. The attributes are retained from the left-side only
    ///
    fn join(&mut self, branch_node_idx: RopeNodeIndex) {
        // Fetch the branch and left/right nodes
        let branch_node     = &self.nodes[branch_node_idx.idx()];

        if let RopeNode::Branch(branch_node) = branch_node {
            let branch_parent   = branch_node.parent;
            let left_idx        = branch_node.left;
            let right_idx       = branch_node.right;
            let left_node       = self.nodes[left_idx.idx()].take();
            let right_node      = self.nodes[right_idx.idx()].take();

            match (left_node, right_node) {
                (RopeNode::Leaf(_, left_cells, left_attributes), RopeNode::Leaf(_, right_cells, _right_attributes)) => {
                    // Join the cells
                    let joined_cells                    = left_cells.into_iter()
                        .chain(right_cells.into_iter())
                        .collect();
                    let joined_attributes               = left_attributes.clone();

                    // Free the old leaf nodes (the 'take()' action has already marked them as unused)
                    self.free_nodes.push(left_idx.idx());
                    self.free_nodes.push(right_idx.idx());

                    // Replace the branch node with a leaf node
                    self.nodes[branch_node_idx.idx()]   = RopeNode::Leaf(branch_parent, joined_cells, joined_attributes);
                }

                (left_node, right_node) => {
                    // Not two leaf nodes, so there's no joining action that can be taken: put the nodes back where they were
                    self.nodes[left_idx.idx()]  = left_node;
                    self.nodes[right_idx.idx()] = right_node;
                }
            }
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
        match action {
            RopeAction::Replace(range, new_cells)                               => { unimplemented!() }
            RopeAction::SetAttributes(range, new_attributes)                    => { unimplemented!() }
            RopeAction::ReplaceAttributes(range, new_cells, new_attributes)     => { unimplemented!() }
        }
    }
}
