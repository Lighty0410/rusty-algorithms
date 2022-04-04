// https://leetcode.com/problems/validate-binary-search-tree/

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate_bst_helper(root, i64::MAX, i64::MIN)
    }

    pub fn validate_bst_helper(root: Option<Rc<RefCell<TreeNode>>>, max: i64, min: i64) -> bool {
        match root {
            None => true,
            Some(node) => {
                if node.borrow().val as i64 <= min || node.borrow().val as i64 >= max {
                    return false;
                }

                Self::validate_bst_helper(node.borrow().left.clone(), node.borrow().val as i64, min)
                    && Self::validate_bst_helper(
                        node.borrow().right.clone(),
                        max,
                        node.borrow().val as i64,
                    )
            }
        }
    }
}

mod tests {
    use crate::validate_bst::TreeNode;
    use crate::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_validate_bst() {
        let tree_node = TreeNode {
            val: i32::MIN,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(i32::MAX)))),
        };

        println!("{}", i32::MAX);

        let is_valid = Solution::is_valid_bst(Some(Rc::new(RefCell::new(tree_node))));

        println!("{}", is_valid);
    }
}
