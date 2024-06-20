struct Solution {}
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut output = vec![0; n + 1];
        for i in 1..=n {
            output[i] = output[i >> 1] + (i as i32 & 1);
        }
        output
    }
}

pub fn run() {
    let input = [0, 2, 5, 7, 12, 20, 56];

    for n in input {
        println!("{:?}", Solution::count_bits(n));
    }
}
