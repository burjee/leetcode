struct Solution {}
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman = vec![
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ];

        let mut ans = String::new();
        while num > 0 {
            let &(n, s) = roman.last().unwrap();
            if n > num {
                roman.pop();
                continue;
            }

            num -= n;
            ans.push_str(s);
        }

        ans
    }
}

pub fn run() {
    let input = [3749, 58, 1994, 3999];

    for num in input {
        println!("{}", Solution::int_to_roman(num));
    }
}
