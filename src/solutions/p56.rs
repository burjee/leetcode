struct Solution {}
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals: Vec<(i32, i32)> = intervals.into_iter().map(|v| (v[0], v[1])).collect();
        intervals.sort_by(|a, b| b.0.cmp(&a.0));
        let mut i = 0;
        let mut ans = vec![intervals.pop().unwrap()];
        while let Some(val) = intervals.pop() {
            if ans[i].1 < val.0 {
                ans.push(val);
                i += 1;
            } else if ans[i].1 < val.1 {
                ans[i].1 = val.1;
            }
        }
        ans.into_iter().map(|v| vec![v.0, v.1]).collect()
    }
}

pub fn run() {
    let input = vec![
        vec![vec![1, 4], vec![2, 3]],
        vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
        vec![vec![1, 4], vec![4, 5]],
        vec![vec![10, 15], vec![3, 7], vec![6, 8], vec![12, 17]],
    ];

    for intervals in input {
        println!("{:?}", Solution::merge(intervals));
    }
}
