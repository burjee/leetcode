struct Solution {}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut count = vec![1; n + 1];
        for i in 2..=n {
            let mut sum = 0;
            for j in 0..i {
                sum += count[j] * count[i - j - 1];
            }
            count[i] = sum;
        }
        count[n]
    }
}

fn main() {
    for n in 1..=19 {
        println!("{}: {}", n, Solution::num_trees(n));
    }
}
