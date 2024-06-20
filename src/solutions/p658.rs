use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let l = arr.len();

        if x <= arr[0] {
            return arr[0..k].into();
        }
        if x >= *arr.last().unwrap() {
            return arr[l - k..l].into();
        }

        let m = arr.binary_search(&x).unwrap_or_else(|x| x);
        let mut ans = VecDeque::with_capacity(k);
        let mut l = m - 1;
        let mut r = m;
        while ans.len() < k {
            let d0 = (arr[l] - x).abs();
            let d1 = (arr[r] - x).abs();
            if d0 <= d1 {
                ans.push_front(arr[l]);
                if l == 0 {
                    let qq = k - ans.len();
                    let remain = arr[r..r + qq].to_vec();
                    ans.extend(remain);
                    break;
                } else {
                    l -= 1;
                }
            } else {
                ans.push_back(arr[r]);
                if r == arr.len() - 1 {
                    let qq = k - ans.len() - 1;
                    let mut remain = arr[l - qq..=l].to_vec();
                    remain.extend(ans);
                    ans = remain.into();
                    break;
                } else {
                    r += 1;
                }
            }
        }
        ans.into()
    }
}

pub fn run() {
    let input = [
        (vec![1, 2, 3, 4, 5], 4, 3),
        (vec![1, 2, 3, 4, 5], 4, -1),
        (vec![1, 1, 1, 10, 10, 10], 1, 9),
        (vec![-2, -1, 1, 2, 3, 4, 5], 7, 3),
        (vec![0, 1, 1, 1, 2, 3, 6, 7, 8, 9], 9, 4),
    ];

    for (arr, k, x) in input {
        println!("{:?}", Solution::find_closest_elements(arr, k, x));
    }
}

/* approach2
use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ans = VecDeque::new();
        let mut dis = VecDeque::new();
        for n in arr {
            let d = (n - x).abs();
            if ans.len() < k {
                ans.push_back(n);
                dis.push_back(d);
            } else {
                if n == *ans.front().unwrap() {
                    continue;
                } else if d < *dis.front().unwrap() {
                    ans.pop_front();
                    dis.pop_front();
                    ans.push_back(n);
                    dis.push_back(d);
                } else {
                    break;
                }
            }
        }
        ans.into()
    }
}

pub fn run() {
    let input = [
        (vec![1, 2, 3, 4, 5], 4, 3),
        (vec![1, 2, 3, 4, 5], 4, -1),
        (vec![1, 1, 1, 10, 10, 10], 1, 9),
    ];
    for (arr, k, x) in input {
        println!("{:?}", Solution::find_closest_elements(arr, k, x));
    }
}
 */
