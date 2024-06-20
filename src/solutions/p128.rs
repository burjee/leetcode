use std::cmp;

struct Solution {}
impl Solution {
    // 4ms 3MB
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        nums.sort();
        let mut max = 0;
        let mut long = 1;
        let mut temp = nums[0];
        for n in nums.into_iter().skip(1) {
            if n == temp {
                continue;
            } else if n - temp == 1 {
                long += 1;
            } else {
                max = cmp::max(max, long);
                long = 1;
            }
            temp = n;
        }
        cmp::max(max, long)
    }
}

pub fn run() {
    let input = [
        vec![100, 4, 200, 1, 3, 2],
        vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
        vec![],
    ];

    for nums in input {
        println!("{}", Solution::longest_consecutive(nums));
    }
}

/* map
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

pub fn run() {
    let input = [
        vec![100, 4, 200, 1, 3, 2],
        vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
    ];

    for nums in input {
        println!("{}", Solution::longest_consecutive(nums));
    }
}
 */

/* set
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

pub fn run() {
    let input = [
        vec![100, 4, 200, 1, 3, 2],
        vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
    ];

    for nums in input {
        println!("{}", Solution::longest_consecutive(nums));
    }
}
 */
