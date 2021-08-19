use std::cmp;
use std::collections::HashMap;

struct Solution {}
impl Solution {
    // 12ms 4.7MB
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut max = 0;
        for n in nums {
            if !map.contains_key(&n) {
                let mut l = 0;
                let mut r = 0;
                if let Some(&v) = map.get(&(n - 1)) {
                    l = v;
                }
                if let Some(&v) = map.get(&(n + 1)) {
                    r = v;
                }
                let long = l + 1 + r;
                map.insert(n, long);
                map.insert(n - l, long);
                map.insert(n + r, long);
                max = cmp::max(max, long);
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
