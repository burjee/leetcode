struct Solution {}
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for (i, j) in (0..mat.len()).zip((0..mat.len()).rev()) {
            ans += mat[i][i];
            ans += mat[j][i];
        }
        if mat.len() % 2 == 1 {
            ans -= mat[mat.len() / 2][mat.len() / 2];
        }
        ans
    }
}

pub fn run() {
    let input = [
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ],
        vec![vec![5]],
    ];
    for mat in input {
        println!("{}", Solution::diagonal_sum(mat));
    }
}
