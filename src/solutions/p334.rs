struct Solution {}
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min = i32::MAX;
        let mut max = i32::MAX;
        for n in nums {
            if n > max {
                return true;
            } else if min > n {
                min = n;
            } else if min < n && n < max {
                max = n;
            }
        }
        false
    }
}

pub fn run() {
    let input = [
        vec![1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1],
        vec![2, 1, 5, 0, 4, 6],
        vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
    ];

    for nums in input {
        println!("{}", Solution::increasing_triplet(nums));
    }
}
