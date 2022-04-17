use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut dup_maps = HashSet::new();

        for num in nums {
            if dup_maps.contains(&num) {
                return true;
            }

            dup_maps.insert(num);
        }

        false
    }
}
