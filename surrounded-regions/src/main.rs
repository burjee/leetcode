struct Solution {}
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();
        if m < 3 || n < 3 {
            return;
        }

        for j in 0..n {
            Solution::helper(board, 0, j);
            Solution::helper(board, m - 1, j);
        }
        for i in 1..m - 1 {
            Solution::helper(board, i, 0);
            Solution::helper(board, i, n - 1);
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == 'T' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    pub fn helper(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if board[i][j] == 'O' {
            board[i][j] = 'T';

            if i > 0 {
                Solution::helper(board, i - 1, j);
            }
            if i < board.len() - 1 {
                Solution::helper(board, i + 1, j);
            }
            if j > 0 {
                Solution::helper(board, i, j - 1);
            }
            if j < board[0].len() - 1 {
                Solution::helper(board, i, j + 1);
            }
        }
    }
}

fn main() {
    let input = vec![
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ],
        vec![vec!['X']],
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'O'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'O'],
        ],
        vec![vec!['O', 'X', 'X', 'O'], vec!['X', 'O', 'O', 'X']],
    ];
    for mut board in input {
        Solution::solve(&mut board);
        for row in board {
            println!("{:?}", row);
        }
        println!();
    }
}
