struct Solution {}
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0f64;
        for i in 0..k as usize {
            sum += nums[i] as f64;
        }

        let mut ans = sum;
        for i in k as usize..nums.len() {
            sum = sum + nums[i] as f64 - nums[i - k as usize] as f64;
            ans = ans.max(sum);
        }

        ans / k as f64
    }
}

pub fn run() {
    let input = [(vec![1, 12, -5, -6, 50, 3], 4), (vec![5], 1)];

    for (nums, k) in input {
        println!("{}", Solution::find_max_average(nums, k));
    }
}
