struct Solution {}
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut ans = 0;
        while r < nums.len() {
            let jump = nums[l] as usize;
            let mut farthest = l + jump;
            let mut next_l = l;
            for k in r..=farthest.min(nums.len() - 1) {
                let far = k + nums[k] as usize;
                if far > farthest {
                    farthest = far;
                    next_l = k;
                }
            }
            r = l + jump + 1;
            l = next_l;
            ans += 1;
        }
        ans
    }
}

pub fn run() {
    let input = [
        vec![2, 1],
        vec![2, 3, 1, 1, 4],
        vec![2, 3, 0, 1, 4],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 2, 1, 1],
    ];

    for nums in input {
        println!("{}", Solution::jump(nums));
    }
}
