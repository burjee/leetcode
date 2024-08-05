struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut hold = -prices[0]; // 持有的股票，實現損益時會變成正的
        let mut profit = 0; // 賣掉的利潤
        for i in 1..prices.len() {
            // 價格跌時，更新應該持有的價位
            // 時間線之前持有的最佳價位 與 現在的總利潤扣掉售價 做比較
            // 總利潤即之前每次買入賣出的收入加總
            hold = hold.max(profit - prices[i]);

            // 價格漲時，更新售出的利潤
            // 時間線之前售出的總最大利潤 與 現在的價位扣掉手續費再加上持有的股票利潤 做比較
            // 持有的股票已扣掉購買時的價格
            profit = profit.max(prices[i] - fee + hold);
        }

        profit
    }

    // pub fn max_profit(mut prices: Vec<i32>, fee: i32) -> i32 {
    //     prices.push(i32::MIN);
    //     let mut ans = vec![];
    //     let mut min = prices[0];
    //     let mut max = prices[0];
    //     for i in 0..prices.len() - 1 {
    //         max = max.max(prices[i]);
    //         min = min.min(prices[i]);
    //         if max >= prices[i + 1] + fee {
    //             ans.push((max - min - fee).max(0));
    //             min = prices[i + 1];
    //             max = prices[i + 1];
    //         }
    //     }
    //     ans.iter().sum()
    // }
}

pub fn run() {
    let input = [
        (vec![1, 3, 2, 8, 4, 9], 2),
        (vec![1, 3, 7, 5, 10, 3], 3),
        (vec![1, 4, 6, 2, 8, 3, 10, 14], 3),
        (vec![4, 5, 2, 4, 3, 3, 1, 2, 5, 4], 1),
        (vec![2, 1, 4, 4, 2, 3, 2, 5, 1, 2], 1),
        (vec![2, 2, 1, 1, 5, 5, 3, 1, 5, 4], 2),
    ];

    for (prices, fee) in input {
        println!("{}", Solution::max_profit(prices, fee));
    }
}
