struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .replace(|c: char| !c.is_ascii_alphanumeric(), "")
            .to_ascii_lowercase();
        let r = s.chars().rev().collect::<String>();
        s == r
    }
}

fn main() {
    let input = vec![
        "A man, a plan, a canal: Panama".to_string(),
        "race a car".to_string(),
        " ".to_string(),
    ];
    for s in input {
        println!("{}", Solution::is_palindrome(s));
    }
}
