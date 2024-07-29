struct Solution {}
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut fresh = 0;
        let mut oranges = std::collections::VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    oranges.push_back((i, j));
                } else if grid[i][j] == 1 {
                    fresh += 1;
                }
            }
        }
        if fresh == 0 {
            return 0;
        }

        let mut minutes = -1;
        while !oranges.is_empty() {
            minutes += 1;
            for _ in 0..oranges.len() {
                let (i, j) = oranges.pop_front().unwrap();
                if i > 0 && grid[i - 1][j] == 1 {
                    grid[i - 1][j] = 2;
                    fresh -= 1;
                    oranges.push_back((i - 1, j));
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    grid[i][j - 1] = 2;
                    fresh -= 1;
                    oranges.push_back((i, j - 1));
                }
                if i < m - 1 && grid[i + 1][j] == 1 {
                    grid[i + 1][j] = 2;
                    fresh -= 1;
                    oranges.push_back((i + 1, j));
                }
                if j < n - 1 && grid[i][j + 1] == 1 {
                    grid[i][j + 1] = 2;
                    fresh -= 1;
                    oranges.push_back((i, j + 1));
                }
            }
        }

        if fresh == 0 {
            minutes
        } else {
            -1
        }
    }
}

pub fn run() {
    let input = [
        vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]],
        vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]],
        vec![vec![0, 2]],
        vec![vec![0]],
    ];

    for grid in input {
        println!("{}", Solution::oranges_rotting(grid));
    }
}
