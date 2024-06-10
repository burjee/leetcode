use std::collections::VecDeque;

// 7 8 9
// 4 5 6
// 1 2 3
const DIRECTIONS: [(i32, i32); 8] = [
    (1, 1),   // 3
    (0, 1),   // 6
    (1, 0),   // 2
    (1, -1),  // 1
    (-1, 1),  // 9
    (0, -1),  // 4
    (-1, 0),  // 8
    (-1, -1), // 7
];

struct Solution {}
impl Solution {
    pub fn check_boundary(a: i32, b: i32, len: i32) -> bool {
        a >= 0 && b >= 0 && a <= len && b <= len
    }

    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }

        let mut gone = vec![vec![false; grid.len()]; grid.len()];
        let mut cells = VecDeque::new();
        let len = (grid.len() - 1) as i32;
        gone[0][0] = true;
        cells.push_back((0, 0, 1));

        while let Some((i, j, step)) = cells.pop_front() {
            if i == grid.len() - 1 && j == grid.len() - 1 {
                return step;
            }

            for &(x, y) in &DIRECTIONS {
                let a = i as i32 + x;
                let b = j as i32 + y;
                if Solution::check_boundary(a, b, len)
                    && grid[a as usize][b as usize] == 0
                    && !gone[a as usize][b as usize]
                {
                    gone[a as usize][b as usize] = true;
                    cells.push_back((a as usize, b as usize, step + 1));
                }
            }
        }
        -1
    }
}

pub fn run() {
    let input = [
        vec![vec![0]],
        vec![vec![0, 1], vec![1, 0]],
        vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]],
        vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]],
        vec![
            vec![0, 0, 1, 0, 1, 1],
            vec![1, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ],
    ];

    for grid in input {
        println!("{}", Solution::shortest_path_binary_matrix(grid));
    }
}

// dfs tle
// struct Solution {}
// impl Solution {
//     pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
//         let mut path = vec![vec![false; grid.len()]; grid.len()];
//         let mut cache = vec![vec![-1; grid.len()]; grid.len()];
//         let k = grid.len() - 1;
//         Solution::helper(&grid, 0, 0, &mut path, &mut cache, 1);
//         cache[k][k]
//     }

//     pub fn helper(
//         grid: &Vec<Vec<i32>>,
//         i: usize,
//         j: usize,
//         path: &mut Vec<Vec<bool>>,
//         cache: &mut Vec<Vec<i32>>,
//         step: i32,
//     ) {
//         if grid[i][j] == 0 && (cache[i][j] == -1 || step < cache[i][j]) {
//             let k = grid.len() - 1;
//             cache[i][j] = step;
//             path[i][j] = true;
//             // 7 8 9
//             // 4 5 6
//             // 1 2 3

//             // 3
//             if i < k && j < k && !path[i + 1][j + 1] {
//                 Solution::helper(grid, i + 1, j + 1, path, cache, step + 1);
//             }
//             // 6
//             if j < k && !path[i][j + 1] {
//                 Solution::helper(grid, i, j + 1, path, cache, step + 1);
//             }
//             // 2
//             if i < k && !path[i + 1][j] {
//                 Solution::helper(grid, i + 1, j, path, cache, step + 1);
//             }
//             // 1
//             if i > 0 && j < k && !path[i - 1][j + 1] {
//                 Solution::helper(grid, i - 1, j + 1, path, cache, step + 1);
//             }
//             // 9
//             if i < k && j > 0 && !path[i + 1][j - 1] {
//                 Solution::helper(grid, i + 1, j - 1, path, cache, step + 1);
//             }
//             // 4
//             if j > 0 && !path[i][j - 1] {
//                 Solution::helper(grid, i, j - 1, path, cache, step + 1);
//             }
//             // 8
//             if i > 0 && !path[i - 1][j] {
//                 Solution::helper(grid, i - 1, j, path, cache, step + 1);
//             }
//             // 7
//             if i > 0 && j > 0 && !path[i - 1][j - 1] {
//                 Solution::helper(grid, i - 1, j - 1, path, cache, step + 1);
//             }
//             path[i][j] = false;
//         }
//     }
// }
