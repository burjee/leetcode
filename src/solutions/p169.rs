struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter = (0, 0);
        for n in nums {
            if counter.1 == 0 {
                counter.0 = n;
            }
            if n == counter.0 {
                counter.1 += 1;
            } else {
                counter.1 -= 1;
            }
        }
        counter.0
    }
}

pub fn run() {
    let input = [
        vec![3, 2, 3],
        vec![2, 2, 1, 1, 1, 2, 2],
        vec![2, 2, 2, 2, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 2, 2],
    ];

    for nums in input {
        println!("{}", Solution::majority_element(nums));
    }
}
