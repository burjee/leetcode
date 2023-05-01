struct Solution {}
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut output = String::new();
        while column_number > 0 {
            column_number -= 1;
            output = format!("{}{}", ((65 + column_number % 26) as u8) as char, output);
            column_number /= 26;
        }
        output
    }

    pub fn _convert_to_title2(column_number: i32) -> String {
        let mut column_number = column_number as usize;
        let column = [
            'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
        ];
        let mut output = String::new();
        while column_number > 0 {
            let c = column[column_number % 26];
            output = format!("{}{}", c, output);
            column_number = (column_number - 1) / 26;
        }
        output
    }
}

pub fn run() {
    let input = [1, 26, 27, 28, 52, 701, 5566, 55688, 54321, 987654321];
    for column_number in input {
        println!("{}", Solution::convert_to_title(column_number));
    }
}
