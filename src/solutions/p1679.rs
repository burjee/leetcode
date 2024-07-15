struct Solution {}
impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut ans = 0;

        while l < r {
            match k.cmp(&(nums[l] + nums[r])) {
                std::cmp::Ordering::Equal => {
                    l += 1;
                    r -= 1;
                    ans += 1;
                }
                std::cmp::Ordering::Greater => l += 1,
                std::cmp::Ordering::Less => r -= 1,
            }
        }

        ans
    }

    // pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    //     use std::collections::HashMap;

    //     let mut map = HashMap::new();
    //     let mut ans = 0;

    //     for n in nums {
    //         if n >= k {
    //             continue;
    //         }

    //         if let Some(count) = map.get_mut(&n) {
    //             if *count > 0 {
    //                 *count -= 1;
    //                 ans += 1;
    //             } else {
    //                 *map.entry(k - n).or_insert(0) += 1;
    //             }
    //         } else {
    //             *map.entry(k - n).or_insert(0) += 1;
    //         }
    //     }

    //     ans
    // }
}

pub fn run() {
    let input = [
        (vec![1, 2, 3, 4], 5),
        (vec![3, 1, 3, 4, 3], 6),
        (
            vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2],
            3,
        ),
    ];

    for (nums, k) in input {
        println!("{}", Solution::max_operations(nums, k));
    }
}
