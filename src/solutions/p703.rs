use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k {
                heap.pop();
            }
        }
        KthLargest { k, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

// struct KthLargest {
//     k: usize,
//     nums: Vec<i32>,
// }

// impl KthLargest {
//     fn new(k: i32, mut nums: Vec<i32>) -> Self {
//         let k = k as usize;
//         nums.sort();
//         KthLargest { k, nums }
//     }

//     fn add(&mut self, val: i32) -> i32 {
//         let idx = self.nums.partition_point(|&x| x < val);
//         self.nums.insert(idx, val);
//         self.nums[self.nums.len() - self.k]
//     }
// }

enum Kth {
    Init { k: i32, nums: Vec<i32> },
    Add(i32),
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
pub fn run() {
    let input = [
        Kth::Init {
            k: 3,
            nums: vec![4, 5, 8, 2],
        },
        Kth::Add(3),
        Kth::Add(5),
        Kth::Add(10),
        Kth::Add(9),
        Kth::Add(4),
    ];

    let mut kth = KthLargest::new(0, vec![]);
    for cmd in input {
        match cmd {
            Kth::Init { k, nums } => {
                kth = KthLargest::new(k, nums);
                print!("null, ");
            }
            Kth::Add(val) => {
                print!("{}, ", kth.add(val));
            }
        }
    }
}
