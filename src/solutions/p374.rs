/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

struct Solution {}
impl Solution {
    unsafe fn guess_number(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n as i64;
        while l <= r {
            let m = (l + r) / 2;
            let result = guess(m as i32);
            if result == 0 {
                return m as i32;
            } else if result == 1 {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        unreachable!()
    }
}

static mut PICK: i32 = 0;
unsafe fn guess(num: i32) -> i32 {
    if num == PICK {
        0
    } else if num > PICK {
        -1
    } else {
        1
    }
}

pub fn run() {
    let input = [(10, 6), (1, 1), (2, 1), (2126753390, 1702766719)];

    for (n, pick) in input {
        unsafe {
            PICK = pick;
            println!("{}", Solution::guess_number(n));
        }
    }
}
