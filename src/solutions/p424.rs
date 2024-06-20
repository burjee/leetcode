struct Solution {}
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut map = vec![0; 26];
        let mut max = 1;
        let mut l = 0;
        let mut r = 0;
        let k = k as usize;
        let chars: Vec<u8> = s.bytes().collect();
        while r < chars.len() {
            let i = (chars[r] - b'A') as usize;
            map[i] += 1;
            r += 1;
            max = max.max(map[i]);
            if r - l - max > k {
                let i = (chars[l] - b'A') as usize;
                map[i] -= 1;
                l += 1;
            }
        }
        (r - l) as i32
    }
}

pub fn run() {
    let input = [
        ("ABAB", 2),
        ("AABABBA", 1),
        ("ABABABABABAB", 3),
        ("AAAABBCCCEEQQCC", 5),
        ("ABAJKGDSJJJJJJJJJJJJJJ", 5),
        ("JKGDSJJJJJJJJJJJJJJ", 5),
        ("ANBJANBJAABBABAJABABJNJBJNA", 5),
    ];

    for (s, k) in input {
        println!("{}", Solution::character_replacement(s.to_string(), k));
    }
}
