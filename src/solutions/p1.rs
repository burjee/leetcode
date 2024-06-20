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

pub fn run() {
    let input = [
        (vec![2, 7, 11, 15], 9),
        (vec![2, 7, 11, 15], 26),
        (vec![2, 7, 11, 15], 13),
    ];

    for (num, target) in input {
        println!("{:?}", Solution::two_sum(num, target));
    }
}
