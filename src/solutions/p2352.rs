struct Solution {}
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::with_capacity(grid.len() * 2);
        for i in 0..grid.len() {
            *map.entry(&grid[i]).or_insert(0) += 1;
        }

        let mut ans = 0;
        let mut temp = vec![0; grid.len()];
        for j in 0..grid.len() {
            for i in 0..grid.len() {
                temp[i] = grid[i][j];
            }
            if let Some(count) = map.get(&temp) {
                ans += count;
            }
        }

        ans
    }
}

pub fn run() {
    let input = [
        vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]],
        vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ],
    ];

    for grid in input {
        println!("{}", Solution::equal_pairs(grid));
    }
}
