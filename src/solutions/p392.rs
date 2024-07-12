struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
            }
            j += 1;
        }
        i == s.len()
    }
}

pub fn run() {
    let input = [("abc", "ahbgdc"), ("axc", "ahbgdc")];

    for (s, t) in input {
        println!("{}", Solution::is_subsequence(s.to_string(), t.to_string()));
    }
}
