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

        let mut letters = vec![];
        for d in digits.chars() {
            let n = d.to_digit(10).unwrap() as usize;
            letters.push(&LETTER_MAP[n - 2]);
        }
        Solution::helper(letters)
    }

    pub fn helper(mut letters: Vec<&[char; 4]>) -> Vec<String> {
        if let Some(letter) = letters.pop() {
            let prefix = Solution::helper(letters);
            let mut s = vec![];
            for p in prefix {
                for &ch in letter.iter() {
                    if ch != '0' {
                        let mut _p = p.clone();
                        _p.push(ch);
                        s.push(_p);
                    }
                }
            }
            s
        } else {
            vec!["".to_string()]
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
