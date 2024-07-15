struct Solution {}
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            if nums[r] == 0 {
                k -= 1;
            }

            if k < 0 {
                if nums[l] == 0 {
                    k += 1;
                }
                l += 1;
            }

            r += 1;
        }

        (r - l) as i32
    }

    // pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    //     let mut l = 0;
    //     let mut r = 0;
    //     let mut used = 0;
    //     let mut ans = 0;

    //     while r < nums.len() {
    //         if nums[r] == 0 {
    //             if used < k {
    //                 used += 1;
    //             } else {
    //                 while nums[l] == 1 {
    //                     l += 1;
    //                 }
    //                 l += 1;
    //             }
    //         }

    //         r += 1;
    //         ans = ans.max((r - l) as i32);
    //     }

    //     ans
    // }
}

pub fn run() {
    let input = [
        (vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
        (
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3,
        ),
        (
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            0,
        ),
        (vec![0, 0, 0, 0, 0, 0], 0),
    ];

    for (nums, k) in input {
        println!("{}", Solution::longest_ones(nums, k));
    }
}
