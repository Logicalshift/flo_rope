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
            nodes:          vec![RopeNode::Leaf(None, Arc::new(vec![]), Arc::new(Attribute::default()))],
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
    fn split(&mut self, leaf_node: RopeNodeIndex, index: usize) {
        if let RopeNode::Leaf(parent, cells, attribute) = &self.nodes[leaf_node.idx()] {
            // We split the node by cloning
            let (parent, cells, attribute) = (parent.clone(), cells.clone(), attribute.clone());

            // Split the cells into two halves
            let left_cells      = cells[0..index].iter().cloned().collect::<Vec<_>>();
            let right_cells     = cells[index..cells.len()].iter().cloned().collect::<Vec<_>>();

            // Generate the left and right nodes (the current leaf node will become the branch node)
            let left_node       = RopeNode::Leaf(Some(leaf_node), Arc::new(left_cells), attribute.clone());
            let right_node      = RopeNode::Leaf(Some(leaf_node), Arc::new(right_cells), attribute.clone());

            let left_idx        = self.store_new_node(left_node);
            let right_idx       = self.store_new_node(right_node);

            // Replace the leaf node with the new node
            self.nodes[leaf_node.idx()] = RopeNode::Branch(Arc::new(RopeBranch {
                left:   left_idx,
                right:  right_idx,
                length: cells.len(),
                parent: parent
            }));
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
        unimplemented!()
    }
}
