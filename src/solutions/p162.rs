struct Solution {}
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            let m = (l + r) / 2;
            if m > 0 && nums[m] < nums[m - 1] {
                r = m - 1;
            } else if m < nums.len() - 1 && nums[m] < nums[m + 1] {
                l = m + 1;
            } else {
                return m as i32;
            }
        }
        unreachable!()
    }
}

pub fn run() {
    let input = [vec![1, 2, 3, 1], vec![1, 2, 1, 3, 5, 6, 4]];

    for nums in input {
        println!("{}", Solution::find_peak_element(nums));
    }
}
