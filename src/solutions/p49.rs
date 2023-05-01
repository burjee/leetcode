use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs {
            let mut count: [u8; 26] = [0; 26];
            for &b in s.as_bytes() {
                count[b as usize - 97] += 1;
            }
            let vec = map.entry(count).or_insert(Vec::new());
            vec.push(s);
        }
        map.drain().map(|(_, v)| v).collect()
    }
}

pub fn run() {
    let input = vec![
        to_string(vec!["eat", "tea", "tan", "ate", "nat", "bat"]),
        to_string(vec!["baa", "aba", "bab"]),
        to_string(vec![""]),
        to_string(vec!["a"]),
    ];

    for strs in input {
        println!("{:?}", Solution::group_anagrams(strs));
    }
}

fn to_string(strs: Vec<&str>) -> Vec<String> {
    let mut vec = Vec::new();
    for s in strs {
        vec.push(String::from(s));
    }
    vec
}
