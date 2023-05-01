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
    let strs = vec![
        String::from("dvdf"),
        String::from("a"),
        String::from(" "),
        String::from("abcabcbb"),
        String::from("bbbbb"),
        String::from("pwwkew"),
        String::from(""),
        String::from("tghshtte"),
        String::from("redsfgaffer"),
        String::from("ffdsfdsfsdf"),
        String::from("asdqweert"),
        String::from("fffasfaffsd"),
        String::from("pmfzsevcfsegvs"),
    ];

    for s in strs {
        let res = Solution::length_of_longest_substring(s);
        println!("res: {}", res);
    }
}
