struct Solution {}
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut bits = x ^ y;
        let mut count = 0;
        while bits > 0 {
            count += bits & 1;
            bits >>= 1;
        }
        count
    }
}

pub fn run() {
    let input = [(1, 4), (3, 1)];
    for (x, y) in input {
        println!("{}", Solution::hamming_distance(x, y));
    }
}
