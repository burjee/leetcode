struct Solution {}
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut ans = String::with_capacity(s.len());
        let mut chars1 = s.chars();
        let mut chars2 = s.chars().rev();

        while let Some(c1) = chars1.next() {
            if !Solution::is_vowels(c1) {
                ans.push(c1);
            } else {
                while let Some(c2) = chars2.next() {
                    if Solution::is_vowels(c2) {
                        ans.push(c2);
                        break;
                    }
                }
            }
        }

        ans
    }

    // pub fn reverse_vowels(s: String) -> String {
    //     let mut l = 0;
    //     let mut r = s.len() - 1;
    //     let mut chars: Vec<char> = s.chars().collect();

    //     while l < r {
    //         if !Solution::is_vowels(chars[l]) {
    //             l += 1;
    //             continue;
    //         }
    //         if !Solution::is_vowels(chars[r]) {
    //             r -= 1;
    //             continue;
    //         }

    //         chars.swap(l, r);
    //         l += 1;
    //         r -= 1;
    //     }

    //     chars.into_iter().collect()
    // }

    pub fn is_vowels(c: char) -> bool {
        c == 'a'
            || c == 'e'
            || c == 'i'
            || c == 'o'
            || c == 'u'
            || c == 'A'
            || c == 'E'
            || c == 'I'
            || c == 'O'
            || c == 'U'
    }
}

pub fn run() {
    let input = ["hello", "leetcode"];

    for s in input {
        println!("{}", Solution::reverse_vowels(s.to_string()));
    }
}
