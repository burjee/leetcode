struct Solution {}
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut len = t.len();
        let mut counts = [0i32; 58];
        let mut matched = [false; 58];
        for &c in t {
            let cu = c as usize - 65;
            counts[cu] += 1;
            matched[cu] = true;
        }

        let mut range = (0, 0);
        let mut start = 0;
        let mut i = 0;
        while i < s.len() {
            let cu = s[i] as usize - 65;
            if matched[cu] {
                counts[cu] -= 1;
                if counts[cu] >= 0 {
                    len -= 1;
                }
                if len == 0 {
                    while start <= i {
                        let cs = s[start] as usize - 65;
                        if matched[cs] {
                            if counts[cs] < 0 {
                                counts[cs] += 1;
                            } else {
                                if range == (0, 0) || i + 1 - start < range.1 - range.0 {
                                    range.0 = start;
                                    range.1 = i + 1;
                                }
                                break;
                            }
                        }
                        start += 1;
                    }
                }
            }
            i += 1;
        }
        String::from_utf8((&s[range.0..range.1]).to_vec()).unwrap()
    }
}

pub fn run() {
    let input = [
        ("cgklivwehljxrdzpfdqsapogwvjtvbzahjnsejwnuhmomlfsrvmrnczjzjevkdvroiluthhpqtffhlzyglrvorgnalk", "mqfff"),
        ("bba", "ab"),
        ("ab", "b"),
        ("BDOAECODEBANC", "ABC"),
        ("a", "a"),
        ("ADOBECODENC", "ABC"),
        ("ADOBECODENC", "DDD"),
        ("ASSSSASSSSSBCSASSSS", "ABC"),
    ];

    for (s, t) in input {
        println!("{}", Solution::min_window(s.to_string(), t.to_string()));
    }
}
