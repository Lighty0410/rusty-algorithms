use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut seq_map: HashMap<char, usize> = HashMap::new();
        let (mut l, mut current_char) = (0, s.chars().nth(0).unwrap());
        let mut ans = 0;
        let mut maxf = 1;

        for (r, symbol) in s.chars().enumerate() {
            if seq_map.contains_key(&symbol) {
                seq_map.entry(symbol).and_modify(|entry: &mut usize| {
                    *entry += 1;
                    maxf = maxf.max(*entry);
                });
            } else {
                seq_map.insert(symbol, 1);
            }

            let current_window = r + 1 - l;

            if current_window - maxf > k as usize {
                seq_map.entry(current_char).and_modify(|entry| {
                    *entry -= 1;
                });

                current_char = s.chars().nth(l).unwrap();
                println!("shifted:{}", r);
            } else {
                ans = ans.max(current_window)
            }
        }

        ans as i32
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_character_replacement() {
        let word = "AABABBA";

        let ans = Solution::character_replacement(word.to_string(), 1);

        println!("{}", ans);
    }
}
