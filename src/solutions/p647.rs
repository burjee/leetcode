struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        for i in 0..s.len() - 1 {
            count += Solution::helper(&chars, i, i + 1);
            count += Solution::helper(&chars, i, i);
        }
        count + 1
    }

    pub fn helper(chars: &Vec<char>, mut l: usize, mut r: usize) -> i32 {
        let mut count = 0;
        while chars[l] == chars[r] {
            count += 1;
            if l == 0 || r == chars.len() - 1 {
                break;
            }
            l -= 1;
            r += 1;
        }
        count
    }
}

pub fn run() {
    let input = [
        "abc",
        "aaa",
        "aaffifjaa",
        "aba",
        "aabbcca",
        "aaeeddeeaa",
        "aaggggaa",
        "abccba",
    ];

    for s in input {
        println!("{}", Solution::count_substrings(s.to_string()));
    }
}

/* dp
struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut map = vec![vec![false; s.len()]; chars.len()];
        let mut count = 0;
        for r in 0..s.len() {
            for l in 0..=r {
                if chars[l] == chars[r] {
                    if l == r || l + 1 == r || map[l + 1][r - 1] {
                        count += 1;
                        map[l][r] = true;
                    }
                }
            }
        }
        count
    }
}

 */
