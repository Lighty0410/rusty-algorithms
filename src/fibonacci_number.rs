use crate::Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        if n == 1 {
            return 1;
        }

        Self::fib(n - 1) + Self::fib(n - 2)
    }
}

mod test {
    use crate::Solution;

    #[test]
    pub fn test_fib() {
        println!("{}", Solution::fib(25));
    }
}
