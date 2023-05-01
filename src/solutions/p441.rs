struct Solution {}
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let mut l = 1;
        let mut r = n as i64;
        while l <= r {
            let m = (l + r) / 2;
            let v = m * (m + 1) / 2;
            if v == n {
                return m as i32;
            } else if v < n {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        r as i32
    }
}

pub fn run() {
    let input = vec![1, 5, 8, 2, 3, 11, 30, 56, 79, 1804289383];
    for n in input {
        println!("{}", Solution::arrange_coins(n));
    }
}
