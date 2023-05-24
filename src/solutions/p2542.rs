use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut nums: Vec<(i32, i32)> = nums1.into_iter().zip(nums2.into_iter()).collect();
        nums.sort_by_key(|(_, n2)| Reverse(*n2));

        let mut sum = 0;
        let mut heap = BinaryHeap::new();
        for i in 0..k {
            sum += nums[i].0 as i64;
            heap.push(Reverse(nums[i].0 as i64));
        }

        let mut ans = sum * nums[k - 1].1 as i64;
        for i in k..nums.len() {
            sum = sum + nums[i].0 as i64 - heap.pop().unwrap().0;
            heap.push(Reverse(nums[i].0 as i64));
            ans = ans.max(sum * nums[i].1 as i64);
        }
        ans
    }
}

pub fn run() {
    let inputs = [
        (vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3),
        (vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1),
    ];
    for (nums1, nums2, k) in inputs {
        println!("{}", Solution::max_score(nums1, nums2, k));
    }
}
