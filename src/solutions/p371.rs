struct Solution {}
impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while a != 0 {
            let and = (a & b) << 1;
            b = a ^ b;
            a = and;
        }
        b
    }
}

pub fn run() {
    let input = [
        (1, 2),
        (2, 3),
        (5, 7),
        (20, 36),
        (12, 0),
        (5, 7),
        (6, -5),
        (-8, 3),
        (6, -12),
        (-77, -55),
    ];

    for (a, b) in input {
        println!("{}", Solution::get_sum(a, b));
    }
}
