struct Solution {}
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by(|a, b| b.cmp(a));

        let mut count = 0;
        for &citation in &citations {
            count += 1;
            if count > citation {
                return count - 1;
            }
        }

        citations.len() as i32
    }
}

pub fn run() {
    let input = [
        vec![3, 0, 6, 1, 5],
        vec![1, 3, 1],
        vec![100],
        vec![11, 15],
        vec![0, 0, 0, 11, 15],
        vec![0, 0, 0, 1, 11, 15],
    ];

    for citations in input {
        println!("{}", Solution::h_index(citations));
    }
}
