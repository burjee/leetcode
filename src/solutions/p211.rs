use std::collections::HashMap;

struct WordDictionary {
    next: HashMap<char, WordDictionary>,
    is_word: bool,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            next: HashMap::new(),
            is_word: false,
        }
    }

    fn add_word(&mut self, word: String) {
        let chars: Vec<char> = word.chars().collect();
        self.add_word_by_chars(0, &chars);
    }

    fn add_word_by_chars(&mut self, i: usize, chars: &Vec<char>) {
        if i < chars.len() {
            let next = self.next.entry(chars[i]).or_insert(WordDictionary::new());
            next.add_word_by_chars(i + 1, chars);
        } else {
            self.is_word = true;
        }
    }

    fn search(&self, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        self.search_by_chars(0, &chars)
    }

    fn search_by_chars(&self, i: usize, chars: &Vec<char>) -> bool {
        if i == chars.len() {
            return self.is_word;
        }
        if chars[i] == '.' {
            for (_, wd) in self.next.iter() {
                if wd.search_by_chars(i + 1, chars) {
                    return true;
                }
            }
        } else if let Some(wd) = self.next.get(&chars[i]) {
            return wd.search_by_chars(i + 1, chars);
        }
        false
    }
}

pub fn run() {
    enum Cmd {
        WordDictionary,
        AddWord,
        Search,
    }

    let input = vec![
        (
            vec![
                Cmd::WordDictionary,
                Cmd::AddWord,
                Cmd::AddWord,
                Cmd::AddWord,
                Cmd::Search,
                Cmd::Search,
                Cmd::Search,
                Cmd::Search,
            ],
            vec![
                vec![],
                vec!["bad".to_string()],
                vec!["dad".to_string()],
                vec!["mad".to_string()],
                vec!["pad".to_string()],
                vec!["bad".to_string()],
                vec![".ad".to_string()],
                vec!["b..".to_string()],
            ],
        ),
        (
            vec![
                Cmd::WordDictionary,
                Cmd::AddWord,
                Cmd::AddWord,
                Cmd::Search,
                Cmd::Search,
                Cmd::Search,
                Cmd::Search,
            ],
            vec![
                vec![],
                vec!["apple".to_string()],
                vec!["google".to_string()],
                vec!["ap..e".to_string()],
                vec!["...le".to_string()],
                vec![".oo.e".to_string()],
                vec!["a.p.e".to_string()],
            ],
        ),
    ];

    let mut wd = WordDictionary::new();
    for (cmd, words) in input {
        for i in 0..cmd.len() {
            match cmd[i] {
                Cmd::WordDictionary => {
                    wd = WordDictionary::new();
                    print!("null, ")
                }
                Cmd::AddWord => {
                    for word in &words[i] {
                        wd.add_word(word.clone());
                        print!("null, ")
                    }
                }
                Cmd::Search => {
                    for word in &words[i] {
                        print!("{}, ", wd.search(word.clone()));
                    }
                }
            }
        }
        println!();
    }
}
