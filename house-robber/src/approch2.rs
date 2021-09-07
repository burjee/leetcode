use std::cmp;

struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return nums[0];
        }

        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = cmp::max(nums[0], nums[1]);
        for i in 2..len {
            dp[i] = cmp::max(nums[i] + dp[i - 2], dp[i - 1]);
        }
        dp[len - 1]
    }
}

fn main() {
    let input = vec![
        vec![1, 2, 3, 1],
        vec![2, 7, 9, 3, 1],
        vec![7, 1, 1, 7],
        vec![7, 1, 1, 7, 1, 1, 7],
        vec![0, 1, 2],
        vec![0, 1],
        vec![0],
    ];
    for nums in input {
        println!("{}", Solution::rob(nums));
    }
}
