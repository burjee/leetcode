struct Solution {}
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ans = String::with_capacity(word1.len() + word2.len());
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        loop {
            match (chars1.next(), chars2.next()) {
                (Some(c1), Some(c2)) => {
                    ans.push(c1);
                    ans.push(c2);
                }
                (None, Some(c)) | (Some(c), None) => ans.push(c),
                (None, None) => return ans,
            }
        }
    }
}

pub fn run() {
    let input = [("abc", "pqr"), ("ab", "pqrs"), ("abcd", "pq")];

    for (word1, word2) in input {
        println!(
            "{}",
            Solution::merge_alternately(word1.to_string(), word2.to_string())
        );
    }
}
