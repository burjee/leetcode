struct Solution {}
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        use std::cmp::Ordering::{Equal, Greater, Less};

        #[derive(PartialEq, Eq)]
        enum Mountain {
            Start,
            Up,
            Down,
            Flat,
        }

        let mut prev = Mountain::Start;
        let mut top = None;
        let mut get = 1;
        let mut ans = 1;

        for i in 1..ratings.len() {
            match ratings[i].cmp(&ratings[i - 1]) {
                Greater => {
                    if prev == Mountain::Down {
                        Solution::check_top(&mut top, get + 1, &mut ans);
                        get = 1;
                    }
                    prev = Mountain::Up;
                }
                Equal => {
                    if prev == Mountain::Down {
                        Solution::check_top(&mut top, get + 1, &mut ans);
                    }
                    get = 0;
                    prev = Mountain::Flat;
                }
                Less => {
                    if prev != Mountain::Down {
                        top = Some(get);
                        get = 0;
                    }
                    prev = Mountain::Down;
                }
            }
            get += 1;
            ans += get;
        }
        Solution::check_top(&mut top, get + 1, &mut ans);

        ans
    }

    pub fn check_top(top: &mut Option<i32>, new_get: i32, ans: &mut i32) {
        if let Some(top_get) = top {
            if *top_get < new_get {
                *ans += new_get - *top_get;
            }
        }
        *top = None;
    }
}

pub fn run() {
    let input = [
        vec![1, 0, 2],
        vec![1, 2, 2],
        vec![1, 2, 2, 2],
        vec![1, 2, 2, 1],
        vec![1, 2, 2, 3],
        vec![1, 2, 3, 4, 5, 5],
        vec![10, 20, 19, 18, 17, 16, 15, 14],
    ];

    for ratings in input {
        println!("{}", Solution::candy(ratings));
    }
}
