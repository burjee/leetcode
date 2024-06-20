// 每個值先算出左邊的乘積，再乘上右邊的乘積
// 0是還沒有乘 階段代表擁有哪些欄位的乘積
// 1----2----3----4
// 0    0    0    0
// 0    1    0    0
// 0    1    12   0
// 0    1    12   123

// 1----2----3----4    right = 0
// 0    1    12   123, right = 4
// 0    1    124  123, right = 34
// 0    134  124  123, right = 234
// 234  134  124  123, right = 1234

struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut output = Vec::with_capacity(len);
        let mut left = 1;
        for &n in nums.iter() {
            output.push(left);
            left *= n;
        }
        let mut right = 1;
        for (i, n) in nums.into_iter().enumerate().rev() {
            output[i] *= right;
            right *= n;
        }
        output
    }
}

pub fn run() {
    let input = [
        vec![1, 2, 3, 4],
        vec![-1, 1, 0, -3, 3],
        vec![2, -5, 7, 2, 2, 1, 3, -6, -2, 2],
    ];

    for nums in input {
        println!("{:?}", Solution::product_except_self(nums));
    }
}
