use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut right_stack = Vec::new();
        let mut left_stack = Vec::new();

        if let Some(node) = root {
            right_stack = Self::get_right_nodes_helper(node.borrow().right.clone());
            left_stack = Self::get_left_nodes_helper(node.borrow().left.clone());
        }

        println!("{:?}:{:?}", left_stack, right_stack);

        right_stack == left_stack
    }

    pub fn get_right_nodes_helper(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut stack = Vec::new();

        if let Some(node) = node {
            if node.borrow().right.is_none() && node.borrow().left.is_none() {
                stack.push(Some(node.borrow().val));
                return stack;
            }

            stack.append(&mut Self::get_right_nodes_helper(
                node.borrow().right.clone(),
            ));
            stack.append(&mut Self::get_right_nodes_helper(
                node.borrow().left.clone(),
            ));

            stack.push(Some(node.borrow().val));
            return stack;
        }

        stack.push(None);
        stack

        // match node {
        //     None => stack,
        //     Some(node) => {
        //         stack.append(&mut Self::get_right_nodes_helper(
        //             node.borrow().right.clone(),
        //         ));
        //         stack.push(node.borrow().val);
        //         stack.append(&mut Self::get_right_nodes_helper(
        //             node.borrow().left.clone(),
        //         ));
        //
        //         stack
        //     }
        //     Some(node) if node.borrow().right.is_none() && node.borrow().left.is_none() => {
        //         stack.push(node.borrow().val);
        //         stack
        //     }
        // }
    }

    pub fn get_left_nodes_helper(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut stack = Vec::new();

        if let Some(node) = node {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                stack.push(Some(node.borrow().val));
                return stack;
            }

            stack.append(&mut Self::get_left_nodes_helper(node.borrow().left.clone()));
            stack.append(&mut Self::get_left_nodes_helper(
                node.borrow().right.clone(),
            ));

            stack.push(Some(node.borrow().val));
            return stack;
        }

        stack.push(None);
        stack

        // match node {
        //     None => stack,
        //     Some(node) => {
        //         stack.append(&mut Self::get_left_nodes_helper(
        //             node.borrow().right.clone(),
        //         ));
        //         stack.push(node.borrow().val);
        //         stack.append(&mut Self::get_left_nodes_helper(node.borrow().left.clone()));
        //
        //         stack
        //     }
        // }
    }
}

mod test {
    use crate::symmetric_tree::TreeNode;
    use crate::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_is_symmetric() {
        let tree_node = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        };

        let tree_node = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        };

        let tree_node = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: None,
        };

        Solution::is_symmetric(Some(Rc::new(RefCell::new(tree_node))));
    }
}
