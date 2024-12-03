struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut str_rows = vec![String::new(); num_rows as usize];
        let mut row = 0;
        let mut offset = 1;
        for ch in s.chars() {
            str_rows[row as usize].push(ch);
            row += offset;

            if row == 0 {
                offset = 1;
            }
            if row == num_rows - 1 {
                offset = -1;
            }
        }
        str_rows.concat()
    }
}

pub fn run() {
    let input = [("PAYPALISHIRING", 3), ("PAYPALISHIRING", 4), ("A", 1), ("AB", 1)];

    for (s, num_rows) in input {
        println!("{}", Solution::convert(s.to_string(), num_rows));
    }
}
