struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = format!("^{}$", s);
        let v: Vec<char> = s.chars().collect();
        let mut ans: [usize; 2] = [1, 2];
        let mut pos = Vec::new();
        for (i, &c) in v.iter().enumerate() {
            for j in (0..pos.len()).rev() {
                pos[j] -= 1;
                if v[pos[j]] != c {
                    let len = i - pos[j] - 1;
                    if len > ans[1] - ans[0] {
                        ans[0] = pos[j] + 1;
                        ans[1] = i;
                    }
                    pos.swap_remove(j);
                }
            }
            // type aa
            if i > 0 && c == v[i - 1] {
                pos.push(i - 1);
            }
            // type aba
            if i > 1 && c == v[i - 2] {
                pos.push(i - 2);
            }
        }
        String::from(&s[ans[0]..ans[1]])
    }
}

// vec.rotate_right
// vec.retain
// vec.swap_remove
fn main() {
    let string = vec![
        String::from("babad"),
        String::from("cbbd"),
        String::from("a"),
        String::from("ac"),
        String::from("aa"),
        String::from("athjuiodsf"),
        String::from("aaaa"),
        String::from("aaaaccbb"),
        String::from("abcdcba"),
        String::from("aabbccbbaa"),
        String::from("abccba"),
        String::from("aaaba"),
        String::from("aaaabcviivcbaaaa"),
        String::from("abcbavvabcbawerrewabcbavvabcba"),
    ];
    for s in string {
        println!("ans: {}", Solution::longest_palindrome(s));
    }
}
