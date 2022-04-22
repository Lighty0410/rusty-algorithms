use crate::Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if mat[0].len() * mat.len() != (r * c) as usize {
            return mat;
        }

        let mut mat_1d = vec![i32::default(); (r * c) as usize];
        for (idx, num) in mat_1d.iter_mut().enumerate() {
            let i_1 = idx / mat[0].len();
            let i_2 = idx % mat[0].len();

            *num = mat[i_1][i_2];
        }

        let mut new_mat = vec![vec![Default::default(); c as usize]; r as usize];
        let n = new_mat[0].len();
        println!("{:?}", new_mat);

        for (row_idx, num_row) in new_mat.iter_mut().enumerate() {
            for (col_idx, num_col) in num_row.iter_mut().enumerate() {
                *num_col = mat_1d[n * row_idx + col_idx];
            }
        }

        new_mat
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_matrix_reshape() {
        let mat = [[1, 2].to_vec()].to_vec();
        let reshaped = Solution::matrix_reshape(mat, 1, 1);

        println!("{:?}", reshaped);
    }
}
