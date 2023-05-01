struct Solution {}
impl Solution {
    // horizontal
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc, s| {
                acc.chars()
                    .zip(s.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|(c, _)| c)
                    .collect()
            })
            .unwrap()
    }

    // vertical
    pub fn _longest_common_prefix(strs: Vec<String>) -> String {
        let strs: Vec<Vec<char>> = strs.into_iter().map(|s| s.chars().collect()).collect();
        for i in 0..strs[0].len() {
            let c = strs[0][i];
            for chars in &strs {
                if i == chars.len() || c != chars[i] {
                    return strs[0][0..i].iter().collect();
                }
            }
        }
        strs[0].iter().collect()
    }
}

pub fn run() {
    let input = [
        vec!["flower", "flow", "flight"],
        vec!["dog", "racecar", "car"],
    ];
    for strs in input {
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        println!("{}", Solution::longest_common_prefix(strs));
    }
}
