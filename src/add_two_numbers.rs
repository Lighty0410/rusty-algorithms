// https://leetcode.com/problems/add-two-numbers/
use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::collections::VecDeque;

pub fn push_to_queue(mut list: Option<Box<ListNode>>) -> VecDeque<i32> {
    let mut stack = VecDeque::new();

    while let Some(inner) = list {
        stack.push_back(inner.val);
        list = inner.next;
    }

    stack
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut stack_1 = push_to_queue(l1);
        let mut stack_2 = push_to_queue(l2);
        let mut carry = 0;
        let mut output_list = None;
        let mut curr = &mut output_list;

        while stack_1.len() != 0 || stack_2.len() != 0 {
            let num1 = stack_1.pop_front();
            let num2 = stack_2.pop_front();

            match (num1, num2) {
                (Some(num1), Some(num2)) => {
                    let output_num = (num1 + num2 + carry) % 10;
                    *curr = Some(Box::new(ListNode::new(output_num)));
                    if let Some(curr_ref) = curr {
                        curr = &mut curr_ref.next;
                    }
                    carry = (num1 + num2 + carry) / 10;
                }
                (Some(num1), None) => {
                    let output_num = (num1 + carry) % 10;
                    *curr = Some(Box::new(ListNode::new(output_num)));
                    if let Some(curr_ref) = curr {
                        curr = &mut curr_ref.next;
                    }
                    carry = (num1 + carry) / 10;
                }
                (None, Some(num2)) => {
                    let output_num = (num2 + carry) % 10;
                    *curr = Some(Box::new(ListNode::new(output_num)));
                    if let Some(curr_ref) = curr {
                        curr = &mut curr_ref.next;
                    }
                    carry = (num2 + carry) / 10;
                }
                _ => {}
            }

            if carry != 0 {
                *curr = Some(Box::new(ListNode::new(carry)));
            }
        }

        output_list
    }
}

mod tests {
    use crate::add_two_numbers::ListNode;
    use crate::Solution;

    fn list_from_vec(mut stack: Vec<i32>) -> Option<Box<ListNode>> {
        let mut list: Option<Box<ListNode>> = None;
        let mut list_ref = &mut list;

        while stack.len() != 0 {
            let num = stack.remove(0);
            *list_ref = Some(Box::new(ListNode {
                val: num,
                next: None,
            }));

            if let Some(node_ref) = list_ref {
                list_ref = &mut node_ref.next;
            }
        }

        list
    }

    #[test]
    fn test_add_two_numbers() {
        let l1 = list_from_vec(vec![2, 4, 9]);
        let l2 = list_from_vec(vec![5, 6, 4, 9]);

        let list = Solution::add_two_numbers(l2, l1);

        println!("{:?}", list);
    }
}
