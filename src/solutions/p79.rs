use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let m = board.len();
        let n = board[0].len();

        let mut char_set = HashSet::new();
        for row in &board {
            for c in row {
                char_set.insert(c);
            }
        }
        for c in &chars {
            if !char_set.contains(&c) {
                return false;
            }
        }

        for i in 0..m {
            for j in 0..n {
                let mut path = HashSet::new();
                if Solution::find(i, j, &board, &mut path, &chars, 0) {
                    return true;
                }
            }
        }
        false
    }

    pub fn find(
        i: usize,
        j: usize,
        board: &Vec<Vec<char>>,
        path: &mut HashSet<(usize, usize)>,
        chars: &Vec<char>,
        mut at: usize,
    ) -> bool {
        if board[i][j] == chars[at] {
            at += 1;
            if at == chars.len() {
                return true;
            }

            path.insert((i, j));
            if i > 0 && !path.contains(&(i - 1, j)) {
                if Solution::find(i - 1, j, board, path, chars, at) {
                    return true;
                }
            }
            if i < board.len() - 1 && !path.contains(&(i + 1, j)) {
                if Solution::find(i + 1, j, board, path, chars, at) {
                    return true;
                }
            }
            if j > 0 && !path.contains(&(i, j - 1)) {
                if Solution::find(i, j - 1, board, path, chars, at) {
                    return true;
                }
            }
            if j < board[0].len() - 1 && !path.contains(&(i, j + 1)) {
                if Solution::find(i, j + 1, board, path, chars, at) {
                    return true;
                }
            }
            path.remove(&(i, j));
        }
        false
    }
}

pub fn run() {
    let input = [
        (
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            String::from("ABCCED"),
        ),
        (
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            String::from("SEE"),
        ),
        (
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            String::from("ABCB"),
        ),
        (
            vec![
                vec!['Q', 'A', 'Q', 'Q'],
                vec!['C', 'B', 'C', 'Q'],
                vec!['Q', 'Q', 'D', 'Q'],
            ],
            String::from("ABCD"),
        ),
        (vec![vec!['A']], String::from("B")),
        (vec![vec!['B']], String::from("B")),
    ];

    for (board, word) in input {
        println!("{}", Solution::exist(board, word));
    }
}
