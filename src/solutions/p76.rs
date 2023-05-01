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
    let input = vec![
        to_string("cgklivwehljxrdzpfdqsapogwvjtvbzahjnsejwnuhmomlfsrvmrnczjzjevkdvroiluthhpqtffhlzyglrvorgnalk", "mqfff"),
        to_string("bba", "ab"),
        to_string("ab", "b"),
        to_string("BDOAECODEBANC", "ABC"),
        to_string("a", "a"),
        to_string("ADOBECODENC", "ABC"),
        to_string("ADOBECODENC", "DDD"),
        to_string("ASSSSASSSSSBCSASSSS", "ABC"),
    ];

    for s in input {
        println!("{}", Solution::min_window(s.0, s.1));
    }
}

fn to_string(s1: &str, s2: &str) -> (String, String) {
    (String::from(s1), String::from(s2))
}
