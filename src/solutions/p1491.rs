struct Solution {}
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = salary[0];
        let mut max = salary[0];
        let mut sum = 0;
        for &n in &salary {
            sum += n;
            min = min.min(n);
            max = max.max(n);
        }
        sum -= min;
        sum -= max;
        (sum as f64) / (salary.len() - 2) as f64
    }
}

pub fn run() {
    let input = [vec![4000, 3000, 1000, 2000], vec![1000, 2000, 3000]];
    for salary in input {
        println!("{}", Solution::average(salary));
    }
}
