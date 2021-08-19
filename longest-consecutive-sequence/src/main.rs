use std::cmp;

struct Solution {}
impl Solution {
    // 4ms 3MB
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        nums.sort();
        let mut max = 0;
        let mut long = 1;
        let mut temp = nums[0];
        for n in nums.into_iter().skip(1) {
            if n == temp {
                continue;
            } else if n - temp == 1 {
                long += 1;
            } else {
                max = cmp::max(max, long);
                long = 1;
            }
            temp = n;
        }
        cmp::max(max, long)
    }
}

fn main() {
    let input = vec![
        vec![100, 4, 200, 1, 3, 2],
        vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
        vec![],
    ];
    for nums in input {
        println!("{}", Solution::longest_consecutive(nums));
    }
}
