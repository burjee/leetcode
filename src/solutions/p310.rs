use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count = vec![0; n as usize];
        let mut joins = vec![vec![]; n as usize];
        for edge in edges {
            count[edge[0] as usize] += 1;
            count[edge[1] as usize] += 1;
            joins[edge[0] as usize].push(edge[1] as usize);
            joins[edge[1] as usize].push(edge[0] as usize);
        }

        let mut leaves = VecDeque::new();
        count.iter().enumerate().for_each(|(i, &c)| {
            if c <= 1 {
                leaves.push_back(i);
            }
        });

        while n > 2 {
            for _ in 0..leaves.len() {
                n -= 1;
                let leaf = leaves.pop_front().unwrap();
                for &join in &joins[leaf] {
                    count[join] -= 1;
                    if count[join] == 1 {
                        leaves.push_back(join);
                    }
                }
            }
        }

        leaves.into_iter().map(|leaf| leaf as i32).collect()
    }
}

pub fn run() {
    let input = [
        (1, vec![]),
        (2, vec![vec![0, 1]]),
        (4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
        (
            6,
            vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]],
        ),
        (
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![3, 4], vec![4, 5]],
        ),
    ];

    for (n, edges) in input {
        println!("{:?}", Solution::find_min_height_trees(n, edges));
    }
}

// struct Solution {}
// impl Solution {
//     pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
//         if n == 1 {
//             return vec![0];
//         } else if n == 2 {
//             return vec![0, 1];
//         }

//         let mut count = vec![0; n as usize];
//         let mut trees = vec![vec![]; n as usize];
//         for edge in edges {
//             count[edge[0] as usize] += 1;
//             count[edge[1] as usize] += 1;
//             trees[edge[0] as usize].push(edge[1] as usize);
//             trees[edge[1] as usize].push(edge[0] as usize);
//         }

//         let mut joins: Vec<_> = (0..n as usize).collect();
//         let mut leaves: Vec<_> = vec![];
//         while n > 2 {
//             joins.retain(|&join| {
//                 if count[join] == 1 {
//                     n -= 1;
//                     leaves.push(join);
//                 }
//                 count[join] > 1
//             });
//             for leaf in leaves.drain(..) {
//                 for &tree in &trees[leaf] {
//                     count[tree] -= 1;
//                 }
//             }
//         }

//         joins.into_iter().map(|join| join as i32).collect()
//     }
// }

// dfs
// struct Solution {}
// impl Solution {
//     pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
//         if n == 1 {
//             return vec![0];
//         } else if n == 2 {
//             return vec![0, 1];
//         }

//         let mut trees = vec![vec![]; n as usize];
//         for edge in edges {
//             let a = edge[0] as usize;
//             let b = edge[1] as usize;
//             trees[a].push((b, 0));
//             trees[b].push((a, 0));
//         }

//         let mut min_h = i32::MAX;
//         let mut ans = HashSet::new();

//         for i in 0..trees.len() {
//             let new_h = 1 + Solution::helper(i, i, &mut trees);
//             if new_h < min_h {
//                 min_h = new_h;
//                 ans.clear();
//             }
//             if new_h <= min_h {
//                 ans.insert(i as i32);
//             }
//         }
//         ans.into_iter().collect()
//     }

//     pub fn helper(from: usize, to: usize, trees: &mut Vec<Vec<(usize, i32)>>) -> i32 {
//         let mut max = 1;
//         for i in 0..trees[to].len() {
//             if from != trees[to][i].0 {
//                 if trees[to][i].1 == 0 {
//                     trees[to][i].1 = 1 + Solution::helper(to, trees[to][i].0, trees);
//                 }
//                 max = max.max(trees[to][i].1);
//             }
//         }
//         max
//     }
// }
