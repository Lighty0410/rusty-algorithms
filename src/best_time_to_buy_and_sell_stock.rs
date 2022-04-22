use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut curr_lowest = prices[0];
        let mut max = 0;

        for price in prices {
            max = max.max(price - curr_lowest);
            curr_lowest = curr_lowest.min(price);
        }

        max
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_max_profit() {
        println!("{}", Solution::max_profit([7, 6, 4, 3, 1].to_vec()));
    }
}
