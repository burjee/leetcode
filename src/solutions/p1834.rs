use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(tasks.len());
        let mut heap = BinaryHeap::with_capacity(tasks.len());
        let mut tasks = tasks
            .into_iter()
            .enumerate()
            .map(|(i, t)| (t[0], t[1], i as i32))
            .collect::<Vec<(i32, i32, i32)>>();
        tasks.sort_by_key(|t| t.0);

        let mut time = 0;
        let mut i = 0;
        while i < tasks.len() {
            if time < tasks[i].0 && heap.is_empty() {
                time = tasks[i].0;
            }
            while i < tasks.len() && time >= tasks[i].0 {
                heap.push((Reverse(tasks[i].1), Reverse(tasks[i].2)));
                i += 1;
            }
            if let Some((Reverse(p), Reverse(j))) = heap.pop() {
                ans.push(j);
                time += p;
            }
        }
        while let Some((_, Reverse(i))) = heap.pop() {
            ans.push(i);
        }
        ans
    }

    pub fn _get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(tasks.len());
        let mut heap1 = BinaryHeap::with_capacity(tasks.len());
        let mut heap2 = BinaryHeap::with_capacity(tasks.len());

        for (i, task) in tasks.into_iter().enumerate() {
            let (q, p) = (task[0], task[1]);
            heap1.push((Reverse(q), p, i as i32));
        }

        let mut time = 0;
        while !heap1.is_empty() {
            while let Some(&(Reverse(q), p, i)) = heap1.peek() {
                if time >= q {
                    heap1.pop();
                    heap2.push((Reverse(p), Reverse(i)));
                } else {
                    break;
                }
            }
            if let Some((Reverse(p), Reverse(i))) = heap2.pop() {
                ans.push(i);
                time += p;
            } else {
                let (Reverse(q), p, i) = heap1.pop().unwrap();
                heap2.push((Reverse(p), Reverse(i)));
                time = q;
            }
        }
        while let Some((_, Reverse(i))) = heap2.pop() {
            ans.push(i);
        }
        ans
    }
}

pub fn run() {
    let input = [
        vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]],
        vec![vec![7, 10], vec![7, 12], vec![7, 5], vec![7, 4], vec![7, 2]],
        vec![
            vec![19, 13],
            vec![16, 9],
            vec![21, 10],
            vec![32, 25],
            vec![37, 4],
            vec![49, 24],
            vec![2, 15],
            vec![38, 41],
            vec![37, 34],
            vec![33, 6],
            vec![45, 4],
            vec![18, 18],
            vec![46, 39],
            vec![12, 24],
        ],
    ];

    for tasks in input {
        println!("{:?}", Solution::get_order(tasks));
    }
}
