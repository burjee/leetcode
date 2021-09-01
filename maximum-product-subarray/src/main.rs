use std::cmp;

struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut product = 0;
        let mut hold = 1;
        for n in nums {
            if n == 0 {
                max = cmp::max(max, 0);
                max = cmp::max(max, product);
                if product != hold {
                    max = cmp::max(max, product / hold);
                }
                product = 0;
                hold = 1;
                continue;
            }
            if product == 0 {
                product = n;
            } else {
                product *= n;
            }
            max = cmp::max(max, product);
            max = cmp::max(max, product / hold);
            if n < 0 && hold == 1 {
                hold = product;
            }
        }
        max
    }
}

fn main() {
    let input = vec![
        vec![2, 3, -2, 4],
        vec![-2, 0, -1],
        vec![-2, 2, -1],
        vec![-2, -1, 0],
        vec![-3, 2, 1, 3, 2, -5],
        vec![-3, -2, -1, 0, 1, 2, 3, 5, 6],
        vec![-9, -5, -1],
        vec![-1, -1, -1],
        vec![-1, 0, -1, 0, -1, 0, -1],
        vec![-1],
        vec![2, -5, -2, -4, 3],
        vec![2, -5, -2, -4, -7, -2, 6],
        vec![-1, -3, -2, -10, -3],
        vec![0, -3, 1, 1],
    ];
    for nums in input {
        println!("{}", Solution::max_product(nums));
    }
}
