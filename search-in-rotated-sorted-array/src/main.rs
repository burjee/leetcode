use std::cmp::max;

struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut size = nums.len();
        let mut left = 0;
        while size > 0 {
            let mid = left + size / 2;
            if target == nums[mid] {
                return mid as i32;
            }
            if !((nums[left] <= target && target <= nums[mid])
                || (nums[left] > nums[mid] && (nums[left] <= target || target <= nums[mid])))
            {
                left = mid;
            }
            size -= max(1, size / 2);
        }
        -1
    }
}

fn main() {
    let input = vec![
        (vec![1], 1),
        (vec![1, 3], 1),
        (vec![1, 3], 3),
        (vec![1, 2, 3], 3),
        (vec![1, 2, 3], 1),
        (vec![1, 3], 0),
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![4, 5, 6, 7, 0, 1, 2], 3),
        (vec![1], 0),
        (vec![7, 8, 9, 1, 2, 3, 4, 5, 6], 9),
        (vec![1, 2, 3, 4, 5], 5),
        (vec![9, 10, 11, 12, 13, 14, 15, 1, 2, 3, 4, 5, 6, 7, 8], 8),
        (vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9], 8),
    ];
    for val in input {
        println!("ans: {}", Solution::search(val.0, val.1));
    }
}
