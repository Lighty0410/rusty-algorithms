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
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        Self::helper(root, 0)
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, mut current: i32) -> i32 {
        match root {
            None => current,
            Some(node) => {
                current += 1;
                let left = Self::helper(node.borrow().left.clone(), current);
                let right = Self::helper(node.borrow().right.clone(), current);

                std::cmp::max(left, right)
            }
        }
    }
}

mod tests {
    use crate::max_depth_binary_tree::TreeNode;
    use crate::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_max_depth() {
        let b_tree = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
            }))),
        };

        let max_depth = Solution::max_depth(Some(Rc::new(RefCell::new(b_tree))));

        println!("{}", max_depth);
    }
}
