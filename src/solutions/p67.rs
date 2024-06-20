struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut output = String::with_capacity(a.len());
        let mut a = a.chars().rev();
        let mut b = b.chars().rev();
        let mut hold = 0;
        loop {
            let mut n = hold;
            match (a.next(), b.next()) {
                (Some(c0), Some(c1)) => {
                    if c0 == '1' {
                        n += 1;
                    }
                    if c1 == '1' {
                        n += 1;
                    }
                }
                (None, Some(c)) | (Some(c), None) => {
                    if c == '1' {
                        n += 1;
                    }
                }
                (None, None) => {
                    if hold == 1 {
                        output.push('1');
                    }
                    break;
                }
            }
            if n % 2 == 0 {
                output.push('0');
            } else {
                output.push('1');
            }
            if n > 1 {
                hold = 1;
            } else {
                hold = 0;
            }
        }
        output.chars().rev().collect()
    }
}

pub fn run() {
    let input = [
        ("11", "1"),
        ("1010", "1011"),
        ("110110", "1011011"),
        ("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101",
        "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011")
    ];

    for (a, b) in input {
        println!("{}", Solution::add_binary(a.to_string(), b.to_string()));
    }
}

/* XD
struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!(
            "{:b}",
            u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
        )
    }
}

pub fn run() {
    let input = [
        ("11", "1"),
        ("1010", "1011"),
        ("110110", "1011011"),
        ("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101",
        "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011")
    ];

    for (a, b) in input {
        println!("{}", Solution::add_binary(a.to_string(), b.to_string()));
    }
}
 */
