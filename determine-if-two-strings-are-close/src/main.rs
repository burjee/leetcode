struct Solution {}
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut char_count1 = [0; 26];
        let mut char_count2 = [0; 26];

        for (c1, c2) in word1.bytes().zip(word2.bytes()) {
            char_count1[(c1 - b'a') as usize] += 1;
            char_count2[(c2 - b'a') as usize] += 1;
        }

        for (c1, c2) in char_count1.iter().zip(char_count2.iter()) {
            if (c1 == &0) != (c2 == &0) {
                return false;
            }
        }

        char_count1.sort();
        char_count2.sort();

        char_count1 == char_count2
    }
}

fn main() {
    let words = [
        ("abc".to_string(), "bca".to_string()),
        ("a".to_string(), "aa".to_string()),
        ("cabbba".to_string(), "abbccc".to_string()),
        (
            "aaabbbbccddeeeeefffff".to_string(),
            "aaaaabbcccdddeeeeffff".to_string(),
        ),
    ];
    for (word1, word2) in words {
        println!("{}", Solution::close_strings(word1, word2));
    }
}
