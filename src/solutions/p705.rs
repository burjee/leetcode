struct MyHashSet {
    set: Vec<bool>,
}

impl MyHashSet {
    fn new() -> Self {
        let set = vec![false; 1000001];
        MyHashSet { set }
    }

    fn add(&mut self, key: i32) {
        self.set[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.set[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.set[key as usize]
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

pub fn run() {
    enum Cmd {
        Init,
        Add(i32),
        Remove(i32),
        Contains(i32),
    }

    let input = [
        Cmd::Init,
        Cmd::Add(1),
        Cmd::Add(2),
        Cmd::Contains(1),
        Cmd::Contains(3),
        Cmd::Add(2),
        Cmd::Contains(2),
        Cmd::Remove(2),
        Cmd::Contains(2),
    ];

    let mut set = MyHashSet::new();
    for cmd in input {
        match cmd {
            Cmd::Init => {
                set = MyHashSet::new();
                print!("null, ");
            }
            Cmd::Add(key) => {
                set.add(key);
                print!("null, ");
            }
            Cmd::Remove(key) => {
                set.remove(key);
                print!("null, ");
            }
            Cmd::Contains(key) => {
                print!("{}, ", set.contains(key));
            }
        }
    }
}
