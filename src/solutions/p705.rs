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

enum HashSet {
    Init,
    Add(i32),
    Remove(i32),
    Contains(i32),
}

pub fn run() {
    let input = [
        HashSet::Init,
        HashSet::Add(1),
        HashSet::Add(2),
        HashSet::Contains(1),
        HashSet::Contains(3),
        HashSet::Add(2),
        HashSet::Contains(2),
        HashSet::Remove(2),
        HashSet::Contains(2),
    ];

    let mut set = MyHashSet::new();
    for call in input {
        match call {
            HashSet::Init => {
                set = MyHashSet::new();
                print!("null, ");
            }
            HashSet::Add(key) => {
                set.add(key);
                print!("null, ");
            }
            HashSet::Remove(key) => {
                set.remove(key);
                print!("null, ");
            }
            HashSet::Contains(key) => {
                print!("{}, ", set.contains(key));
            }
        }
    }
}
