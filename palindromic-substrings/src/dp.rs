struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut map = vec![vec![false; s.len()]; chars.len()];
        let mut count = 0;
        for r in 0..s.len() {
            for l in 0..=r {
                if chars[l] == chars[r] {
                    if l == r || l + 1 == r || map[l + 1][r - 1] {
                        count += 1;
                        map[l][r] = true;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let input = vec![
        "abc".to_string(),
        "aaa".to_string(),
        "aaffifjaa".to_string(),
        "aba".to_string(),
        "aabbcca".to_string(),
        "aaeeddeeaa".to_string(),
        "aaggggaa".to_string(),
        "abccba".to_string(),
    ];
    for s in input {
        println!("{}", Solution::count_substrings(s));
    }
}
