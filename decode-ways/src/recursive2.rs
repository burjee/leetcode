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

fn main() {
    let input = vec![
        "10011".to_string(),
        "120211022".to_string(),
        "1312012020320320211022".to_string(),
        "10".to_string(),
        "190".to_string(),
        "11".to_string(),
        "111".to_string(),
        "1111".to_string(),
        "11111".to_string(),
        "111111".to_string(),
        "1111111".to_string(),
        "11106".to_string(),
        "12".to_string(),
        "226".to_string(),
        "0".to_string(),
        "06".to_string(),
        "121122315413146123343463".to_string(),
        "111111111111111111111111111111111111111111111".to_string(),
    ];

    for s in input {
        println!("{}", Solution::num_decodings(s));
    }
}
