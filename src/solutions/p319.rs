struct Solution {}
impl Solution {
    // math
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }

    // Time Limit Exceeded
    pub fn bulb_switch1(n: i32) -> i32 {
        let n = n as usize;
        let mut b = vec![true; n];
        for i in 1..n {
            for j in (i..n).step_by(i + 1) {
                b[j] = !b[j];
            }
        }
        b.into_iter().filter(|&n| n).count() as i32
    }
}

pub fn run() {
    for n in 0..=100 {
        println!("{}", Solution::bulb_switch(n));
    }
}
