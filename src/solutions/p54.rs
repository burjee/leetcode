struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = 0;
        let mut h = 0;
        let mut m = matrix[0].len();
        let mut n = matrix.len();
        let mut ans = Vec::with_capacity(m * n);
        while m > 1 && n > 1 {
            for _ in 0..m - 1 {
                ans.push(matrix[v][h]);
                h += 1;
            }
            for _ in 0..n - 1 {
                ans.push(matrix[v][h]);
                v += 1;
            }
            for _ in 0..m - 1 {
                ans.push(matrix[v][h]);
                h -= 1;
            }
            for _ in 0..n - 1 {
                ans.push(matrix[v][h]);
                v -= 1;
            }
            v += 1;
            h += 1;
            m -= 2;
            n -= 2;
        }
        if m == 1 {
            for _ in 0..n {
                ans.push(matrix[v][h]);
                v += 1;
            }
        } else if n == 1 {
            for _ in 0..m {
                ans.push(matrix[v][h]);
                h += 1;
            }
        }
        ans
    }
}

pub fn run() {
    let input = [
        vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
        vec![vec![1], vec![2], vec![3]],
        vec![vec![1, 2, 3]],
        vec![vec![1, 2], vec![3, 4]],
        vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]],
        vec![vec![1]],
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
    ];

    for matrix in input {
        println!("{:?}", Solution::spiral_order(matrix));
    }
}

/* approach2
struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = 0;
        let mut h = 0;
        let mut m = matrix[0].len();
        let mut n = matrix.len();
        let mut ans = Vec::with_capacity(m * n);
        loop {
            for i in h..m {
                ans.push(matrix[v][i])
            }
            v += 1;
            if v == n {
                break;
            }
            for j in v..n {
                ans.push(matrix[j][m - 1])
            }
            m -= 1;
            if h == m {
                break;
            }
            for i in (h..m).rev() {
                ans.push(matrix[n - 1][i])
            }
            n -= 1;
            if v == n {
                break;
            }
            for j in (v..n).rev() {
                ans.push(matrix[j][h])
            }
            h += 1;
            if h == m {
                break;
            }
        }
        ans
    }
}

pub fn run() {
    let input = [
        vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
        vec![vec![1], vec![2], vec![3]],
        vec![vec![1, 2, 3]],
        vec![vec![1, 2], vec![3, 4]],
        vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]],
        vec![vec![1]],
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
    ];

    for matrix in input {
        println!("{:?}", Solution::spiral_order(matrix));
    }
}
 */
