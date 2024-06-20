use crate::utils::string::strs_to_string;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution {}
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();
        for word in words {
            let counter = map.entry(word).or_insert(0);
            *counter += 1;
        }
        let mut heap: BinaryHeap<(i32, Reverse<String>)> =
            map.into_iter().map(|(k, v)| (v, Reverse(k))).collect();
        let mut ans = vec![];
        for _ in 0..k {
            let (_, Reverse(s)) = heap.pop().unwrap();
            ans.push(s);
        }
        ans
    }
}

pub fn run() {
    let input = [
        (vec!["i", "love", "leetcode", "i", "love", "coding"], 2),
        (
            vec![
                "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
            ],
            4,
        ),
        (vec!["i", "love", "leetcode", "i", "love", "coding"], 3),
    ];

    for (words, k) in input {
        println!("{:?}", Solution::top_k_frequent(strs_to_string(words), k));
    }
}

/* sort
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();
        for word in words {
            let counter = map.entry(word).or_insert(0);
            *counter += 1;
        }
        let mut ans = map.into_iter().collect::<Vec<(String, i32)>>();
        ans.sort_by(|a, b| {
            if b.1 == a.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        (&ans[0..k as usize])
            .iter()
            .map(|word| word.0.clone())
            .collect()
    }
}

 */
