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

pub fn run() {
    let input = ["A man, a plan, a canal: Panama", "race a car", " "];

    for s in input {
        println!("{}", Solution::is_palindrome(s.to_string()));
    }
}
