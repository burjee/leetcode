struct Solution {}
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Solution::helper(&graph, vec![], &mut ans, 0);
        ans
    }

    pub fn helper(graph: &Vec<Vec<i32>>, mut path: Vec<i32>, ans: &mut Vec<Vec<i32>>, i: usize) {
        path.push(i as i32);
        if i == graph.len() - 1 {
            ans.push(path);
        } else {
            for &node in &graph[i] {
                Solution::helper(graph, path.clone(), ans, node as usize);
            }
        }
    }
}

pub fn run() {
    let input = [
        vec![vec![1, 2], vec![3], vec![3], vec![]],
        vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]],
    ];
    for graph in input {
        println!("{:?}", Solution::all_paths_source_target(graph));
    }
}
