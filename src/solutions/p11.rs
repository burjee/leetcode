use std::cmp;
struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        while l < r {
            ans = cmp::max(ans, (r - l) * cmp::min(height[l], height[r]) as usize);
            if height[l] <= height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        ans as i32
    }
}

pub fn run() {
    let input = [
        vec![1, 8, 6, 2, 5, 4, 8, 3, 7],
        vec![1, 1],
        vec![4, 3, 2, 1, 4],
        vec![1, 2, 1],
        vec![5, 5, 5, 5, 5, 5, 5],
        vec![5, 5, 5, 5, 100, 100, 5, 5, 5],
    ];

    for height in input {
        println!("{}", Solution::max_area(height));
    }
}
