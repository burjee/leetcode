struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim_end();
        let mut l = 0;
        for c in s.chars().rev() {
            if c != ' ' {
                l += 1;
            } else {
                break;
            }
        }
        l
    }
}

pub fn run() {
    let input = [
        "Hello World".to_string(),
        "   fly me   to   the moon  ".to_string(),
        "luffy is still joyboy".to_string(),
    ];
    for s in input {
        println!("{}", Solution::length_of_last_word(s));
    }
}
