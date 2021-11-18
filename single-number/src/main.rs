struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut output = 0;
        for n in &nums {
            output ^= n;
        }
        output
    }
}

fn main() {
    let input = vec![
        vec![2, 2, 1],
        vec![4, 1, 2, 1, 2],
        vec![1],
        vec![4, 1, 2, 1, 2, 4, 3, 3, 6, 7, 8, 9, 7, 9, 8],
    ];
    for nums in input {
        println!("{}", Solution::single_number(nums));
    }
}
