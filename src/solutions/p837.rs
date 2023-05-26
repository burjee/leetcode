// 抵達0的機率是1
// 抵達1的機率是1 / max_pts ， 除非k=0 不用抵達1
// 抵達2的機率是1 / max_pts ， 再加上從1抵達2的機率 ， 所以再加上 dp[1] * 1 / max_pts ， 最後dp[2] = 1 / max_pts + dp[1] * 1 / max_pts
// 抵達3的機率是1 / max_pts ， 再加上從1抵達3以及從2抵達3的機率

// 以上可以簡化成
// dp[i] = s / max_pts as f64;
// if i < k {
//     s += dp[i];
// }
// 加上這筆機率，算下一個數量會自動乘 1 / max_pts

// if i < k 才要加上這筆機率是因為抵達k之後就停止前進了
// 所以抵達k後接下來的機率都不會加上一筆機率
// 抵達 k + 1 之後就不會加上 k 的機率，因為在 k 停止前進了
// 抵達 k + 2 之後就不會加上 k 和 k + 1 的機率，因為在 k 和 k+1 停止前進了

// if i as i32 - max_pts as i32 >= 0 && i - max_pts < k {
//     s -= dp[i - max_pts];
// }
// 首先 i as i32 - max_pts as i32 >= 0
// 這個在 1 ~ max_pts - 1 都不會觸發
// 抵達 max_pts 開始觸發 s -= dp[i - max_pts];
// 因為一次累積的點數最大就是 max_pts
// 所以下次機率需要減掉之前不會抵達這邊的機率
// 假設 max_pts 為6
// 那麼 dp[1] 沒有辦法抵達 dp[8]
// 所以在 dp[7] 的結尾要把 dp[1] 的機率減掉
// 這樣 dp[8] 的機率就會正常

// 接著是 i - max_pts < k
// 因為在 i - max_pts 的機率要小於 k 才會繼續前進
// 如果 i - max_pts >= k ， 代表那個機率不會前進，這已經在
// if i < k {
//     s += dp[i];
// }
// 這邊處理過了，所以如果 i - max_pts 的位置已經抵達 k
// 就不需要減掉抵達不了的機率

// 最後將抵達 k 到 n 之間 ( p >= k ， p <= n ) 的機率相加就是答案
struct Solution {}
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;

        let mut dp = vec![0f64; n + 1];
        dp[0] = 1f64;
        let mut s = if k > 0 { 1f64 } else { 0f64 };
        for i in 1..=n {
            dp[i] = s / max_pts as f64;
            if i < k {
                s += dp[i];
            }
            if i as i32 - max_pts as i32 >= 0 && i - max_pts < k {
                s -= dp[i - max_pts];
            }
        }
        dp[k..=n].iter().fold(0f64, |acc, x| acc + x)
    }
}

pub fn run() {
    let input = [(10, 1, 10), (6, 1, 10), (21, 17, 10)];
    for (n, k, max_pts) in input {
        println!("{}", Solution::new21_game(n, k, max_pts));
    }
}
