struct Solution {}
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut map = HashMap::new();
        for n in arr {
            *map.entry(n).or_insert(0) += 1;
        }

        let set: HashSet<_> = map.values().collect();
        set.iter().count() == map.iter().count()
    }
}

pub fn run() {
    let input = [
        vec![1, 2, 2, 1, 1, 3],
        vec![1, 2],
        vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0],
        vec![1, 1, 1, 2, 2, 3, 3, 3],
    ];

    for arr in input {
        println!("{}", Solution::unique_occurrences(arr));
    }
}
