struct Solution {}
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        // n = m = len
        let mut sum = vec![vec![0; matrix.len()]; matrix.len()];
        for i in 0..matrix.len() {
            sum[0][i] = matrix[0][i];
        }
        for i in 1..matrix.len() {
            for j in 0..matrix.len() {
                sum[i][j] = sum[i - 1][j] + matrix[i][j];
            }
            for j in 1..matrix.len() {
                sum[i][j] = sum[i][j].min(sum[i - 1][j - 1] + matrix[i][j]);
            }
            for j in 0..matrix.len() - 1 {
                sum[i][j] = sum[i][j].min(sum[i - 1][j + 1] + matrix[i][j]);
            }
        }
        *sum[matrix.len() - 1].iter().min().unwrap()
    }

    // pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    //     // n = m = len
    //     let mut sum = vec![vec![0; matrix.len()]; matrix.len()];
    //     for i in 0..matrix.len() {
    //         sum[0][i] = matrix[0][i];
    //     }
    //     for i in 1..matrix.len() {
    //         for j in 0..matrix.len() {
    //             let c = sum[i - 1][j] + matrix[i][j];
    //             let mut l = i32::MAX;
    //             let mut r = i32::MAX;
    //             if j > 0 {
    //                 l = sum[i - 1][j - 1] + matrix[i][j];
    //             }
    //             if j < matrix.len() - 1 {
    //                 r = sum[i - 1][j + 1] + matrix[i][j];
    //             }
    //             sum[i][j] = c.min(l.min(r));
    //         }
    //     }
    //     *sum[matrix.len() - 1].iter().min().unwrap()
    // }
}

pub fn run() {
    let input = [
        vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]],
        vec![vec![-19, 57], vec![-40, -5]],
    ];
    for matrix in input {
        println!("{}", Solution::min_falling_path_sum(matrix));
    }
}
