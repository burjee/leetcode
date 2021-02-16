struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let len = intervals.len();
        if len == 0 {
            return vec![new_interval];
        }
        let mut intervals = intervals;
        if new_interval[1] < intervals[0][0] {
            intervals.insert(0, new_interval);
            return intervals;
        }
        if new_interval[0] > intervals[len - 1][1] {
            intervals.push(new_interval);
            return intervals;
        }
        let mut i = 0;
        let mut s = 0;
        while i < len {
            if new_interval[0] <= intervals[i][1] {
                if new_interval[0] >= intervals[i][0] && new_interval[1] <= intervals[i][1] {
                    return intervals;
                }
                if new_interval[1] < intervals[i][0] {
                    intervals.insert(i, new_interval);
                    return intervals;
                }
                if new_interval[0] < intervals[i][0] {
                    intervals[i][0] = new_interval[0];
                }
                if new_interval[1] > intervals[i][1] {
                    intervals[i][1] = new_interval[1];
                }
                s = i;
                i += 1;
                break;
            }
            i += 1;
        }
        while i < len {
            if intervals[s][1] >= intervals[i][0] {
                intervals[i][0] = -1;
                if intervals[s][1] <= intervals[i][1] {
                    intervals[s][1] = intervals[i][1];
                    break;
                }
            } else {
                break;
            }
            i += 1;
        }
        intervals.retain(|v| v[0] != -1);
        intervals
    }
}

fn main() {
    let input = vec![
        (vec![vec![3, 5], vec![12, 15]], vec![6, 8]),
        (vec![vec![1, 2], vec![4, 6]], vec![3, 5]),
        (vec![vec![3, 4], vec![6, 9]], vec![1, 2]),
        (vec![vec![1, 3], vec![6, 9]], vec![10, 15]),
        (vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        (
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
        ),
        (
            vec![vec![1, 2], vec![6, 7], vec![8, 10], vec![12, 16]],
            vec![4, 8],
        ),
        (vec![], vec![5, 7]),
        (vec![vec![1, 5]], vec![2, 3]),
        (vec![vec![1, 5]], vec![2, 7]),
        (
            vec![vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9]],
            vec![1, 10],
        ),
    ];

    for intervals in input {
        println!("{:?}", Solution::insert(intervals.0, intervals.1));
    }
}
