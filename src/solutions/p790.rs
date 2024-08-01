struct Solution {}
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let modulo = 1000000007;
        let mut count = vec![1, 1, 2, 5, 11, 24];
        for i in 6..=n as usize {
            count.push((count[i - 1] * 2 % modulo + count[i - 3]) % modulo);
        }
        count[n as usize]
    }
}

pub fn run() {
    let input = [1, 2, 3, 4, 5, 6, 7, 800];

    for n in input {
        println!("{}", Solution::num_tilings(n));
    }
}
