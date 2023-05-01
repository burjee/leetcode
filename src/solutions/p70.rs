struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 1;
        let mut c = b;
        for _ in 1..n {
            c = a + b;
            a = b;
            b = c;
        }
        c
    }
}

pub fn run() {
    let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for &n in &input {
        println!("ans: {}", Solution::climb_stairs(n));
    }
}
