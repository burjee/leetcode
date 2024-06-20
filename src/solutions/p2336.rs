use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    cursor: i32,
    set: BTreeSet<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet {
            cursor: 1,
            set: BTreeSet::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.set.is_empty() {
            self.cursor += 1;
            return self.cursor - 1;
        }
        let &n = self.set.iter().next().unwrap();
        self.set.remove(&n);
        n
    }

    fn add_back(&mut self, num: i32) {
        if self.cursor > num {
            self.set.insert(num);
        }
    }
}

// 另一種解法
// 使用從大到小排序的vec
// vec[0]存cursor
// 比vec[0]小就二分搜尋他
// 搜尋到就不管 找不到就按照索引插入
// 提取時vec長度>1就pop最小值
// 否則vec[0]+1

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

pub fn run() {
    enum Cmd {
        SmallestInfiniteSet,
        Add(Vec<i32>),
        Pop,
    }

    let input = [
        Cmd::SmallestInfiniteSet,
        Cmd::Add(vec![2]),
        Cmd::Pop,
        Cmd::Pop,
        Cmd::Pop,
        Cmd::Add(vec![1]),
        Cmd::Pop,
        Cmd::Pop,
        Cmd::Pop,
    ];

    let mut set = SmallestInfiniteSet::new();
    for cmd in input {
        match cmd {
            Cmd::SmallestInfiniteSet => {
                set = SmallestInfiniteSet::new();
            }
            Cmd::Add(nums) => {
                for num in nums {
                    set.add_back(num);
                }
            }
            Cmd::Pop => {
                println!("{}", set.pop_smallest());
            }
        }
    }
}
