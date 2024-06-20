struct Solution {}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut len = 0;
        let mut iter = nums.into_iter();
        while let Some(n) = iter.next_back() {
            if n >= len {
                len = 0;
            }
            len += 1;
        }
        len == 1
    }
}

pub fn run() {
    let input = [
        vec![2, 3, 1, 1, 4],
        vec![3, 2, 1, 0, 4],
        vec![3, 1, 0, 2, 3, 4],
        vec![0],
    ];

    for nums in input {
        println!("{}", Solution::can_jump(nums));
    }
}
