struct Solution {}
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut road = vec![vec![]; num_courses as usize];
        for req in prerequisites {
            road[req[1] as usize].push(req[0] as usize);
        }
        for i in 0..num_courses {
            if Solution::helper(
                &mut road,
                &mut vec![false; num_courses as usize],
                i as usize,
            ) {
                return false;
            }
        }
        true
    }

    pub fn helper(road: &mut Vec<Vec<usize>>, used: &mut Vec<bool>, i: usize) -> bool {
        used[i] = true;
        for j in 0..road[i].len() {
            if used[road[i][j]] || Solution::helper(road, used, road[i][j]) {
                return true;
            }
        }
        road[i].clear();
        used[i] = false;
        false
    }
}

pub fn run() {
    let input = [
        (2, vec![vec![1, 0]]),
        (2, vec![vec![1, 0], vec![0, 1]]),
        (3, vec![vec![1, 0], vec![2, 1]]),
        (5, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 2]]),
        (5, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![1, 3]]),
        (
            9,
            vec![
                vec![1, 0],
                vec![2, 1],
                vec![3, 2],
                vec![4, 1],
                vec![5, 1],
                vec![6, 5],
                vec![7, 6],
                vec![8, 7],
                vec![8, 5],
            ],
        ),
        (
            9,
            vec![
                vec![1, 0],
                vec![2, 1],
                vec![3, 2],
                vec![4, 1],
                vec![5, 1],
                vec![6, 5],
                vec![7, 6],
                vec![8, 7],
                vec![5, 8],
            ],
        ),
    ];

    for (num_courses, prerequisites) in input {
        println!("{}", Solution::can_finish(num_courses, prerequisites));
    }
}
