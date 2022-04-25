use crate::Solution;
use std::collections::HashSet;

fn check_row(row: &[char]) -> bool {
    let mut char_set = HashSet::new();

    for num in row {
        if char_set.contains(num) {
            return false;
        }
        if *num != '.' {
            char_set.insert(*num);
        }
    }

    true
}

fn check_col(board: &[Vec<char>], size: usize) -> bool {
    for col in 0..size {
        let mut char_set = HashSet::new();
        for row in 0..size {
            let curr = board[row][col];

            if curr == '.' {
                continue;
            }

            if !char_set.insert(curr) {
                return false;
            }
        }
    }

    true
}

fn fill_sub_boxes(board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let r = board[0].len();
    let c = board.len();

    let mut boxes = vec![vec![Default::default(); c]; r];

    for (row_idx, board) in board.into_iter().enumerate() {
        for (col_idx, elem) in board.into_iter().enumerate() {
            let idx_1 = (row_idx / 3) * 3 + col_idx / 3;
            let idx_2 = (col_idx % 3) + (row_idx % 3) * 3;

            boxes[idx_1][idx_2] = elem;
        }
    }

    boxes
}

fn is_valid_sub_boxes(sub_boxes: Vec<Vec<char>>) -> bool {
    for char_box in sub_boxes {
        let mut elem_set = HashSet::new();

        for elem in char_box {
            if elem == '.' {
                continue;
            }

            if !elem_set.insert(elem) {
                return false;
            };
        }
    }

    true
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in 0..board.len() {
            if !check_row(&board[row]) {
                return false;
            }
        }

        if !check_col(&board, 9) {
            return false;
        }

        let sub_boxes = fill_sub_boxes(board);
        is_valid_sub_boxes(sub_boxes)
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            ['.', '.', '4', '.', '.', '.', '6', '3', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec(),
            ['5', '.', '.', '.', '.', '.', '.', '9', '.'].to_vec(),
            ['.', '.', '.', '5', '6', '.', '.', '.', '.'].to_vec(),
            ['4', '.', '3', '.', '.', '.', '.', '.', '1'].to_vec(),
            ['.', '.', '.', '7', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '5', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'].to_vec(),
        ];

        println!("{}", Solution::is_valid_sudoku(board));
    }
}
