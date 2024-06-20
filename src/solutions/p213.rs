use std::cmp;

struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return cmp::max(nums[0], nums[1]);
        }
        let (max0, max1) = (
            Solution::rob_by_house(&nums[..nums.len() - 1]),
            Solution::rob_by_house(&nums[1..]),
        );
        cmp::max(max0, max1)
    }

    pub fn rob_by_house(nums: &[i32]) -> i32 {
        let mut income = vec![0; nums.len()];
        income[0] = nums[0];
        income[1] = cmp::max(nums[0], nums[1]);
        for i in 2..nums.len() {
            income[i] = cmp::max(income[i - 1], nums[i] + income[i - 2]);
        }
        income[nums.len() - 1]
    }
}

pub fn run() {
    let input = [
        vec![2, 3, 2],
        vec![1, 2, 3, 1],
        vec![1, 2, 3],
        vec![1, 2, 3, 1, 1, 2, 1, 2],
        vec![1, 2, 3, 1, 2],
        vec![1, 2, 3, 1, 2, 1, 1],
        vec![0],
        vec![1, 3, 1, 3, 100],
        vec![0, 0],
    ];

    for nums in input {
        println!("{}", Solution::rob(nums));
    }
}
