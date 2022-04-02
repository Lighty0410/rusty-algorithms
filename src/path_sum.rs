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

struct Solution;

impl Solution {
    // https://leetcode.com/problems/path-sum/
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        return match root {
            None => false,
            Some(node) => {
                let borrowed_node = node.borrow();

                match (&borrowed_node.left, &borrowed_node.right) {
                    (None, None) => (target_sum - borrowed_node.val) == 0,
                    _ => {
                        let target = target_sum - borrowed_node.val;
                        Solution::has_path_sum(borrowed_node.left.clone(), target)
                            || Solution::has_path_sum(borrowed_node.right.clone(), target)
                    }
                }
            }
        };
    }
}
