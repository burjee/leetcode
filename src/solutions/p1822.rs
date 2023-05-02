struct Solution {}
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(1, |a, b| a * b.signum())
    }

    pub fn _array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;
        for n in nums {
            if n == 0 {
                return 0;
            } else if n < 0 {
                sign *= -1;
            }
        }
        sign
    }
}

pub fn run() {
    let input = [
        vec![-1, -2, -3, -4, 3, 2, 1],
        vec![1, 5, 0, 2, -3],
        vec![-1, 1, -1, 1, -1],
    ];
    for nums in input {
        println!("{}", Solution::array_sign(nums));
    }
}
