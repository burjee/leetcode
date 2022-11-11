use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ans = VecDeque::new();
        let mut dis = VecDeque::new();
        for n in arr {
            let d = (n - x).abs();
            if ans.len() < k {
                ans.push_back(n);
                dis.push_back(d);
            } else {
                if n == *ans.front().unwrap() {
                    continue;
                } else if d < *dis.front().unwrap() {
                    ans.pop_front();
                    dis.pop_front();
                    ans.push_back(n);
                    dis.push_back(d);
                } else {
                    break;
                }
            }
        }
        ans.into()
    }
}

fn main() {
    let input = [
        (vec![1, 2, 3, 4, 5], 4, 3),
        (vec![1, 2, 3, 4, 5], 4, -1),
        (vec![1, 1, 1, 10, 10, 10], 1, 9),
    ];
    for (arr, k, x) in input {
        println!("{:?}", Solution::find_closest_elements(arr, k, x));
    }
}
