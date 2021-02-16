// waiting for optimization
use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        let mut map_vec = Vec::new();
        for n in nums {
            let count = map.entry(n).or_insert(0);
            *count += 1;
            if *count == 1 {
                map_vec.push(n);
            }
        }
        for i in 0..map_vec.len() {
            let v1 = map_vec[i];
            let skip = if *map.get(&map_vec[i]).unwrap() < 2 {
                1
            } else {
                0
            };
            for j in i + skip..map_vec.len() {
                let v2 = map_vec[j];
                let v3 = 0 - v1 - v2;
                if v2 > v3 || set.contains(&v3) {
                    continue;
                }
                if let Some(&c) = map.get(&v3) {
                    let mut count = 0;
                    if v1 == v3 {
                        count += 1;
                    }
                    if v2 == v3 {
                        count += 1;
                    }
                    if c > count {
                        ans.push(vec![v1, v2, v3]);
                    }
                }
            }
            set.insert(map_vec[i]);
        }
        ans
    }
}

fn main() {
    let numbers = vec![
        vec![-1, 0, 1, 2, -1, -4],
        vec![-1, 0, 1, 2, -1, -4, 0, 0, 2, 8],
        vec![],
        vec![0],
    ];
    for num in numbers {
        println!("ans: {:?}", Solution::three_sum(num));
    }
}
