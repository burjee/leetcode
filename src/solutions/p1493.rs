struct Solution {}
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut k = 1;
        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            if nums[r] == 0 {
                k -= 1;
            }

            if k < 0 {
                if nums[l] == 0 {
                    k += 1;
                }
                l += 1;
            }

            r += 1;
        }

        (r - l - 1) as i32
    }
}

pub fn run() {
    let input = [
        vec![1, 1, 0, 1],
        vec![0, 1, 1, 1, 0, 1, 1, 0, 1],
        vec![1, 1, 1],
    ];

    for nums in input {
        println!("{}", Solution::longest_subarray(nums));
    }
}
