struct Solution {}
impl Solution {
    // 10ms
    pub fn max_vowels(s: String, k: i32) -> i32 {
        fn is_vowel(c: char) -> bool {
            c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        }

        let k = k as usize;
        let c: Vec<char> = s.chars().collect();
        let mut ans;
        let mut count = 0;
        for i in 0..k {
            if is_vowel(c[i]) {
                count += 1;
            }
        }
        ans = count;
        for i in k..s.len() {
            if is_vowel(c[i]) {
                count += 1;
            }
            if is_vowel(c[i - k]) {
                count -= 1;
            }
            ans = ans.max(count);
        }
        ans
    }

    // 25ms
    pub fn _max_vowels(s: String, k: i32) -> i32 {
        use std::collections::HashSet;
        let k = k as usize;
        let set = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let c: Vec<char> = s.chars().collect();
        let mut ans;
        let mut count = 0;
        for i in 0..k {
            if set.contains(&c[i]) {
                count += 1;
            }
        }
        ans = count;
        for i in k..s.len() {
            if set.contains(&c[i]) {
                count += 1;
            }
            if set.contains(&c[i - k]) {
                count -= 1;
            }
            ans = ans.max(count);
        }
        ans
    }
}

pub fn run() {
    let input = [
        ("abciiidef", 3),
        ("aeiou", 2),
        ("leetcode", 3),
        ("weallloveyou", 7),
        ("ibpbhixfiouhdljnjfflpapptrxgcomvnb", 33),
        ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 999),
    ];

    for (s, k) in input {
        println!("{}", Solution::max_vowels(s.to_string(), k));
    }
}
