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

fn main() {
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
        let words = to_str(words);
        println!("{:?}", Solution::top_k_frequent(words, k));
    }
}

fn to_str(words: Vec<&str>) -> Vec<String> {
    let mut w = Vec::with_capacity(words.len());
    for word in words {
        w.push(word.to_string());
    }
    w
}
