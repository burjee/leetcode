struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!(
            "{:b}",
            u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
        )
    }
}

fn main() {
    let input = vec![
        ("11".to_string(), "1".to_string()),
        ("1010".to_string(), "1011".to_string()),
        ("110110".to_string(), "1011011".to_string()),
        ("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(),
        "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string())
    ];
    for (a, b) in input {
        println!("{}", Solution::add_binary(a, b));
    }
}
