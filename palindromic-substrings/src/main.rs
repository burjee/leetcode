struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        for i in 0..s.len() - 1 {
            count += Solution::helper(&chars, i, i + 1);
            count += Solution::helper(&chars, i, i);
        }
        count + 1
    }

    pub fn helper(chars: &Vec<char>, mut l: usize, mut r: usize) -> i32 {
        let mut count = 0;
        while chars[l] == chars[r] {
            count += 1;
            if l == 0 || r == chars.len() - 1 {
                break;
            }
            l -= 1;
            r += 1;
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
