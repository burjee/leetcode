struct Solution {}
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            0
        } else if num % 9 == 0 {
            9
        } else {
            num % 9
        }
    }

    pub fn _add_digits1(mut num: i32) -> i32 {
        while num > 9 {
            num = num % 10 + num / 10;
        }
        num
    }

    pub fn _add_digits2(mut num: i32) -> i32 {
        while num > 9 {
            let mut temp = 0;
            while num > 0 {
                temp = temp + num % 10;
                num /= 10;
            }
            num = temp;
        }
        num
    }
}

pub fn run() {
    let input = [38, 0, 9, 99, 123, 2438186];
    for num in input {
        println!("{}", Solution::add_digits(num));
    }
}
