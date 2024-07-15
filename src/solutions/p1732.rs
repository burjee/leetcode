struct Solution {}
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        for n in gain {
            sum += n;
            ans = ans.max(sum);
        }
        ans
    }
}

pub fn run() {
    let input = [vec![-5, 1, 5, 0, -7], vec![-4, -3, -2, -1, 4, 3, 2]];

    for gain in input {
        println!("{}", Solution::largest_altitude(gain));
    }
}
