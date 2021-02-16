use std::collections::HashMap;

struct Solution {}
impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut ans = vec![];
        for (i, num) in nums.iter().enumerate() {
            let half_num = target - num;
            if map.contains_key(&half_num) {
                ans.push(*map.get(&half_num).unwrap());
                ans.push(i as i32);
                break;
            }
            map.insert(num, i as i32);
        }
        ans
    }
}

fn main() {
    let nums = vec![vec![2, 7, 11, 15], vec![2, 7, 11, 15], vec![2, 7, 11, 15]];
    let target = vec![9, 26, 13];

    for (i, num) in nums.iter().enumerate() {
        let res = Solution::two_sum(num.to_vec(), target[i]);
        println!("{}, {}", res[0], res[1]);
    }

    for i in 0..3 {
        let res = Solution::two_sum(nums[i].to_vec(), target[i]);
        println!("{}, {}", res[0], res[1]);
    }

    // nums = vec![3, 2, 4];
    // target = 6;
    // res = Solution::two_sum(nums, target);
    // println!("{}, {}", res[0], res[1]);

    // nums = vec![3, 3];
    // target = 6;
    // res = Solution::two_sum(nums, target);
    // println!("{}, {}", res[0], res[1]);
}
