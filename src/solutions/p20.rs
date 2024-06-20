struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                _ => {
                    if Some(c) != stack.pop() {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

pub fn run() {
    let input = [
        "[",
        "]",
        "()",
        "()[]{}",
        "(]",
        "([)]",
        "{[]}",
        "{{}[][[[]]]}",
    ];

    for s in input {
        println!("{}", Solution::is_valid(s.to_string()));
    }
}
