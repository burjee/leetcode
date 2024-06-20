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
        (&s[ans[0]..ans[1]]).to_string()
    }
}

// vec.rotate_right
// vec.retain
// vec.swap_remove
pub fn run() {
    let input = [
        "babad",
        "cbbd",
        "a",
        "ac",
        "aa",
        "athjuiodsf",
        "aaaa",
        "aaaaccbb",
        "abcdcba",
        "aabbccbbaa",
        "abccba",
        "aaaba",
        "aaaabcviivcbaaaa",
        "abcbavvabcbawerrewabcbavvabcba",
    ];

    for s in input {
        println!("{}", Solution::longest_palindrome(s.to_string()));
    }
}
