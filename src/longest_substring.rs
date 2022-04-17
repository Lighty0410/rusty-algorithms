use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set = HashMap::new();
        let mut max_value = 0;
        let mut start = 0;

        for (current, symbol) in s.chars().enumerate() {
            if char_set.get(&symbol).is_some() {
                char_set.entry(symbol).and_modify(|num| {
                    start = start.max(*num);
                    *num = current + 1;
                    println!("{}:{}", num, symbol);
                });
            } else {
                char_set.insert(symbol, current + 1);
            }

            max_value = max_value.max(current + 1 - start)
        }

        max_value as i32
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn length_of_longest_substring() {
        let word = "fkkew";

        println!(
            "{}",
            Solution::length_of_longest_substring(word.to_string())
        );
    }
}
