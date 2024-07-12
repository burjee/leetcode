struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

pub fn run() {
    let input = ["the sky is blue", "  hello world  ", "a good   example"];

    for s in input {
        println!("{}", Solution::reverse_words(s.to_string()));
    }
}
