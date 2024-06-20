struct Solution {}
impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_by(|a, b| b.cmp(a));
        happiness[0..k as usize]
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &h)| acc + (h as i64 - i as i64).max(0))
    }
}

pub fn run() {
    let input = [
        (vec![1, 2, 3], 2),
        (vec![1, 1, 1, 1], 2),
        (vec![2, 3, 4, 5], 1),
        (vec![12, 1, 42], 3),
    ];

    for (happiness, k) in input {
        println!("{}", Solution::maximum_happiness_sum(happiness, k));
    }
}
