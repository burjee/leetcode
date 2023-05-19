struct Solution {}
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut gone = vec![false; graph.len()];
        let mut footprint1;
        let mut footprint2;
        for i in 0..graph.len() {
            if gone[i] {
                continue;
            }
            footprint1 = vec![false; graph.len()];
            footprint2 = vec![false; graph.len()];
            if !Solution::helper(&graph, &mut footprint1, &mut footprint2, &mut gone, i) {
                return false;
            }
        }
        true
    }

    pub fn helper(
        graph: &Vec<Vec<i32>>,
        footprint1: &mut Vec<bool>,
        footprint2: &mut Vec<bool>,
        gone: &mut Vec<bool>,
        i: usize,
    ) -> bool {
        gone[i] = true;
        if graph[i].len() <= 1 || footprint1[i] {
            return true;
        }
        footprint1[i] = true;
        for &node in &graph[i] {
            if footprint1[node as usize] {
                return false;
            }
            if !Solution::helper(graph, footprint2, footprint1, gone, node as usize) {
                return false;
            }
        }
        true
    }
}

pub fn run() {
    let input = [
        vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]],
        vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]],
        vec![vec![1], vec![0], vec![3], vec![2]],
    ];
    for graph in input {
        println!("{:?}", Solution::is_bipartite(graph));
    }
}
