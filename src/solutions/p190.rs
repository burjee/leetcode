struct Solution {}
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut output = 0;
        for _ in 0..32 {
            output <<= 1;
            output |= x & 1;
            x >>= 1;
        }
        output
    }
}

pub fn run() {
    let input = [
        0b00000010100101000001111010011100,
        0b11111111111111111111111111111101,
    ];

    for x in input {
        println!("{}", Solution::reverse_bits(x));
    }
}
