use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    node: HashMap<char, Trie>,
    word: Option<String>,
}
impl Trie {
    pub fn new(words: Vec<String>) -> Trie {
        let mut trie: Trie = Default::default();
        for word in words {
            trie.add_word(word);
        }
        trie
    }

    pub fn add_word(&mut self, word: String) {
        let mut point = self;
        for c in word.chars() {
            let node = point.node.entry(c).or_default();
            point = node;
        }
        point.word = Some(word);
    }
}

struct WordBoard {
    board: Vec<Vec<char>>,
    used: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}
impl WordBoard {
    pub fn new(board: Vec<Vec<char>>) -> WordBoard {
        let (rows, cols) = (board.len(), board[0].len());
        let used = vec![vec![false; cols]; rows];
        WordBoard {
            board,
            used,
            rows,
            cols,
        }
    }

    pub fn search(&mut self, i: usize, j: usize, trie: &mut Trie, output: &mut Vec<String>) {
        if i >= self.rows || j >= self.cols || self.used[i][j] {
            return;
        }
        let current_char = self.board[i][j];
        if let Some(node) = trie.node.get_mut(&current_char) {
            if node.word.is_some() {
                output.push(node.word.take().unwrap());
            }

            self.used[i][j] = true;
            if i > 0 {
                self.search(i - 1, j, node, output);
            }
            if j > 0 {
                self.search(i, j - 1, node, output);
            }
            self.search(i + 1, j, node, output);
            self.search(i, j + 1, node, output);
            self.used[i][j] = false;
        }
    }
}

struct Solution {}
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let (m, n) = (board.len(), board[0].len());
        let mut trie = Trie::new(words);
        let mut word_board = WordBoard::new(board);

        let mut output = vec![];
        for i in 0..m {
            for j in 0..n {
                word_board.search(i, j, &mut trie, &mut output);
            }
        }
        output
    }
}

pub fn run() {
    let input = vec![
        (
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            vec![
                "oath".to_string(),
                "pea".to_string(),
                "eat".to_string(),
                "rain".to_string(),
            ],
        ),
        (
            vec![vec!['a', 'b'], vec!['c', 'd']],
            vec!["abcb".to_string(), "abcd".to_string(), "abdc".to_string()],
        ),
        (
            vec![
                vec!['o', 'a', 'b', 'n'],
                vec!['o', 't', 'a', 'e'],
                vec!['a', 'h', 'k', 'r'],
                vec!['a', 'f', 'l', 'v'],
            ],
            vec!["oa".to_string(), "oaa".to_string()],
        ),
        (
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            vec![
                "oath".to_string(),
                "pea".to_string(),
                "eat".to_string(),
                "rain".to_string(),
                "oathi".to_string(),
                "oathk".to_string(),
                "oathf".to_string(),
                "oate".to_string(),
                "oathii".to_string(),
                "oathfi".to_string(),
                "oathfii".to_string(),
            ],
        ),
    ];
    for (board, words) in input {
        println!("{:?}", Solution::find_words(board, words));
    }
}
