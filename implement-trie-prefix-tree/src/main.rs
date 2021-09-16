use std::collections::HashMap;

const ASCII_A: u8 = 'a' as u8;

pub struct Trie {
    next: HashMap<u8, Trie>,
    is_word: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            next: HashMap::new(),
            is_word: false,
        }
    }

    fn insert(&mut self, word: String) {
        let bytes = Trie::string_to_ascii_bytes(word);
        self.insert_by_bytes(0, &bytes)
    }

    fn insert_by_bytes(&mut self, i: usize, bytes: &Vec<u8>) {
        if i < bytes.len() {
            let next = self.next.entry(bytes[i]).or_insert(Trie::new());
            next.insert_by_bytes(i + 1, bytes);
        } else {
            self.is_word = true;
        }
    }

    fn search(&self, word: String) -> bool {
        let bytes = Trie::string_to_ascii_bytes(word);
        self.search_by_bytes(0, &bytes)
    }

    fn search_by_bytes(&self, i: usize, bytes: &Vec<u8>) -> bool {
        if i == bytes.len() {
            return self.is_word;
        }
        if let Some(trie) = self.next.get(&bytes[i]) {
            return trie.search_by_bytes(i + 1, bytes);
        }
        false
    }

    fn starts_with(&self, prefix: String) -> bool {
        let bytes = Trie::string_to_ascii_bytes(prefix);
        self.starts_with_by_bytes(0, &bytes)
    }

    fn starts_with_by_bytes(&self, i: usize, bytes: &Vec<u8>) -> bool {
        if i == bytes.len() {
            return true;
        }
        if let Some(trie) = self.next.get(&bytes[i]) {
            return trie.starts_with_by_bytes(i + 1, bytes);
        }
        false
    }

    fn string_to_ascii_bytes(s: String) -> Vec<u8> {
        s.as_bytes().iter().map(|&b| b - ASCII_A).collect()
    }
}

fn main() {
    enum Cmd {
        Trie,
        Insert,
        Search,
        StartsWith,
    }

    let input = vec![
        (
            vec![
                Cmd::Trie,
                Cmd::Insert,
                Cmd::Search,
                Cmd::Search,
                Cmd::StartsWith,
                Cmd::Insert,
                Cmd::Search,
            ],
            vec![
                vec![],
                vec!["apple".to_string()],
                vec!["apple".to_string()],
                vec!["app".to_string()],
                vec!["app".to_string()],
                vec!["app".to_string()],
                vec!["app".to_string()],
            ],
        ),
        (
            vec![
                Cmd::Trie,
                Cmd::Insert,
                Cmd::Search,
                Cmd::Search,
                Cmd::StartsWith,
                Cmd::Insert,
                Cmd::Search,
            ],
            vec![
                vec![],
                vec!["google".to_string()],
                vec!["google".to_string()],
                vec!["g".to_string()],
                vec!["google".to_string()],
                vec!["yooooo".to_string()],
                vec!["yoooo".to_string()],
            ],
        ),
        (
            vec![
                Cmd::Trie,
                Cmd::Insert,
                Cmd::Search,
                Cmd::Search,
                Cmd::StartsWith,
                Cmd::StartsWith,
            ],
            vec![
                vec![],
                vec!["yohaha".to_string()],
                vec!["haha".to_string()],
                vec!["yoha".to_string()],
                vec!["y".to_string()],
                vec!["yoha".to_string()],
            ],
        ),
    ];

    let mut trie = Trie::new();
    for (cmd, words) in input {
        for i in 0..cmd.len() {
            match cmd[i] {
                Cmd::Trie => {
                    trie = Trie::new();
                    print!("null, ")
                }
                Cmd::Insert => {
                    for word in &words[i] {
                        trie.insert(word.clone());
                        print!("null, ")
                    }
                }
                Cmd::Search => {
                    for word in &words[i] {
                        print!("{}, ", trie.search(word.clone()));
                    }
                }
                Cmd::StartsWith => {
                    for word in &words[i] {
                        print!("{}, ", trie.starts_with(word.clone()));
                    }
                }
            }
        }
        println!();
    }
}
