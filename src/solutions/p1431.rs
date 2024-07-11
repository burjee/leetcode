struct Solution {}
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        candies
            .into_iter()
            .map(|n| n + extra_candies >= max)
            .collect()
    }

    // pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    //     let mut ans = Vec::with_capacity(candies.len());
    //     let max = *candies.iter().max().unwrap();

    //     for i in 0..candies.len() {
    //         ans.push(candies[i] + extra_candies >= max);
    //     }
    //     ans
    // }
}

pub fn run() {
    let input = [
        (vec![2, 3, 5, 1, 3], 3),
        (vec![4, 2, 1, 1, 2], 1),
        (vec![12, 1, 12], 10),
    ];

    for (candies, extra_candies) in input {
        println!("{:?}", Solution::kids_with_candies(candies, extra_candies));
    }
}
