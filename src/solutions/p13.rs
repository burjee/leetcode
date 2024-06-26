use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        let mut output = 0;
        let mut pre = 0;
        for c in s.chars() {
            let &value = map.get(&c).unwrap();
            output += value;
            if value > pre {
                output -= pre * 2;
            }
            pre = value;
        }
        output
    }
}

pub fn run() {
    let input = ["III", "IV", "IX", "LVIII", "MCMXCIV"];

    for s in input {
        println!("{}", Solution::roman_to_int(s.to_string()));
    }
}
