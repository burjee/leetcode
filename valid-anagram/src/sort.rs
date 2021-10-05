struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_s: Vec<char> = s.chars().collect();
        let mut char_t: Vec<char> = t.chars().collect();
        char_s.sort();
        char_t.sort();
        char_s == char_t
    }
}

fn main() {
    let input = vec![
        ("anagram".to_string(), "nagaram".to_string()),
        ("rat".to_string(), "car".to_string()),
        ("asdfg".to_string(), "gfasd".to_string()),
    ];
    for (s, t) in input {
        println!("{}", Solution::is_anagram(s, t));
    }
}
