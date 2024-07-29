struct Solution {}
impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len();
        let n = maze[0].len();
        let mut cells = std::collections::VecDeque::new();
        cells.push_back((entrance[0] as usize, entrance[1] as usize, 0));
        maze[entrance[0] as usize][entrance[1] as usize] = '+';

        while !cells.is_empty() {
            for _ in 0..cells.len() {
                let (i, j, c) = cells.pop_front().unwrap();
                if Solution::is_border(m, n, i, j) && (i != entrance[0] as usize || j != entrance[1] as usize) {
                    return c;
                }
                if i > 0 && maze[i - 1][j] == '.' {
                    maze[i - 1][j] = '+';
                    cells.push_back((i - 1, j, c + 1));
                }
                if j > 0 && maze[i][j - 1] == '.' {
                    maze[i][j - 1] = '+';
                    cells.push_back((i, j - 1, c + 1));
                }
                if i < m - 1 && maze[i + 1][j] == '.' {
                    maze[i + 1][j] = '+';
                    cells.push_back((i + 1, j, c + 1));
                }
                if j < n - 1 && maze[i][j + 1] == '.' {
                    maze[i][j + 1] = '+';
                    cells.push_back((i, j + 1, c + 1));
                }
            }
        }
        -1
    }

    pub fn is_border(m: usize, n: usize, i: usize, j: usize) -> bool {
        i == 0 || j == 0 || i == m - 1 || j == n - 1
    }
}

pub fn run() {
    let input = [
        (
            vec![
                vec!['+', '+', '.', '+'],
                vec!['.', '.', '.', '+'],
                vec!['+', '+', '+', '.'],
            ],
            vec![1, 2],
        ),
        (
            vec![vec!['+', '+', '+'], vec!['.', '.', '.'], vec!['+', '+', '+']],
            vec![1, 0],
        ),
        (vec![vec!['.', '+']], vec![0, 0]),
        (
            vec![
                vec!['.', '+', '+', '+', '+'],
                vec!['.', '+', '.', '.', '.'],
                vec!['.', '+', '.', '+', '.'],
                vec!['.', '.', '.', '+', '.'],
                vec!['+', '+', '+', '+', '.'],
            ],
            vec![0, 0],
        ),
        (
            vec![
                vec!['+', '.', '+', '+', '+', '+', '+'],
                vec!['+', '.', '+', '.', '.', '.', '+'],
                vec!['+', '.', '+', '.', '+', '.', '+'],
                vec!['+', '.', '.', '.', '+', '.', '+'],
                vec!['+', '+', '+', '+', '+', '.', '+'],
            ],
            vec![0, 1],
        ),
    ];

    for (maze, entrance) in input {
        println!("{}", Solution::nearest_exit(maze, entrance));
    }
}
