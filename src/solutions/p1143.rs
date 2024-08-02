struct Solution {}
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();
        let mut ans = vec![vec![0; n + 1]; m + 1];
        for (i, c1) in text1.chars().enumerate() {
            for (j, c2) in text2.chars().enumerate() {
                if c1 == c2 {
                    ans[i + 1][j + 1] = 1 + ans[i][j];
                } else {
                    ans[i + 1][j + 1] = ans[i + 1][j].max(ans[i][j + 1]);
                }
            }
        }

        ans[m][n]
    }

    // pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    //     let mut char_index = vec![vec![]; 26];
    //     for (i, c) in text2.chars().enumerate() {
    //         let j = (c as u8 - 97) as usize;
    //         char_index[j].push(i);
    //     }

    //     let mut sequence = vec![];
    //     for c in text1.chars() {
    //         let j = (c as u8 - 97) as usize;
    //         if !char_index[j].is_empty() {
    //             for i in (0..sequence.len()).rev() {
    //                 let point = char_index[j].partition_point(|x| x <= &sequence[i]);
    //                 if point < char_index[j].len() {
    //                     if i == sequence.len() - 1 {
    //                         sequence.push(char_index[j][point]);
    //                     } else if char_index[j][point] < sequence[i + 1] {
    //                         sequence[i + 1] = char_index[j][point];
    //                     }
    //                 }
    //             }
    //             if sequence.is_empty() {
    //                 sequence.push(char_index[j][0]);
    //             } else if char_index[j][0] < sequence[0] {
    //                 sequence[0] = char_index[j][0];
    //             }
    //         }
    //     }

    //     sequence.len() as i32
    // }
}

pub fn run() {
    let input = [
        ("abcde", "ace"),
        ("abc", "abc"),
        ("abc", "def"),
        ("pmjghexybyrgzczy", "hafcdqbgncrcbihkd"),
        ("ghbrgc", "hcbgcrcbh"),
    ];

    for (text1, text2) in input {
        println!(
            "{}",
            Solution::longest_common_subsequence(text1.to_string(), text2.to_string())
        );
    }
}
