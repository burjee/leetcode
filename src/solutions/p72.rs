struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let mut ans = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            ans[i][n] = m - i;
        }
        for j in 0..n {
            ans[m][j] += n - j;
        }

        for (j, b2) in word2.bytes().enumerate().rev() {
            for (i, b1) in word1.bytes().enumerate().rev() {
                if b1 == b2 {
                    ans[i][j] = ans[i + 1][j + 1];
                } else {
                    ans[i][j] = 1 + ans[i + 1][j].min(ans[i][j + 1].min(ans[i + 1][j + 1]));
                }
            }
        }

        for i in 0..m {
            ans[i][0] += i;
        }
        for j in 0..n {
            ans[0][j] += j;
        }

        ans[0][0] as i32
    }
}

pub fn run() {
    let input = [
        ("horse", "ros"),
        ("intention", "execution"),
        ("horse", "rose"),
        ("horse", "horse"),
        ("prosperity", "properties"),
    ];

    for (word1, word2) in input {
        println!("{}", Solution::min_distance(word1.to_string(), word2.to_string()));
    }
}
