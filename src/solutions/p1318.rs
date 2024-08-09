struct Solution {}
impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut ans = 0;
        for _ in 0..32 {
            let a0 = a & 1;
            let b0 = b & 1;
            let c0 = c & 1;
            if a0 | b0 != c0 {
                if c0 == 1 {
                    ans += 1;
                } else {
                    ans += a0 + b0;
                }
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        ans
    }
}

pub fn run() {
    let input = [(2, 6, 5), (4, 2, 7), (1, 2, 3)];

    for (a, b, c) in input {
        println!("{}", Solution::min_flips(a, b, c));
    }
}
