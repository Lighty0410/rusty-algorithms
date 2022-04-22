use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();

        let mut ans = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                i += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else {
                ans.push(nums1[i]);
                i += 1;
                j += 1;
            }
        }

        ans
    }

    // pub fn fill_map(smallest: &Vec<i32>, num_map: &mut HashMap<i32, usize>) {
    //     for (idx, num) in smallest.iter().enumerate() {
    //         num_map.insert(*num, idx + 1);
    //     }
    // }
    //
    // pub fn intersect_2(nums: Vec<i32>, num_map: &mut HashMap<i32, usize>) -> Vec<i32> {
    //     let (mut start, mut end) = (0, 0);
    //     let mut curr_key = 0;
    //
    //     let (mut local, mut local_end) = (0, 0);
    //     let mut next: Option<usize> = None;
    //
    //     for num in nums.iter() {
    //         if let Some(idx) = next {
    //             nums.ge
    //         }
    //
    //         next = *num_map.get(&num);
    //     }
    //
    //     nums
    // }

    pub fn intersect_map(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let mut freq_map = HashMap::new();

        if nums2.len() > nums1.len() {
            std::mem::swap(&mut nums2, &mut nums1);
        }

        let mut ans = Vec::new();
        for num in nums2.iter() {
            freq_map
                .entry(*num)
                .and_modify(|num| {
                    *num += 1;
                })
                .or_insert(1);
        }

        for num_1 in nums1 {
            freq_map.entry(num_1).and_modify(|num| {
                if *num != 0 {
                    ans.push(num_1);
                    *num -= 1;
                }
            });
        }

        ans
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_intersection() {
        let num1 = [1, 3, 4, 6, 9].to_vec();
        let num2 = [4, 9, 5].to_vec();

        println!("{:?}", Solution::intersect_map(num1, num2));
    }
}
