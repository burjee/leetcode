struct Solution {}
impl Solution {
    /// ans(5x5) = ans(4x5) + ans(5x4) -> Time Limit Exceeded
    ///
    /// ans(mx2) = m
    ///
    /// ans(2xn) = n
    ///
    /// ans(mx1) = 1
    ///
    /// ans(1xn) = 1
    ///
    /// ==========
    ///
    /// ans(5x5) = ans[4][4]
    /// ans[4][4] = ans[3][4] + ans[4][3]
    ///
    /// [[ 0,  1,  1,  1,  1]
    ///  [ 1,  2,  3,  4,  5]
    ///  [ 1,  3,  6, 10, 15]
    ///  [ 1,  4, 10, 20, 35]
    ///  [ 1,  5, 15, 35, 70]]
    ///
    /// ans[m][2] = m
    ///
    /// ans[2][n] = n
    ///
    /// ans[m][1] = 1
    ///
    /// ans[1][n] = 1
    ///
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        Solution::get_number(m - 1, n - 1, vec![vec![0; n]; m].as_mut())
    }

    pub fn get_number(m: usize, n: usize, numbers: &mut Vec<Vec<i32>>) -> i32 {
        if m == 0 || n == 0 {
            return 1;
        }
        if m == 1 {
            return n as i32 + 1;
        }
        if n == 1 {
            return m as i32 + 1;
        }
        if numbers[m][n] > 0 {
            return numbers[m][n];
        }
        numbers[m][n] =
            Solution::get_number(m - 1, n, numbers) + Solution::get_number(m, n - 1, numbers);
        numbers[m][n]
    }
}

pub fn run() {
    let input = vec![
        (6, 8),
        (5, 8),
        (6, 7),
        (4, 7),
        (5, 7),
        (3, 2),
        (7, 3),
        (3, 3),
        (1, 1),
        (1, 2),
        (3, 6),
        (2, 2),
        (5, 5),
        (4, 5),
    ];
    for (m, n) in input {
        println!("ans: {}", Solution::unique_paths(m, n));
    }
}
