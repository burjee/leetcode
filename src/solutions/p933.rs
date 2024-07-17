use std::collections::VecDeque;
struct RecentCounter {
    queue: VecDeque<i32>,
}
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        while self.queue[0] < t - 3000 {
            self.queue.pop_front();
        }

        self.queue.len() as i32
    }
}
/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

pub fn run() {
    enum Cmd {
        RecentCounter,
        Ping(i32),
    }

    let input = [vec![
        Cmd::RecentCounter,
        Cmd::Ping(1),
        Cmd::Ping(100),
        Cmd::Ping(3001),
        Cmd::Ping(3002),
    ]];

    let mut recent_counter = RecentCounter::new();
    for commands in input {
        for cmd in commands {
            match cmd {
                Cmd::RecentCounter => {
                    recent_counter = RecentCounter::new();
                    print!("null, ")
                }
                Cmd::Ping(t) => print!("{}, ", recent_counter.ping(t)),
            }
        }
    }
}
