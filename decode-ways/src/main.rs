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
