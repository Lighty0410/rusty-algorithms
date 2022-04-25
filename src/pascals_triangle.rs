use crate::Solution;
use std::process::id;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut current_vec = {
            if num_rows > 1 {
                vec![1, 1]
            } else {
                return vec![vec![1].to_vec()];
            }
        };

        let mut triangles = Vec::new();
        triangles.push(vec![1]);
        triangles.push(current_vec.clone());

        for idx in 2..num_rows {
            let mut prev = 0;

            for idx in 0..current_vec.len() {
                if idx != 0 {
                    let res = prev + current_vec[idx];
                    let temp = current_vec[idx];
                    current_vec[idx] = res;
                    prev = temp;
                } else {
                    prev = current_vec[idx];
                }
            }
            current_vec.push(1);
            triangles.push(current_vec.clone());
        }

        triangles
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_generate() {
        println!("{:?}", Solution::generate(5));
    }
}
