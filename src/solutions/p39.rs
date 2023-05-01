struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut ans = Vec::new();
        Solution::get_ans(&candidates, &mut ans, Vec::new(), target, 0);
        ans
    }

    pub fn get_ans(
        candidates: &Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        vec: Vec<i32>,
        target: i32,
        start: usize,
    ) {
        for i in start..candidates.len() {
            if candidates[i] > target {
                break;
            }
            let mut new_vec = vec.clone();
            new_vec.push(candidates[i]);
            if candidates[i] == target {
                ans.push(new_vec);
                break;
            }
            Solution::get_ans(candidates, ans, new_vec, target - candidates[i], i);
        }
    }
}

pub fn run() {
    let input = vec![
        (vec![2, 3, 6, 7], 7),
        (vec![2, 3, 5], 8),
        (vec![2, 3, 5], 23),
        (vec![2], 1),
        (vec![1], 1),
        (vec![1], 2),
    ];

    for val in input {
        println!("ans: {:?}", Solution::combination_sum(val.0, val.1));
    }
}
