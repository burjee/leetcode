use crate::utils::string::strs_to_string;
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words
            .into_iter()
            .map(|word| word.into_bytes())
            .collect::<Vec<_>>();
        words.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut output = 1;
        let mut level = 0;
        let mut layer0 = HashMap::new();
        let mut layer1 = HashMap::new();

        for word in &words {
            let len = word.len();

            if layer0.is_empty() || level == len {
                layer0.insert(word, 1);
                level = len;
                continue;
            }

            if level < len - 1 {
                layer0.clear();
                std::mem::swap(&mut layer0, &mut layer1);
                level += 1;
            }

            if level < len - 1 {
                layer0.clear();
                level = len;
                layer0.insert(word, 1);
                continue;
            }

            layer1.insert(word, 1);
            for (&key, &value) in &layer0 {
                if Solution::is_predecessor(key, word) {
                    let l = layer1.get_mut(word).unwrap();
                    *l = *l.max(&mut (value + 1));
                    output = output.max(*l);
                }
            }
        }

        output
    }

    pub fn is_predecessor(a: &[u8], b: &[u8]) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < a.len() {
            if a[i] == b[j] {
                i += 1;
            }
            j += 1;
            if j - i > 1 {
                return false;
            }
        }
        true
    }
}

pub fn run() {
    let input = [
        vec![
            "ksqvsyq",
            "ks",
            "kss",
            "czvh",
            "zczpzvdhx",
            "zczpzvh",
            "zczpzvhx",
            "zcpzvh",
            "zczvh",
            "gr",
            "grukmj",
            "ksqvsq",
            "gruj",
            "kssq",
            "ksqsq",
            "grukkmj",
            "grukj",
            "zczpzfvdhx",
            "gru",
        ],
        vec!["bdca", "bda", "ca", "dca", "a"],
        vec!["a", "b", "ab", "bac"],
        vec!["a", "b", "ba", "bca", "bda", "bdca"],
        vec!["a", "b", "bca", "bda", "bdca"],
        vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"],
        vec!["abcd", "dbqca"],
    ];

    for words in input {
        println!("{}", Solution::longest_str_chain(strs_to_string(words)));
    }
}
