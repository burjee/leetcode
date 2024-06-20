// 'A' -> "1"
// 'B' -> "2"
// ...
// 'Z' -> "26"
// 1110119111 -> 11 10 119 111
// = fib(2) * 1 * fib(3) * fib(3) = 2 * 1 * 3 * 3 = 18
// fib_n0 * fib_n1 ... * fib_n
struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        if chars[0] == '0' {
            return 0;
        }
        if chars.len() == 1 {
            return 1;
        }
        let mut fib = vec![1, 1, 2];
        let mut sum = 1;
        let mut c = 0;
        for i in 0..chars.len() - 1 {
            if chars[i + 1] == '0' {
                if chars[i] != '1' && chars[i] != '2' {
                    return 0;
                }
                sum *= fib[c];
                c = 0;
                continue;
            }
            if chars[i] == '0' {
                continue;
            }
            c += 1;
            Solution::progress_fib(&mut fib, c);
            if (chars[i] != '1' && chars[i] != '2')
                || (chars[i] == '2'
                    && (chars[i + 1] == '7' || chars[i + 1] == '8' || chars[i + 1] == '9'))
            {
                sum *= fib[c];
                c = 0;
            }
        }
        if c > 0 {
            c += 1;
            Solution::progress_fib(&mut fib, c);
            sum *= fib[c];
        }
        sum
    }

    pub fn progress_fib(fib: &mut Vec<i32>, progress: usize) {
        for i in fib.len()..=progress {
            fib.push(fib[i - 1] + fib[i - 2]);
        }
    }
}

pub fn run() {
    let input = [
        "1110119111",
        "10011",
        "120211022",
        "1312012020320320211022",
        "10",
        "190",
        "11",
        "111",
        "1111",
        "11111",
        "111111",
        "1111111",
        "11106",
        "12",
        "226",
        "0",
        "06",
        "121122315413146123343463",
        "111111111111111111111111111111111111111111111",
    ];

    for s in input {
        println!("{}", Solution::num_decodings(s.to_string()));
    }
}

/* approach2
// 1110119111
// [*, 1, 1, 1, 0, 1, 1, 9, 1,  1,  1]
// [1, 1, 2, 3, 2, 2, 4, 6, 6, 12, 18]
// if m = 0,           ways(n) = ways(n-2)             | 1110 -> 11 10 = n-2
// if m = 1-9,         ways(n) = ways(n-1)             | 1109 -> 110 9 = n-1
// if m = 11-19,21-26, ways(n) = ways(n-1) + ways(n-2) | 1111 -> 1111  = n-1 + n-2
// => if m != 0 { ways[n] += ways(n-1) }
// => if m >= 10 && double <= 26 { ways[n] += ways(n-2) }
struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut ways = vec![0; s.len() + 1];
        ways[0] = 1;
        ways[1] = 1;
        if &s[..1] == "0" {
            return 0;
        }
        if s.len() == 1 {
            return 1;
        }
        for i in 2..=s.len() {
            let single = s[i - 1..i].parse::<i32>().unwrap();
            let double = s[i - 2..i].parse::<i32>().unwrap();
            if single != 0 {
                ways[i] += ways[i - 1];
            }
            if double >= 10 && double <= 26 {
                ways[i] += ways[i - 2]
            }
        }
        *ways.last().unwrap()
    }
}

pub fn run() {
    let input = [
        "1110119111",
        "10011",
        "120211022",
        "1312012020320320211022",
        "10",
        "190",
        "11",
        "111",
        "1111",
        "11111",
        "111111",
        "1111111",
        "11106",
        "12",
        "226",
        "0",
        "06",
        "121122315413146123343463",
        "111111111111111111111111111111111111111111111",
    ];

    for s in input {
        println!("{}", Solution::num_decodings(s.to_string()));
    }
}
 */

/* recursive
// Time Limit Exceeded
struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        Solution::decode0(&chars, 0) + Solution::decode1(&chars, 0)
    }

    pub fn decode0(chars: &Vec<char>, p: usize) -> i32 {
        if chars.len() == p {
            return 1;
        }
        if chars[p] == '0' {
            return 0;
        }
        Solution::decode0(chars, p + 1) + Solution::decode1(&chars, p + 1)
    }

    pub fn decode1(chars: &Vec<char>, p: usize) -> i32 {
        if chars.len() - 1 <= p {
            return 0;
        }
        if chars[p] != '1' && chars[p] != '2' {
            return 0;
        }
        if chars[p] == '2' && (chars[p + 1] == '7' || chars[p + 1] == '8' || chars[p + 1] == '9') {
            return 0;
        }
        if chars.len() - 2 == p {
            return 1;
        }
        Solution::decode0(chars, p + 2) + Solution::decode1(&chars, p + 2)
    }
}

pub fn run() {
    let input = [
        "190",
        "11",
        "111",
        "1111",
        "11111",
        "111111",
        "1111111",
        "11106",
        "12",
        "226",
        "0",
        "06",
        // "111111111111111111111111111111111111111111111",
    ];

    for s in input {
        println!("{}", Solution::num_decodings(s.to_string()));
    }
}
 */

/* recursive2
// Use cache to solve problems
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        Solution::decode(&s, &mut HashMap::new())
    }

    pub fn decode(chars: &str, map: &mut HashMap<String, i32>) -> i32 {
        if let Some(&cache) = map.get(chars) {
            return cache;
        }
        if chars.len() == 0 {
            return 1;
        }
        if chars.len() == 1 {
            if Solution::decode_a(chars) {
                return 1;
            }
            return 0;
        }
        let mut sum = 0;
        if Solution::decode_a(&chars[..1]) {
            sum += Solution::decode(&chars[1..], map);
        }
        if Solution::decode_b(&chars[..2]) {
            sum += Solution::decode(&chars[2..], map);
        }
        map.insert(chars.to_string(), sum);
        sum
    }

    pub fn decode_a(chars: &str) -> bool {
        if &chars[..] == "0" {
            return false;
        }
        true
    }

    pub fn decode_b(chars: &str) -> bool {
        if &chars[..1] != "1" && &chars[..1] != "2" {
            return false;
        }
        if &chars[..1] == "2" && (&chars[1..2] == "7" || &chars[1..2] == "8" || &chars[1..2] == "9")
        {
            return false;
        }
        true
    }
}

pub fn run() {
    let input = [
        "10011",
        "120211022",
        "1312012020320320211022",
        "10",
        "190",
        "11",
        "111",
        "1111",
        "11111",
        "111111",
        "1111111",
        "11106",
        "12",
        "226",
        "0",
        "06",
        "121122315413146123343463",
        "111111111111111111111111111111111111111111111",
    ];

    for s in input {
        println!("{}", Solution::num_decodings(s.to_string()));
    }
}
 */
