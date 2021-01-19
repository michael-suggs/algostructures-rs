//! Tree and tree-related algorithms.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A single tree node with parent and child references.
pub struct Node {
    pub id: usize,
    pub children: RefCell<Vec<Rc<Node>>>,
    pub parents: RefCell<Vec<Weak<Node>>>,
}

pub struct Tree {

}
