struct Solution {}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let m = nums.len() - k;
        let temp = nums[m..].to_owned();
        for i in (k..nums.len()).rev() {
            nums[i] = nums[i - k];
        }
        for i in 0..k {
            nums[i] = temp[i];
        }
    }
}

pub fn run() {
    let input = [
        (vec![1, 2, 3, 4, 5, 6, 7], 3),
        (vec![-1, -100, 3, 99], 2),
        (vec![-1], 2),
    ];

    for (mut nums, k) in input {
        Solution::rotate(&mut nums, k);
        println!("{:?}", nums);
    }
}
