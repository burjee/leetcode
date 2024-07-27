struct Solution {}
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut path = vec![vec![]; n as usize];
        for connection in connections {
            let a = connection[0] as usize;
            let b = connection[1] as usize;
            path[a].push((b, true));
            path[b].push((a, false));
        }

        let mut ans = 0;
        let mut go = vec![(0, n as usize)];
        while let Some((now, prev)) = go.pop() {
            for &(next, reverse) in &path[now] {
                if prev != next {
                    go.push((next, now));
                    if reverse {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }

    // pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    //     let mut to = vec![vec![]; n as usize];
    //     let mut from = vec![vec![]; n as usize];
    //     for connection in connections {
    //         let a = connection[0] as usize;
    //         let b = connection[1] as usize;
    //         to[a].push(b);
    //         from[b].push(a);
    //     }

    //     let mut ans = 0;
    //     Solution::to(&to, &from, n as usize, &mut ans, 0);
    //     Solution::from(&to, &from, n as usize, &mut ans, 0);
    //     ans
    // }

    // pub fn to(to: &Vec<Vec<usize>>, from: &Vec<Vec<usize>>, prev: usize, ans: &mut i32, i: usize) {
    //     for &city in &to[i] {
    //         if city != prev {
    //             *ans += 1;
    //             Solution::to(to, from, i, ans, city);
    //             Solution::from(to, from, i, ans, city);
    //         }
    //     }
    // }

    // pub fn from(to: &Vec<Vec<usize>>, from: &Vec<Vec<usize>>, prev: usize, ans: &mut i32, i: usize) {
    //     for &city in &from[i] {
    //         if city != prev {
    //             Solution::to(to, from, i, ans, city);
    //             Solution::from(to, from, i, ans, city);
    //         }
    //     }
    // }
}

pub fn run() {
    let input = [
        (6, vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]),
        (5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
    ];

    for (n, connections) in input {
        println!("{}", Solution::min_reorder(n, connections));
    }
}
