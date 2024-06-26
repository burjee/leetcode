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

pub fn run() {
    let input = [
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

/* xor
// 0 ^ b = b
// a ^ b ^ b = a
// 初始為0，去掉重複的，剩下的就是答案

struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut output = 0;
        for i in 0..nums.len() {
            output ^= (i + 1) as i32 ^ nums[i];
        }
        output
    }
}

pub fn run() {
    let input = [
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
 */
