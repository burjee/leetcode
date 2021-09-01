use std::cmp;
use std::mem;

struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut max = nums[0];
        let mut min = nums[0];
        for n in nums.into_iter().skip(1) {
            if n < 0 {
                mem::swap(&mut max, &mut min);
            }
            max = cmp::max(max * n, n);
            min = cmp::min(min * n, n);
            result = cmp::max(result, max);
        }
        result
    }
}

fn main() {
    let input = vec![
        vec![2, 3, -2, 4],
        vec![-2, 0, -1],
        vec![-2, 2, -1],
        vec![-2, -1, 0],
        vec![-3, 2, 1, 3, 2, -5],
        vec![-3, -2, -1, 0, 1, 2, 3, 5, 6],
        vec![-9, -5, -1],
        vec![-1, -1, -1],
        vec![-1, 0, -1, 0, -1, 0, -1],
        vec![-1],
        vec![2, -5, -2, -4, 3],
        vec![2, -5, -2, -4, -7, -2, 6],
        vec![-1, -3, -2, -10, -3],
        vec![0, -3, 1, 1],
    ];
    for nums in input {
        println!("{}", Solution::max_product(nums));
    }
}
