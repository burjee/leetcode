use std::cmp;

struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return nums[0];
        } else if len == 2 {
            return cmp::max(nums[0], nums[1]);
        }

        let mut income = vec![0; nums.len()];
        income[0] = nums[0];
        income[1] = nums[1];
        for i in 0..len - 3 {
            income[i + 2] = cmp::max(income[i + 2], income[i] + nums[i + 2]);
            income[i + 3] = cmp::max(income[i + 3], income[i] + nums[i + 3]);
        }
        income[len - 1] = cmp::max(income[len - 1], income[len - 3] + nums[len - 1]);
        cmp::max(income[len - 1], income[len - 2])
    }
}

pub fn run() {
    let input = [
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

/* approch2
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

pub fn run() {
    let input = [
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
 */
