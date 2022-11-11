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

fn main() {
    let input = vec![
        vec![
            "ksqvsyq".to_string(),
            "ks".to_string(),
            "kss".to_string(),
            "czvh".to_string(),
            "zczpzvdhx".to_string(),
            "zczpzvh".to_string(),
            "zczpzvhx".to_string(),
            "zcpzvh".to_string(),
            "zczvh".to_string(),
            "gr".to_string(),
            "grukmj".to_string(),
            "ksqvsq".to_string(),
            "gruj".to_string(),
            "kssq".to_string(),
            "ksqsq".to_string(),
            "grukkmj".to_string(),
            "grukj".to_string(),
            "zczpzfvdhx".to_string(),
            "gru".to_string(),
        ],
        vec![
            "bdca".to_string(),
            "bda".to_string(),
            "ca".to_string(),
            "dca".to_string(),
            "a".to_string(),
        ],
        vec![
            "a".to_string(),
            "b".to_string(),
            "ab".to_string(),
            "bac".to_string(),
        ],
        vec![
            "a".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "bca".to_string(),
            "bda".to_string(),
            "bdca".to_string(),
        ],
        vec![
            "a".to_string(),
            "b".to_string(),
            "bca".to_string(),
            "bda".to_string(),
            "bdca".to_string(),
        ],
        vec![
            "xbc".to_string(),
            "pcxbcf".to_string(),
            "xb".to_string(),
            "cxbc".to_string(),
            "pcxbc".to_string(),
        ],
        vec!["abcd".to_string(), "dbqca".to_string()],
    ];

    for words in input {
        println!("{}", Solution::longest_str_chain(words));
    }
}
