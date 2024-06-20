use std::cmp::max;
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut start = 0;
        let mut map = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if map.contains_key(&c) {
                let pos = map.get(&c).unwrap();
                if *pos >= start {
                    start = pos + 1;
                } else {
                    map.remove(&c);
                }
            }
            ans = max(ans, i as i32 - start + 1);
            map.insert(c, i as i32);
        }
        ans
    }
}

pub fn run() {
    let input = [
        "dvdf",
        "a",
        " ",
        "abcabcbb",
        "bbbbb",
        "pwwkew",
        "",
        "tghshtte",
        "redsfgaffer",
        "ffdsfdsfsdf",
        "asdqweert",
        "fffasfaffsd",
        "pmfzsevcfsegvs",
    ];

    for s in input {
        println!("{}", Solution::length_of_longest_substring(s.to_string()));
    }
}
