struct Solution {}
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mut m = (l + r) / 2;
            if m % 2 == 1 {
                m -= 1;
            }
            if nums[m] == nums[m + 1] {
                l = m + 2;
            } else {
                r = m;
            }
        }
        nums[l]
    }
}

pub fn run() {
    let input = vec![
        vec![1, 1, 2, 3, 3, 4, 4, 8, 8],
        vec![3, 3, 7, 7, 10, 11, 11],
        vec![1],
        vec![1, 1, 2],
        vec![1, 2, 2, 3, 3],
        vec![0, 1, 1],
    ];
    for nums in input {
        println!("{}", Solution::single_non_duplicate(nums));
    }
}
