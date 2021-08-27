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
