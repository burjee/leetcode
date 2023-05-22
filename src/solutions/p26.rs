struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut l = 0;
        for r in 1..nums.len() {
            if nums[l] == nums[r] {
                continue;
            }
            l += 1;
            nums[l] = nums[r];
        }
        l as i32 + 1
    }
}

pub fn run() {
    let input = [
        vec![1, 1, 2],
        vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
        vec![0, 0, 0, 0, 0],
    ];
    for mut nums in input {
        println!("{}", Solution::remove_duplicates(&mut nums));
    }
}
