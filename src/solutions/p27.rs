struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                nums.swap_remove(i);
            } else {
                i += 1;
            }
        }
        nums.len() as i32
    }
}

pub fn run() {
    let input = [(vec![3, 2, 2, 3], 3), (vec![0, 1, 2, 2, 3, 0, 4, 2], 2)];
    for (mut nums, val) in input {
        println!("{:?}", Solution::remove_element(&mut nums, val));
    }
}
