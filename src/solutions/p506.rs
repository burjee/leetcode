use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut rank = score.clone();
        rank.sort();

        let map: HashMap<_, _> = rank.into_iter().zip((1..=score.len()).rev()).collect();
        let mut ans = Vec::with_capacity(score.len());
        for s in score {
            let r = map.get(&s).unwrap();
            match r {
                1 => ans.push("Gold Medal".to_string()),
                2 => ans.push("Silver Medal".to_string()),
                3 => ans.push("Bronze Medal".to_string()),
                _ => ans.push(r.to_string()),
            }
        }
        ans
    }
}

pub fn run() {
    let input = [vec![5, 4, 3, 2, 1], vec![10, 3, 8, 9, 4]];

    for score in input {
        println!("{:?}", Solution::find_relative_ranks(score));
    }
}
