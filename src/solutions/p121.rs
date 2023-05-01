use std::cmp;

struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min = prices[0];
        for p in prices {
            min = cmp::min(p, min);
            profit = cmp::max(p - min, profit);
        }
        profit
    }
}

pub fn run() {
    let prices = vec![
        vec![7, 1, 5, 3, 6, 4],
        vec![7, 6, 4, 3, 1],
        vec![3, 7, 1, 2, 4],
    ];
    for price in prices {
        println!("{}", Solution::max_profit(price));
    }
}
