struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }
}

fn main() {
    let input = vec![121, -121, 10, 1221];

    for x in input {
        println!("{}", Solution::is_palindrome(x));
    }
}
