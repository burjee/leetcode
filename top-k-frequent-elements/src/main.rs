use std::collections::BTreeMap;
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if nums.len() == k {
            return nums;
        }

        let mut map = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut btree = BTreeMap::new();
        for (num, count) in map {
            btree.entry(count).or_insert(vec![]).push(num);
        }

        let mut output = vec![];
        for (_, nums) in btree.into_iter().rev() {
            output.extend(nums);
            // It is guaranteed that the answer is unique.
            if output.len() == k {
                break;
            }
        }
        output
    }
}

fn main() {
    let input = vec![
        (vec![1, 1, 1, 2, 2, 3], 2),
        (vec![1], 1),
        (
            vec![
                3, 3, 3, 1, 5, 5, 5, 5, 5, 4, 4, 4, 4, 2, 2, 6, 6, 6, 6, 6, 6,
            ],
            3,
        ),
        (
            vec![
                3, 3, 3, 2, 2, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 1, 4, 4, 4, 4, 5, 5, 5, 5, 5,
            ],
            5,
        ),
        (
            vec![
                6, 6, 6, 6, 6, 6, 1, 3, 3, 3, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 2, 2, 4,
                4, 4, 4,
            ],
            4,
        ),
    ];
    for (nums, k) in input {
        println!("{:?}", Solution::top_k_frequent(nums, k));
    }
}
