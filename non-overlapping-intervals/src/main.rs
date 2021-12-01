struct Solution {}
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut end = -50000;
        let mut output = 0;
        for interval in intervals {
            if interval[0] < end {
                output += 1;
                if interval[1] < end {
                    end = interval[1];
                }
            } else {
                end = interval[1];
            }
        }
        output
    }
}

fn main() {
    let input = vec![
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]],
        vec![vec![1, 2], vec![1, 2], vec![1, 2]],
        vec![vec![1, 2], vec![2, 3]],
        vec![
            vec![7, 10],
            vec![6, 9],
            vec![5, 8],
            vec![5, 6],
            vec![3, 6],
            vec![1, 2],
            vec![2, 3],
            vec![4, 5],
        ],
    ];
    for intervals in input {
        println!("{}", Solution::erase_overlap_intervals(intervals));
    }
}
