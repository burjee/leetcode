struct Solution {}
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![true; n as usize];

        for edge in edges {
            ans[edge[1] as usize] = false;
        }

        ans.into_iter()
            .enumerate()
            .filter_map(|(i, b)| if b { Some(i as i32) } else { None })
            .collect()
    }
}

pub fn run() {
    let input = [
        (
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]],
        ),
        (
            5,
            vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]],
        ),
    ];

    for (n, edges) in input {
        println!("{:?}", Solution::find_smallest_set_of_vertices(n, edges));
    }
}
