struct Solution {}
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut c = 1;
        for layer in 0..(n + 1) / 2 {
            for j in layer..n - layer {
                matrix[layer][j] = c;
                c += 1;
            }
            for i in layer + 1..n - layer {
                matrix[i][n - layer - 1] = c;
                c += 1;
            }
            for j in (layer..n - layer - 1).rev() {
                matrix[n - layer - 1][j] = c;
                c += 1;
            }
            for i in (layer + 1..n - layer - 1).rev() {
                matrix[i][layer] = c;
                c += 1;
            }
        }
        matrix
    }

    pub fn _generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut c = 1;
        let mut step = n - 1;
        let mut i = 0;
        let mut j = 0;
        while step > 0 {
            for _ in 0..step {
                matrix[i][j] = c;
                c += 1;
                j += 1;
            }
            for _ in 0..step {
                matrix[i][j] = c;
                c += 1;
                i += 1;
            }
            for _ in 0..step {
                matrix[i][j] = c;
                c += 1;
                j -= 1;
            }
            for _ in 0..step {
                matrix[i][j] = c;
                c += 1;
                i -= 1;
            }
            i += 1;
            j += 1;
            step -= 2;
        }
        if n % 2 == 1 {
            let m = (n / 2) as usize;
            matrix[m][m] = n * n;
        }
        matrix
    }
}

pub fn run() {
    for n in 1..=10 {
        println!("{:?}", Solution::generate_matrix(n));
    }
}
