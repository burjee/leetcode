struct Solution {}
impl Solution {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut number = 0;
        while n > 0 {
            if n & 1 == 1 {
                number += 1;
            }
            n >>= 1;
        }
        number
    }
}

pub fn run() {
    let input = vec![
        0b11111111111111111111111111111101,
        0b00000000000000000000000000001011,
        0b00000000000000000000000010000000,
    ];
    for n in input {
        println!("{}", Solution::hamming_weight(n));
    }
}
