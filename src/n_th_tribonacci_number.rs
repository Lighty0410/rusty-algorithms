use crate::Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            n @ 0 => return 0,
            n @ 1 => return 1,
            n @ 2 => return 1,
            _ => {}
        };

        let (mut first, mut second, mut third) = (0, 1, 1);
        let mut ans = 0;

        for _ in 3..=n {
            ans = first + second + third;
            let temp_3 = third;
            let temp_2 = second;
            third = first + second + third;
            second = temp_3;
            first = temp_2;
        }

        ans
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_tribonacci() {
        println!("{}", Solution::tribonacci(4));
    }
}
