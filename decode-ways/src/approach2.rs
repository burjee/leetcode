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

fn main() {
    let input = vec![
        "1110119111".to_string(),
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
