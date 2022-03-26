use std::collections::HashMap;

// steps:
// 1) Create a map where idx is a required num.
// Required num is a num we should find in order to satisfy the constraints.
// For instance, if nums = [0,1,2,3] and target = 4, and the current idx is 0.
// A required number to satisfy the solution for 0 is 4 because 4-0 = 4 (or 4+0 = 4).
// 2) As soon as we found an appropriate number for a certain key in the map, return idx from the hashmap and the current idx.
// S/T complexity - O(n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut idx_map = HashMap::new();

    for (idx, num) in nums.into_iter().enumerate() {
        if let Some(idx_1) = idx_map.get(&num) {
            return vec![*idx_1 as i32, idx as i32];
        }

        let required_num = target - num;
        idx_map.insert(required_num, idx);
    }

    vec![]
}

mod tests {
    use crate::two_sum::two_sum;

    #[test]
    fn test_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let two_sum = two_sum(nums, target);

        assert_eq!(two_sum, vec![0, 1]);
    }
}
