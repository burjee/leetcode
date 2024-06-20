struct Grid {
    square: Vec<Vec<i32>>,
    empty_count: i32,
    m: usize,
    n: usize,
}

struct Path {
    can_go: Vec<Vec<bool>>,
    pass_count: i32,
}

struct Solution {}
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut empty_count = 1;
        let mut start = (0, 0);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    empty_count += 1;
                } else if grid[i][j] == 1 {
                    start = (i, j);
                }
            }
        }
        let grid = Grid {
            square: grid,
            empty_count,
            m,
            n,
        };
        let mut path = Path {
            can_go: vec![vec![true; n]; m],
            pass_count: 0,
        };
        Solution::helper(&grid, &mut path, start.0, start.1)
    }

    pub fn helper(grid: &Grid, path: &mut Path, i: usize, j: usize) -> i32 {
        if (grid.square[i][j] == 1 || grid.square[i][j] == 0) && path.can_go[i][j] {
            let mut unique_paths = 0;
            path.can_go[i][j] = false;
            path.pass_count += 1;
            if i > 0 {
                unique_paths += Solution::helper(grid, path, i - 1, j);
            }
            if i < grid.m - 1 {
                unique_paths += Solution::helper(grid, path, i + 1, j);
            }
            if j > 0 {
                unique_paths += Solution::helper(grid, path, i, j - 1);
            }
            if j < grid.n - 1 {
                unique_paths += Solution::helper(grid, path, i, j + 1);
            }
            path.can_go[i][j] = true;
            path.pass_count -= 1;
            return unique_paths;
        } else if grid.square[i][j] == 2 && grid.empty_count == path.pass_count {
            return 1;
        }
        0
    }
}

pub fn run() {
    let input = [
        vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]],
        vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]],
        vec![vec![0, 1], vec![2, 0]],
    ];

    for grid in input {
        println!("{:?}", Solution::unique_paths_iii(grid));
    }
}
