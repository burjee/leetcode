use crate::utils::string::vec_strs_to_string;
use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut map = HashMap::new();
        for (equation, value) in equations.iter().zip(values.iter()) {
            let a = &equation[0];
            let b = &equation[1];
            map.entry(a).or_insert(vec![]).push((b, *value));
            map.entry(b).or_insert(vec![]).push((a, 1f64 / value));
        }

        let mut ans = Vec::with_capacity(queries.len());
        let mut set = HashSet::new();
        for query in queries {
            set.clear();
            let a = &query[0];
            let z = &query[1];
            if map.contains_key(z) {
                ans.push(Solution::helper(&map, &mut set, a, z, 1f64));
            } else {
                ans.push(-1f64);
            }
        }
        ans
    }

    pub fn helper<'a>(
        map: &HashMap<&String, Vec<(&'a String, f64)>>,
        set: &mut HashSet<&'a String>,
        a: &String,
        z: &String,
        div: f64,
    ) -> f64 {
        if let Some(pair) = map.get(a) {
            for (b, value) in pair {
                if *b == z {
                    return div * value;
                }
                if set.insert(*b) {
                    let v = Solution::helper(map, set, b, z, div * value);
                    if v != -1f64 {
                        return v;
                    }
                }
            }
        }
        -1f64
    }
}

pub fn run() {
    let input = [
        (
            vec![vec!["a", "b"], vec!["b", "c"]],
            vec![2.0f64, 3.0],
            vec![
                vec!["a", "c"],
                vec!["b", "a"],
                vec!["a", "e"],
                vec!["a", "a"],
                vec!["x", "x"],
            ],
        ),
        (
            vec![vec!["a", "b"], vec!["b", "c"], vec!["bc", "cd"]],
            vec![1.5f64, 2.5, 5.0],
            vec![vec!["a", "c"], vec!["c", "b"], vec!["bc", "cd"], vec!["cd", "bc"]],
        ),
        (
            vec![vec!["a", "b"]],
            vec![0.5f64],
            vec![vec!["a", "b"], vec!["b", "a"], vec!["a", "c"], vec!["x", "y"]],
        ),
        (
            vec![vec!["x1", "x2"], vec!["x2", "x3"], vec!["x3", "x4"], vec!["x4", "x5"]],
            vec![3f64, 4.0, 5.0, 6.0],
            vec![
                vec!["x1", "x5"],
                vec!["x5", "x2"],
                vec!["x2", "x4"],
                vec!["x2", "x2"],
                vec!["x2", "x9"],
                vec!["x9", "x9"],
            ],
        ),
    ];

    for (equations, values, queries) in input {
        let equations = vec_strs_to_string(equations);
        let queries = vec_strs_to_string(queries);
        println!("{:?}", Solution::calc_equation(equations, values, queries));
    }
}
