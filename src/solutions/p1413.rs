struct Solution {}
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min = 1;
        let mut sum = 1;
        for n in nums {
            sum += n;
            if sum < 1 {
                min -= sum - 1;
                sum = 1;
            }
        }
        min
    }
}

pub fn run() {
    let input = vec![vec![-3, 2, -3, 4, 2], vec![1, 2], vec![1, -2, -3]];
    for nums in input {
        println!("{}", Solution::min_start_value(nums));
    }
}
