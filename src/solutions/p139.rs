use crate::utils::string::strs_to_string;
use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut set = HashSet::new();
        for word in word_dict {
            set.insert(word);
        }

        let mut cache = vec![None; s.len()];
        Solution::helper(&s, 0, &set, &mut cache)
    }

    pub fn helper(
        s: &str,
        left: usize,
        set: &HashSet<String>,
        cache: &mut Vec<Option<bool>>,
    ) -> bool {
        if left >= s.len() {
            return true;
        }
        if let Some(b) = cache[left] {
            return b;
        }

        for right in left + 1..=s.len() {
            if set.contains(&s[left..right]) && Solution::helper(s, right, set, cache) {
                return true;
            }
        }

        cache[left] = Some(false);
        false
    }
}

pub fn run() {
    let input = [
        (
            "leetcode",
            vec!["leet", "code"],
        ),
        (
            "applepenapple",
            vec!["apple", "pen"],
        ),
        (
            "catsandog",
            vec![
                "cats",
                "dog",
                "sand",
                "and",
                "cat",
            ],
        ),
        (
            "catscatsing",
            vec!["cats", "cat", "sing"],
        ),
        (
            "aaabbbc",
            vec!["aaa", "bbb"],
        ),
        (
            "cacacacat",
            vec!["ca", "cat"],
        ),
        ("ca", vec!["cagg"]),
        (
            "bb",
            vec![
                "a",
                "b",
                "bbb",
                "bbbb",
            ],
        ),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
            vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
        ),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
        )
    ];

    for (s, word_dict) in input {
        println!(
            "{}",
            Solution::word_break(s.to_string(), strs_to_string(word_dict))
        );
    }
}

/* dp
use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut set = HashSet::new();
        for word in word_dict {
            set.insert(word);
        }

        let mut arrival = vec![false; s.len() + 1];
        arrival[0] = true;

        for right in 1..=s.len() {
            for left in 0..right {
                if arrival[left] && set.contains(&s[left..right]) {
                    arrival[right] = true;
                    break;
                }
            }
        }

        arrival[s.len()]
    }
}

pub fn run() {
    let input = [
        (
            "leetcode",
            vec!["leet", "code"],
        ),
        (
            "applepenapple",
            vec!["apple", "pen"],
        ),
        (
            "catsandog",
            vec![
                "cats",
                "dog",
                "sand",
                "and",
                "cat",
            ],
        ),
        (
            "catscatsing",
            vec!["cats", "cat", "sing"],
        ),
        (
            "aaabbbc",
            vec!["aaa", "bbb"],
        ),
        (
            "cacacacat",
            vec!["ca", "cat"],
        ),
        ("ca", vec!["cagg"]),
        (
            "bb",
            vec![
                "a",
                "b",
                "bbb",
                "bbbb",
            ],
        ),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
            vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
        ),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
        )
    ];

    for (s, word_dict) in input {
        println!(
            "{}",
            Solution::word_break(s.to_string(), strs_to_string(word_dict))
        );
    }
}
 */
