struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut output = nums.len() as i32;
        for i in 0..nums.len() {
            output += i as i32 - nums[i];
        }
        output
    }
}

fn main() {
    let input = vec![
        vec![3, 0, 1],
        vec![0, 1],
        vec![9, 6, 4, 2, 3, 5, 7, 0, 1],
        vec![0, 1, 2, 3, 5],
        vec![0],
        vec![1],
    ];
    for nums in input {
        println!("{}", Solution::missing_number(nums));
    }
}
