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

fn main() {
    let input = vec![
        (
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()],
        ),
        (
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()],
        ),
        (
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string(),
            ],
        ),
        (
            "catscatsing".to_string(),
            vec!["cats".to_string(), "cat".to_string(), "sing".to_string()],
        ),
        (
            "aaabbbc".to_string(),
            vec!["aaa".to_string(), "bbb".to_string()],
        ),
        (
            "cacacacat".to_string(),
            vec!["ca".to_string(), "cat".to_string()],
        ),
        ("ca".to_string(), vec!["cagg".to_string()]),
        (
            "bb".to_string(),
            vec![
                "a".to_string(),
                "b".to_string(),
                "bbb".to_string(),
                "bbbb".to_string(),
            ],
        ),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
            vec!["a".to_string(),"aa".to_string(),"aaa".to_string(),"aaaa".to_string(),"aaaaa".to_string(),"aaaaaa".to_string(),"aaaaaaa".to_string(),"aaaaaaaa".to_string(),"aaaaaaaaa".to_string(),"aaaaaaaaaa".to_string()]
        ),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            vec!["a".to_string(),"aa".to_string(),"aaa".to_string(),"aaaa".to_string(),"aaaaa".to_string(),"aaaaaa".to_string(),"aaaaaaa".to_string(),"aaaaaaaa".to_string(),"aaaaaaaaa".to_string(),"aaaaaaaaaa".to_string()]
        )
    ];
    for (s, word_dict) in input {
        println!("{}", Solution::word_break(s, word_dict));
    }
}
