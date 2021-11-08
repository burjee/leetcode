// 不符合 constant extra space
use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
            } else {
                set.insert(n);
            }
        }
        set.into_iter().collect()
    }
}

fn main() {
    let input = vec![
        vec![1, 2, 1, 3, 2, 5],
        vec![-1, 0],
        vec![0, 1],
        vec![0, 0, 1, 1, 2, 3, 3, 2, 4, 5, 6, 7, 8, 9, 8, 9, 7, 6],
    ];
    for nums in input {
        println!("{:?}", Solution::single_number(nums));
    }
}
