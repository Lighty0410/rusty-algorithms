use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// kinda crutchy. Required for leetcode since the structure isn't exposed.
pub trait NodeSwapper {
    fn swap(&mut self);
}

impl NodeSwapper for TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }
}

// Steps:
// 1) Create a queue.
// 2) Push to the queue.
// 3) Swap nodes.
// S/T complexity - O(n)
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut queue = Vec::new();
    if let Some(ref root) = root {
        queue.push(root.clone());
    } else {
        return root;
    }

    while !queue.is_empty() {
        let current_node = queue.remove(0);
        let mut borrowed_node = current_node.borrow_mut();
        borrowed_node.swap();

        if let Some(ref left_node) = borrowed_node.left {
            queue.push(left_node.clone());
        }
        if let Some(ref right_node) = borrowed_node.right {
            queue.push(right_node.clone());
        }
    }

    root
}
