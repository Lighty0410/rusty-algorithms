use crate::Solution;
use std::process::id;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut dwellers = vec![0; n as usize];

        for connections in trust {
            let decrease_conn_status = (connections[0] - 1) as usize;
            let increase_conn_status = (connections[1] + 1) as usize;

            dwellers[decrease_conn_status] -= 1;
            dwellers[increase_conn_status] += 1;
        }

        for (idx, connections) in dwellers.into_iter().enumerate() {
            if connections == n - 1 {
                return (idx + 1) as i32;
            }
        }

        -1
    }
}

mod tests {
    use crate::Solution;

    #[test]
    fn test_find_judge() {
        let mut people = Vec::new();
        people.push([1, 3].to_vec());
        people.push([2, 3].to_vec());
        people.push([3, 1].to_vec());

        println!("{}", Solution::find_judge(3, people))
    }
}
