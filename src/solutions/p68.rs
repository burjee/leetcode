struct Solution {}
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut ans = vec![];
        let mut text: Vec<String> = vec![];
        let mut text_len = 0;
        for word in words {
            if text_len + text.len() + word.len() > max_width {
                for i in 0..max_width - text_len {
                    let j = i % (text.len() - 1).max(1);
                    text[j].push(' ');
                }
                ans.push(text.concat());

                text_len = 0;
                text.clear();
            }
            text_len += word.len();
            text.push(word);
        }

        let mut str_row = text.join(" ");
        str_row.push_str(&(" ".repeat(max_width - str_row.len())));
        ans.push(str_row);

        ans
    }
}

pub fn run() {
    use crate::utils::string::strs_to_string;

    let input = [
        (vec!["This", "is", "an", "example", "of", "text", "justification."], 16),
        (vec!["What", "must", "be", "acknowledgment", "shall", "be"], 16),
        (
            vec![
                "Science",
                "is",
                "what",
                "we",
                "understand",
                "well",
                "enough",
                "to",
                "explain",
                "to",
                "a",
                "computer.",
                "Art",
                "is",
                "everything",
                "else",
                "we",
                "do",
            ],
            20,
        ),
        (
            vec![
                "ask", "not", "what", "your", "country", "can", "do", "for", "you", "ask", "what", "you", "can", "do",
                "for", "your", "country",
            ],
            16,
        ),
    ];

    for (words, max_width) in input {
        println!("{:?}", Solution::full_justify(strs_to_string(words), max_width));
    }
}
