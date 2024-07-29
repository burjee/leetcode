struct Solution {}
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        for _ in 0..k as usize - 1 {
            heap.pop();
        }

        heap.pop().unwrap()
    }
}

pub fn run() {
    let input = [(vec![3, 2, 1, 5, 6, 4], 2), (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)];

    for (nums, k) in input {
        println!("{}", Solution::find_kth_largest(nums, k));
    }
}
