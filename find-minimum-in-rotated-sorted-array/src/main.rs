use std::cmp;

struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut len = nums.len();
        let mut left = 0;
        while len > 0 {
            let mid = left + len / 2;
            if nums[mid] > min {
                left = mid;
            } else {
                min = nums[mid];
            }
            len -= cmp::max(1, len / 2);
        }
        min
    }
}

fn main() {
    let input = vec![
        vec![2, 1],
        vec![2, 3, 4, 1],
        vec![2, 3, 4, 5, 1],
        vec![3, 4, 5, 1, 2],
        vec![4, 5, 6, 7, 0, 1, 2],
        vec![11, 13, 15, 17],
        vec![4, 5, 6, 7, 0, 1, 2],
        vec![7, 8, 9, 1, 2, 3, 4, 5, 6],
        vec![9, 10, 11, 12, 13, 14, 15, 1, 2, 3, 4, 5, 6, 7, 8],
        vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![2, 3, 4, 5, 6, 1],
    ];
    for nums in input {
        println!("{}", Solution::find_min(nums));
    }
}
