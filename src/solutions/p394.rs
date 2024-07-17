struct Solution {}
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = Vec::new();
        let mut num = 0;
        let mut strs = String::new();

        for c in s.chars() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap();
            } else if c == '[' {
                stack.push((num as usize, strs));
                num = 0;
                strs = String::new();
            } else if c == ']' {
                let (n, mut pre_str) = stack.pop().unwrap();
                pre_str.push_str(&strs.repeat(n));
                strs = pre_str;
            } else {
                strs.push(c);
            }
        }

        strs
    }

    // pub fn decode_string(s: String) -> String {
    //     let chars: Vec<_> = s.chars().collect();
    //     let mut i = 0;
    //     Solution::helper1(&chars, &mut i)
    // }

    // pub fn helper1(chars: &Vec<char>, i: &mut usize) -> String {
    //     let mut s = String::new();
    //     while *i < chars.len() {
    //         if chars[*i] == ']' {
    //             *i += 1;
    //             return s;
    //         }
    //         if chars[*i].is_digit(10) {
    //             s.push_str(&Solution::helper2(&chars, i));
    //         } else {
    //             s.push_str(&Solution::helper3(&chars, i));
    //         }
    //     }
    //     s
    // }

    // pub fn helper2(chars: &Vec<char>, i: &mut usize) -> String {
    //     let mut n = String::new();
    //     while chars[*i].is_digit(10) {
    //         n.push(chars[*i]);
    //         *i += 1;
    //     }

    //     *i += 1;

    //     let mut s = String::new();
    //     while !chars[*i].is_digit(10) && chars[*i] != ']' {
    //         s.push(chars[*i]);
    //         *i += 1;
    //     }

    //     if chars[*i] == ']' {
    //         *i += 1;
    //     } else {
    //         s.push_str(&Solution::helper1(chars, i));
    //     }
    //     s.repeat(n.parse::<usize>().unwrap())
    // }

    // pub fn helper3(chars: &Vec<char>, i: &mut usize) -> String {
    //     let mut s = String::new();
    //     while *i < chars.len() && !chars[*i].is_digit(10) && chars[*i] != ']' {
    //         s.push(chars[*i]);
    //         *i += 1;
    //     }
    //     s
    // }
}

pub fn run() {
    let input = [
        "3[a]2[bc]",
        "3[a2[c]]",
        "2[abc]3[cd]ef",
        "3[z]2[2[y]pq4[2[jk]e1[f]]]ef",
        "sd2[f2[e]g]i",
    ];

    for s in input {
        println!("{}", Solution::decode_string(s.to_string()));
    }
}
