pub struct Trie {
    next: [Option<Box<Trie>>; 26],
    word: Option<String>,
}
impl Trie {
    pub fn new() -> Trie {
        Trie {
            next: Default::default(),
            word: None,
        }
    }

    pub fn insert(&mut self, word: String) {
        let chars = word.chars().collect();
        self.insert_char(&chars, 0);
    }

    fn insert_char(&mut self, chars: &Vec<char>, i: usize) {
        if i == chars.len() {
            let word = chars.into_iter().collect();
            self.word = Some(word);
        } else {
            let j = chars[i] as usize - 97;
            let next = self.next[j].get_or_insert(Box::new(Trie::new()));
            next.insert_char(chars, i + 1);
        }
    }

    pub fn search(&self, word: String) -> Vec<String> {
        let chars = word.chars().collect();
        let mut result = vec![];
        self.search_word(&chars, 0, &mut result);
        result
    }

    fn search_word(&self, chars: &Vec<char>, i: usize, result: &mut Vec<String>) {
        if i < chars.len() {
            let j = chars[i] as usize - 97;
            if let Some(next) = &self.next[j] {
                next.search_word(chars, i + 1, result);
            }
        } else if result.len() < 3 {
            if let Some(s) = &self.word {
                result.push(s.clone());
            }
            for j in 0..26 {
                if let Some(next) = &self.next[j] {
                    next.search_word(chars, i + 1, result)
                }
            }
        }
    }
}

struct Solution {}
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for product in products {
            trie.insert(product);
        }

        let mut ans = Vec::with_capacity(search_word.len());
        for i in 1..=search_word.len() {
            ans.push(trie.search(search_word[0..i].to_string()));
        }
        ans
    }
}

pub fn run() {
    use crate::utils::string::strs_to_string;

    let input = [
        (vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"], "mouse"),
        (vec!["havana"], "havana"),
    ];

    for (products, search_word) in input {
        println!(
            "{:?}",
            Solution::suggested_products(strs_to_string(products), search_word.to_string())
        );
    }
}
