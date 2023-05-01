struct Solution {}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut count = vec![1; n + 1];
        for i in 2..=n {
            let mut sum = 0;
            for j in 0..i {
                sum += count[j] * count[i - j - 1];
            }
            count[i] = sum;
        }
        count[n]
    }
}

pub fn run() {
    for n in 1..=19 {
        println!("{}: {}", n, Solution::num_trees(n));
    }
}

/* recursive
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let nums: Vec<i32> = (1..=n).collect();
        let mut map = HashMap::new();
        Solution::helper(&nums[..], &mut map)
    }

    pub fn helper(nums: &[i32], map: &mut HashMap<Vec<i32>, i32>) -> i32 {
        if nums.len() < 2 {
            return 1;
        }
        if nums.len() == 2 {
            return 2;
        }
        if map.contains_key(nums) {
            return *map.get(nums).unwrap();
        }
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += Solution::helper(&nums[..i], map) * Solution::helper(&nums[i + 1..], map)
        }
        map.insert(nums.to_owned(), sum);
        sum
    }
}

pub fn run() {
    for n in 1..=19 {
        println!("{}: {}", n, Solution::num_trees(n));
    }
}
 */
