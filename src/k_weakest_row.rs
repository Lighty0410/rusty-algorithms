// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/

use std::collections::HashMap;

pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut rows: Vec<(i32, usize)> = Vec::new();
    let size = k as usize;

    for (idx, row) in mat.into_iter().enumerate() {
        let sum = row.iter().sum::<i32>();

        rows.push((sum, idx))
    }

    rows.sort();

    rows[..size].iter().map(|&kv| kv.1 as i32).collect()
}
