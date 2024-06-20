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
    enum Cmd<'a> {
        WordDictionary,
        AddWord(Vec<&'a str>),
        Search(Vec<&'a str>),
    }

    let input = [
        vec![
            Cmd::WordDictionary,
            Cmd::AddWord(vec!["bad"]),
            Cmd::AddWord(vec!["dad"]),
            Cmd::AddWord(vec!["mad"]),
            Cmd::Search(vec!["pad"]),
            Cmd::Search(vec!["bad"]),
            Cmd::Search(vec![".ad"]),
            Cmd::Search(vec!["b.."]),
        ],
        vec![
            Cmd::WordDictionary,
            Cmd::AddWord(vec!["apple"]),
            Cmd::AddWord(vec!["google"]),
            Cmd::Search(vec!["ap..e"]),
            Cmd::Search(vec!["...le"]),
            Cmd::Search(vec![".oo.e"]),
            Cmd::Search(vec!["a.p.e"]),
        ],
    ];

    let mut word_dictionary = WordDictionary::new();
    for commands in input {
        for cmd in commands {
            match cmd {
                Cmd::WordDictionary => {
                    word_dictionary = WordDictionary::new();
                    print!("null, ")
                }
                Cmd::AddWord(words) => {
                    for word in words {
                        word_dictionary.add_word(word.to_string());
                        print!("null, ")
                    }
                }
                Cmd::Search(words) => {
                    for word in words {
                        print!("{}, ", word_dictionary.search(word.to_string()));
                    }
                }
            }
        }
        println!();
    }
}
