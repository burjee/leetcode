struct Solution {}
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = *piles.iter().max().unwrap();
        while l <= r {
            let mut c = 0i64;
            let m = (l + r) / 2;
            for &pile in &piles {
                c += ((pile + m - 1) / m) as i64;
            }
            if c <= h as i64 {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

pub fn run() {
    let input = [
        (vec![3, 6, 7, 11], 8),
        (vec![30, 11, 23, 4, 20], 5),
        (vec![30, 11, 23, 4, 20], 6),
        (vec![312884470], 312884469),
        (vec![805306368, 805306368, 805306368], 1000000000),
    ];

    for (piles, h) in input {
        println!("{}", Solution::min_eating_speed(piles, h));
    }
}
