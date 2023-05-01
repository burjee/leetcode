use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        while n != 1 {
            if set.contains(&n) {
                return false;
            }
            set.insert(n);
            n = Solution::get_next(n);
        }
        true
    }

    pub fn get_next(mut n: i32) -> i32 {
        let mut next = 0;
        while n > 0 {
            let d = n % 10;
            next += d * d;
            n /= 10;
        }
        next
    }
}

pub fn run() {
    let input = vec![
        19, 2, 20, 56, 99, 75, 185616531, 486486465, 864148, 68189614, 412848, 17, 107, 4185, 10,
    ];
    for n in input {
        println!("{}", Solution::is_happy(n));
    }
}
