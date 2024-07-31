struct Solution {}
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Solution::helper(k, n, 1, &mut ans, &mut Vec::with_capacity(9));
        ans
    }

    pub fn helper(k: i32, n: i32, i: i32, ans: &mut Vec<Vec<i32>>, combination: &mut Vec<i32>) {
        if k == 0 && n == 0 {
            ans.push(combination.clone());
        } else if i <= n {
            for j in i..=9 {
                combination.push(j);
                Solution::helper(k - 1, n - j, j + 1, ans, combination);
                combination.pop();
            }
        }
    }
}

pub fn run() {
    let input = [(3, 7), (3, 9), (4, 1)];

    for (k, n) in input {
        println!("{:?}", Solution::combination_sum3(k, n));
    }
}
