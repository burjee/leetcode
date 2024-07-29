struct Solution {}
impl Solution {
    pub fn total_cost(costs: Vec<i32>, mut k: i32, candidates: i32) -> i64 {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, VecDeque};

        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();
        let mut costs = VecDeque::from(costs);
        for _ in 0..candidates {
            if let Some(cost) = costs.pop_front() {
                left.push(Reverse(cost));
            }
            if let Some(cost) = costs.pop_back() {
                right.push(Reverse(cost));
            }
            if costs.is_empty() {
                break;
            }
        }

        let mut ans = 0;
        while k > 0 {
            k -= 1;
            match (left.peek(), right.peek()) {
                (Some(l), Some(r)) => {
                    if l.0 <= r.0 {
                        ans += left.pop().unwrap().0 as i64;
                        if let Some(cost) = costs.pop_front() {
                            left.push(Reverse(cost));
                        }
                    } else {
                        ans += right.pop().unwrap().0 as i64;
                        if let Some(cost) = costs.pop_back() {
                            right.push(Reverse(cost));
                        }
                    }
                }
                (Some(_), None) => ans += left.pop().unwrap().0 as i64,
                (None, Some(_)) => ans += right.pop().unwrap().0 as i64,
                _ => unreachable!(),
            }
        }

        ans
    }
}
pub fn run() {
    let input = [(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4), (vec![1, 2, 4, 1], 3, 3)];

    for (costs, k, candidates) in input {
        println!("{}", Solution::total_cost(costs, k, candidates));
    }
}
