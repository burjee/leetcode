struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut sum = 0;
        for i in 0..nums.len() {
            let temp = nums[i] + sum;
            sum = if nums[i] > temp { nums[i] } else { temp };
            if sum > ans {
                ans = sum;
            }
        }
        ans
    }
}

pub fn run() {
    let input = vec![
        vec![1, 2],
        vec![-2, -1],
        vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
        vec![1],
        vec![0],
        vec![-1],
        vec![-100000],
    ];

    for nums in input {
        println!("ans: {}", Solution::max_sub_array(nums));
    }
}
