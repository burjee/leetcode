struct Solution {}
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut number = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    number += 1;
                    Solution::helper(i, j, &mut grid);
                }
            }
        }
        number
    }

    pub fn helper(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
        grid[i][j] = '0';
        if i > 0 && grid[i - 1][j] == '1' {
            Solution::helper(i - 1, j, grid);
        }
        if j > 0 && grid[i][j - 1] == '1' {
            Solution::helper(i, j - 1, grid);
        }
        if i < grid.len() - 1 && grid[i + 1][j] == '1' {
            Solution::helper(i + 1, j, grid);
        }
        if j < grid[0].len() - 1 && grid[i][j + 1] == '1' {
            Solution::helper(i, j + 1, grid);
        }
    }
}

pub fn run() {
    let input = [
        vec![
            vec!['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
            vec!['0', '1', '1', '0', '1', '1', '1', '0', '1', '1'],
            vec!['1', '0', '1', '0', '1', '1', '0', '1', '0', '1'],
            vec!['1', '0', '1', '1', '0', '1', '1', '1', '1', '1'],
            vec!['1', '1', '0', '0', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '0', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
            vec!['0', '1', '1', '0', '1', '1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '1', '0', '1', '1', '1', '1'],
            vec!['0', '1', '1', '1', '1', '1', '0', '1', '1', '1'],
        ],
        vec![
            vec!['1', '0', '0'],
            vec!['1', '1', '1'],
            vec!['0', '0', '1'],
            vec!['1', '0', '1'],
            vec!['1', '1', '1'],
        ],
        vec![
            vec!['0', '1', '0', '0', '0', '1'],
            vec!['0', '1', '0', '1', '0', '1'],
            vec!['1', '1', '1', '1', '1', '1'],
        ],
        vec![
            vec!['0', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0', '1', '1', '0', '0', '0'],
            vec!['1', '1', '1', '0', '1', '1', '1', '0', '0', '0'],
        ],
        vec![
            vec!['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
            vec!['0', '1', '1', '0', '1', '1', '1', '0', '1', '1'],
            vec!['1', '0', '1', '0', '1', '1', '0', '1', '0', '1'],
            vec!['1', '0', '1', '1', '0', '1', '1', '1', '1', '1'],
            vec!['1', '1', '0', '0', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '0', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
            vec!['0', '1', '1', '0', '1', '1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '1', '0', '1', '1', '1', '1'],
            vec!['0', '1', '1', '1', '1', '1', '0', '1', '1', '1'],
        ],
        vec![
            vec!['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1', '1', '1', '1', '1', '1'],
            vec!['0', '1', '1', '1', '0', '1', '1', '1', '1', '1'],
            vec!['1', '1', '0', '1', '1', '0', '0', '0', '0', '1'],
            vec!['1', '0', '1', '0', '1', '0', '0', '1', '0', '1'],
            vec!['1', '0', '0', '1', '1', '1', '0', '1', '0', '0'],
            vec!['0', '0', '1', '0', '0', '1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1', '0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
            vec!['1', '0', '1', '1', '1', '1', '1', '1', '1', '0'],
        ],
        vec![
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '0', '0', '1', '1'],
            vec!['0', '0', '0', '1', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '0', '1', '1'],
        ],
        vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ],
        vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ],
        vec![
            vec!['0', '0', '1', '0', '1'],
            vec!['0', '0', '1', '0', '1'],
            vec!['1', '1', '1', '0', '1'],
            vec!['0', '1', '0', '1', '1'],
        ],
        vec![
            vec!['0', '0', '1', '0', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['1', '1', '1', '0', '1'],
            vec!['0', '1', '0', '1', '1'],
        ],
        vec![
            vec!['0', '0', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ],
        vec![
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '0', '1', '0'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '0', '1', '0'],
        ],
        vec![vec!['0']],
        vec![vec!['1']],
    ];

    for grid in input {
        println!("{}", Solution::num_islands(grid));
    }
}
