use std::cmp;
use std::collections::HashSet;

struct Solution {}
impl Solution {
    // 292ms 3.5MB
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut set = HashSet::new();
        for &n in nums.iter() {
            set.insert(n);
        }
        for n in nums {
            if !set.contains(&(n - 1)) {
                let mut long = 1;
                let mut i = n + 1;
                while set.contains(&i) {
                    long += 1;
                    i += 1;
                }
                max = cmp::max(max, long)
            }
        }
        max
    }
}

fn main() {
    let input = vec![
        vec![100, 4, 200, 1, 3, 2],
        vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
    ];
    for nums in input {
        println!("{}", Solution::longest_consecutive(nums));
    }
}
