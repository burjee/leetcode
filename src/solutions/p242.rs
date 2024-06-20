struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = [0; 26];
        let byte_s = s.as_bytes();
        let byte_c = t.as_bytes();
        // b'a' = 97
        for i in 0..s.len() {
            count[byte_s[i] as usize - 97] += 1;
            count[byte_c[i] as usize - 97] -= 1;
        }
        count == [0; 26]
    }
}

pub fn run() {
    let input = [("anagram", "nagaram"), ("rat", "car"), ("asdfg", "gfasd")];

    for (s, t) in input {
        println!("{}", Solution::is_anagram(s.to_string(), t.to_string()));
    }
}

/* sort
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

pub fn run() {
    let input = [
        ("anagram".to_string(), "nagaram".to_string()),
        ("rat".to_string(), "car".to_string()),
        ("asdfg".to_string(), "gfasd".to_string()),
    ];

    for (s, t) in input {
        println!("{}", Solution::is_anagram(s, t));
    }
}
 */
