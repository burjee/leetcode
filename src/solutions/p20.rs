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
    let string = vec![
        String::from("["),
        String::from("]"),
        String::from("()"),
        String::from("()[]{}"),
        String::from("(]"),
        String::from("([)]"),
        String::from("{[]}"),
        String::from("{{}[][[[]]]}"),
    ];
    for s in string {
        println!("{}", Solution::is_valid(s));
    }
}
