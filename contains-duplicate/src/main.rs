use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                return true;
            }
            set.insert(n);
        }
        false
    }
}

fn main() {
    let input = vec![
        vec![1, 2, 3, 1],
        vec![1, 2, 3, 4],
        vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
    ];
    for nums in input {
        println!("{}", Solution::contains_duplicate(nums));
    }
}
