struct Solution {}
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix[0].len();
        let m = matrix.len();
        let mut flag = false;
        for h in 0..n {
            if matrix[0][h] == 0 {
                flag = true;
                break;
            }
        }
        for v in 1..m {
            for h in 0..n {
                if matrix[v][h] == 0 {
                    matrix[v][0] = 0;
                    matrix[0][h] = 0;
                }
            }
        }

        for v in 1..m {
            if matrix[v][0] == 0 {
                for h in 1..n {
                    matrix[v][h] = 0;
                }
            }
        }
        for h in 0..n {
            if matrix[0][h] == 0 {
                for v in 0..m {
                    matrix[v][h] = 0;
                }
            }
        }
        if flag {
            for h in 0..n {
                matrix[0][h] = 0;
            }
        }
    }
}

pub fn run() {
    let input = vec![
        vec![
            vec![-4, -2147483648, 6, -7, 0],
            vec![-8, 6, -8, -6, 0],
            vec![2147483647, 2, -9, -6, -10],
        ],
        vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
        vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
    ];

    for mut matrix in input {
        Solution::set_zeroes(matrix.as_mut());
        println!("ans: {:?}", matrix);
    }
}
