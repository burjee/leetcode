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

pub fn run() {
    enum Cmd<'a> {
        Trie,
        Insert(Vec<&'a str>),
        Search(Vec<&'a str>),
        StartsWith(Vec<&'a str>),
    }

    let input = [
        vec![
            Cmd::Trie,
            Cmd::Insert(vec!["apple"]),
            Cmd::Search(vec!["apple"]),
            Cmd::Search(vec!["app"]),
            Cmd::StartsWith(vec!["app"]),
            Cmd::Insert(vec!["app"]),
            Cmd::Search(vec!["app"]),
        ],
        vec![
            Cmd::Trie,
            Cmd::Insert(vec!["google"]),
            Cmd::Search(vec!["google"]),
            Cmd::Search(vec!["g"]),
            Cmd::StartsWith(vec!["google"]),
            Cmd::Insert(vec!["yooooo"]),
            Cmd::Search(vec!["yoooo"]),
        ],
        vec![
            Cmd::Trie,
            Cmd::Insert(vec!["yohaha"]),
            Cmd::Search(vec!["haha"]),
            Cmd::Search(vec!["yoha"]),
            Cmd::StartsWith(vec!["y"]),
            Cmd::StartsWith(vec!["yoha"]),
        ],
    ];

    let mut trie = Trie::new();
    for commands in input {
        for cmd in commands {
            match cmd {
                Cmd::Trie => {
                    trie = Trie::new();
                    print!("null, ")
                }
                Cmd::Insert(words) => {
                    for word in words {
                        trie.insert(word.to_string());
                        print!("null, ")
                    }
                }
                Cmd::Search(words) => {
                    for word in words {
                        print!("{}, ", trie.search(word.to_string()));
                    }
                }
                Cmd::StartsWith(words) => {
                    for word in words {
                        print!("{}, ", trie.starts_with(word.to_string()));
                    }
                }
            }
        }
        println!();
    }
}
