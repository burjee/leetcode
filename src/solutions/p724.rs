struct Solution {}
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut l_sum: i32 = 0;
        let mut r_sum: i32 = nums.iter().sum();
        let mut pivot = 0;
        while pivot < nums.len() {
            r_sum -= nums[pivot];
            if l_sum == r_sum {
                return pivot as i32;
            }
            l_sum += nums[pivot];
            pivot += 1;
        }
        -1
    }
}

pub fn run() {
    let input = [vec![1, 7, 3, 6, 5, 6], vec![1, 2, 3], vec![2, 1, -1]];

    for nums in input {
        println!("{}", Solution::pivot_index(nums));
    }
}
