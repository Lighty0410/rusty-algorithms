use crate::Solution;
use std::process::id;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if nums2.is_empty() {
            return;
        }

        let (mut p1_idx, mut p2_idx) = ((m - 1) as usize, (n - 1) as usize);
        let (mut p1, mut p2) = (nums1.get(p1_idx).map(|n| *n), nums2.get(p2_idx).map(|n| *n));

        let mut idx = nums1.len() - 1;
        while idx >= 0 {
            if p1 >= p2 {
                if let Some(num) = p1 {
                    let temp = nums1[idx];
                    nums1[idx] = num;
                    nums1[p1_idx] = temp;

                    if p1_idx as i32 - 1 < 0 {
                        p1 = None;
                    } else {
                        p1_idx -= 1;
                        p1 = nums1.get(p1_idx).map(|n| *n);
                    }
                }
            } else {
                if let Some(num) = p2 {
                    nums1[idx] = num;
                    if p2_idx as i32 - 1 < 0 {
                        p2 = None;
                    } else {
                        p2_idx -= 1;
                        p2 = nums2.get(p2_idx).map(|n| *n);
                    }
                }
            }

            if idx == 0 {
                break;
            }

            idx -= 1;
        }
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![-1, -1, 0, 0, 0, 0];
        let mut nums2 = vec![-1, 0];

        Solution::merge(&mut nums1, 4, &mut nums2, 2);

        println!("{:?}", nums1);
    }
}
