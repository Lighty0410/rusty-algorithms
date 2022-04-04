use crate::Solution;
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

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut output = Vec::new();
        let mut stack = Vec::new();
        let mut curr = root;

        while curr != None || !stack.is_empty() {
            while curr != None {
                stack.push(curr.clone());
                if let Some(left_node) = curr {
                    curr = left_node.borrow().left.clone();
                }
            }

            if let Some(node) = stack.pop() {
                curr = node;
            }
            if let Some(ref val) = curr {
                output.push(val.borrow().val);
            }
            if let Some(right) = curr {
                curr = right.borrow().right.clone();
            }
        }

        output
    }

    pub fn inorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::inorder_traversal_recursive_helper(root)
    }

    fn inorder_traversal_recursive_helper(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut output = Vec::new();

        match root {
            None => output,
            Some(root) => {
                output.append(&mut Self::inorder_traversal_recursive_helper(
                    root.borrow().left.clone(),
                ));
                output.push(root.borrow().val);
                output.append(&mut Self::inorder_traversal_recursive_helper(
                    root.borrow().right.clone(),
                ));

                output
            }
        }
    }
}

mod tests {
    use crate::binary_tree_inorder_traversal::TreeNode;
    use crate::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_inorder_traversal() {
        let b_tree = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        };

        let root = Solution::inorder_traversal_recursive(Some(Rc::new(RefCell::new(b_tree))));

        println!("{:?}", root);
    }
}
