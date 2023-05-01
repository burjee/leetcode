use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    big: BinaryHeap<Reverse<i32>>,
    small: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            big: BinaryHeap::new(),
            small: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small.push(num);
        if let Some(n) = self.small.pop() {
            self.big.push(Reverse(n));
        }
        if self.small.len() < self.big.len() {
            if let Some(n) = self.big.pop() {
                self.small.push(n.0);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() == self.big.len() {
            let small = *self.small.peek().unwrap();
            let big = self.big.peek().unwrap().0;
            (small + big) as f64 / 2.0
        } else {
            *self.small.peek().unwrap() as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
pub fn run() {
    enum Cmd {
        MedianFinder,
        AddNum,
        FindMedian,
    }

    let input: Vec<(Vec<Cmd>, Vec<Vec<i32>>)> = vec![(
        vec![
            Cmd::MedianFinder,
            Cmd::AddNum,
            Cmd::AddNum,
            Cmd::FindMedian,
            Cmd::AddNum,
            Cmd::FindMedian,
        ],
        vec![vec![], vec![1], vec![2], vec![], vec![3], vec![]],
    )];

    for (cmds, nums) in input {
        let mut median_finder = MedianFinder::new();
        for (cmd, num) in cmds.into_iter().zip(nums.into_iter()) {
            match cmd {
                Cmd::MedianFinder => {
                    median_finder = MedianFinder::new();
                    print!("null, ");
                }
                Cmd::AddNum => {
                    for n in num {
                        median_finder.add_num(n);
                        print!("null, ");
                    }
                }
                Cmd::FindMedian => {
                    print!("{}, ", median_finder.find_median());
                }
            }
        }
        println!();
    }
}
