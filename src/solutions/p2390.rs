struct Solution {}
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut ans = String::new();

        for c in s.chars() {
            if c == '*' {
                ans.pop();
            } else {
                ans.push(c);
            }
        }

        ans
    }
}

pub fn run() {
    let input = ["leet**cod*e", "erase*****"];

    for s in input {
        println!("{}", Solution::remove_stars(s.to_string()));
    }
}
