use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr = nums[0];
        let mut max = nums[0];

        for num in nums[1..].iter() {
            curr += num;
            curr = curr.max(*num);
            max = max.max(curr);
        }

        max
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_max_sub_array() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

        println!("{}", Solution::max_sub_array(nums));
    }
}
