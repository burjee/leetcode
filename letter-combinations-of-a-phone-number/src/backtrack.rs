const LETTER_MAP: [[char; 4]; 8] = [
    ['a', 'b', 'c', '0'],
    ['d', 'e', 'f', '0'],
    ['g', 'h', 'i', '0'],
    ['j', 'k', 'l', '0'],
    ['m', 'n', 'o', '0'],
    ['p', 'q', 'r', 's'],
    ['t', 'u', 'v', '0'],
    ['w', 'x', 'y', 'z'],
];

struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut ids = vec![];
        for d in digits.chars() {
            let n = d.to_digit(10).unwrap() as usize;
            ids.push(n - 2);
        }

        let mut output = vec![];
        Solution::helper(&mut output, &mut "".to_string(), &ids);
        output
    }

    pub fn helper(output: &mut Vec<String>, s: &mut String, ids: &[usize]) {
        if ids.is_empty() {
            output.push(s.clone());
            return;
        }

        let letters = &LETTER_MAP[ids[0]];
        for &ch in letters {
            if ch != '0' {
                s.push(ch);
                Solution::helper(output, s, &ids[1..]);
                s.pop();
            }
        }
    }
}

fn main() {
    let input = vec![
        "23".to_string(),
        "".to_string(),
        "56".to_string(),
        "49".to_string(),
        "2356".to_string(),
    ];
    for digits in input {
        println!("{:?}", Solution::letter_combinations(digits));
    }
}
