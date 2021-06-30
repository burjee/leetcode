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

fn main() {
    let input = vec![
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
        // "111111111111111111111111111111111111111111111".to_string(),
    ];

    for s in input {
        println!("{}", Solution::num_decodings(s));
    }
}
