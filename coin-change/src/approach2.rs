struct Solution {}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut cache = vec![i32::MAX; amount as usize + 1];
        Solution::helper(&coins[..], amount, &mut cache)
    }

    pub fn helper(coins: &[i32], amount: i32, cache: &mut Vec<i32>) -> i32 {
        if amount == 0 {
            return 0;
        }
        if amount < 0 {
            return -1;
        }
        if cache[amount as usize] != i32::MAX {
            return cache[amount as usize];
        }
        let mut min_count = i32::MAX;
        for coin in coins {
            let count = Solution::helper(coins, amount - coin, cache);
            if count != -1 {
                min_count = min_count.min(1 + count);
            }
        }
        if min_count == i32::MAX {
            min_count = -1;
        };
        cache[amount as usize] = min_count;
        min_count
    }
}

fn main() {
    let input = vec![
        (vec![75, 25, 1], 100),
        (vec![3, 2, 1], 12),
        (vec![431, 62, 88, 428], 9084),
        (
            vec![411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422],
            9864,
        ),
        (vec![1, 333, 800], 999),
        (vec![186, 419, 83, 408], 6249),
        (vec![2, 5, 10, 1], 27),
        (vec![1, 2, 5], 11),
        (vec![3, 5], 11),
        (vec![2], 3),
        (vec![1], 0),
        (vec![1], 1),
        (vec![1], 2),
    ];
    for (coins, amount) in input {
        println!("{}", Solution::coin_change(coins, amount));
    }
}
