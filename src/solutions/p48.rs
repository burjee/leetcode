struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for i in 0..len {
            for j in (0..len - i).rev() {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][len - i - 1];
                matrix[j][len - i - 1] = temp;
            }
        }
        for i in 0..(len - 1) / 2 {
            for j in i + 1..len - 1 - i {
                let temp = matrix[j][i];
                matrix[j][i] = matrix[len - i - 1][j];
                matrix[len - i - 1][j] = temp;
            }
        }
    }
}

pub fn run() {
    let input = [
        vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ],
        get_matrix(1),
        get_matrix(2),
        get_matrix(3),
        get_matrix(4),
        get_matrix(5),
        get_matrix(6),
        get_matrix(7),
        get_matrix(8),
    ];

    for mut matrix in input {
        Solution::rotate(&mut matrix);
        println!();
        for m in matrix {
            println!("{:2?}", m);
        }
    }
}

fn get_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut vec = Vec::new();
    for i in 0..n {
        let mut array = Vec::new();
        for j in 0..n {
            array.push(j + i * n);
        }
        vec.push(array);
    }
    vec
}
