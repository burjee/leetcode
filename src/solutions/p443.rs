struct Solution {}
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut c;
        let mut n;

        while j < chars.len() {
            c = chars[j];
            n = 0;
            while j < chars.len() && chars[j] == c {
                n += 1;
                j += 1;
            }

            chars[i] = c;
            if n > 1 {
                for c in n.to_string().chars() {
                    i += 1;
                    chars[i] = c;
                }
            }
            i += 1;
        }

        chars.drain(i..);
        chars.len() as i32
    }
}

pub fn run() {
    let input = [
        vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'],
        vec!['a'],
        vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ],
        vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'],
    ];

    for mut chars in input {
        Solution::compress(&mut chars);
        println!("{:?}", chars);
    }
}
