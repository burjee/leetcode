use rand::prelude::*;
use std::collections::HashSet;

struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            set: HashSet::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }

    fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }

    fn get_random(&self) -> i32 {
        *self.set.iter().choose(&mut rand::thread_rng()).unwrap()
    }
}

pub fn run() {
    enum Cmd {
        RandomizedSet,
        Insert(i32),
        Remove(i32),
        GetRandom,
    }

    let input = [
        Cmd::RandomizedSet,
        Cmd::Insert(1),
        Cmd::Remove(2),
        Cmd::Insert(2),
        Cmd::GetRandom,
        Cmd::Remove(1),
        Cmd::Insert(2),
        Cmd::GetRandom,
    ];

    let mut randomized_set = RandomizedSet::new();
    for cmd in input {
        match cmd {
            Cmd::RandomizedSet => {
                randomized_set = RandomizedSet::new();
            }
            Cmd::Insert(val) => {
                println!("{}", randomized_set.insert(val));
            }
            Cmd::Remove(val) => {
                println!("{}", randomized_set.remove(val));
            }
            Cmd::GetRandom => {
                println!("{}", randomized_set.get_random());
            }
        }
    }
}
