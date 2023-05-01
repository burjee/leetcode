struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }
        profit
    }
}

pub fn run() {
    let input = vec![
        vec![7, 1, 5, 3, 6, 4],
        vec![1, 2, 3, 4, 5],
        vec![7, 6, 4, 3, 1],
    ];
    for prices in input {
        println!("{}", Solution::max_profit(prices));
    }
}
