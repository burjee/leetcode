struct Solution {}
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();

        let mut ans = 1;
        let mut end = points[0][1];
        for point in points {
            if point[0] > end {
                end = point[1];
                ans += 1;
            }
            end = end.min(point[1]);
        }
        ans
    }
}

pub fn run() {
    let input = [
        vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]],
        vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]],
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
        vec![
            vec![9, 12],
            vec![1, 10],
            vec![4, 11],
            vec![8, 12],
            vec![3, 9],
            vec![6, 9],
            vec![6, 7],
        ],
    ];

    for points in input {
        println!("{}", Solution::find_min_arrow_shots(points));
    }
}
