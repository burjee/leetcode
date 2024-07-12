struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }
        for j in i..nums.len() {
            nums[j] = 0;
        }
    }
}

pub fn run() {
    let input = [vec![0, 1, 0, 3, 12], vec![0]];

    for mut nums in input {
        Solution::move_zeroes(&mut nums);
        println!("{:?}", nums);
    }
}
