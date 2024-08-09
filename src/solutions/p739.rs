struct Solution {}
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; temperatures.len()];
        let mut daily_temperatures: Vec<(usize, i32)> = vec![];
        for (i, temperature) in temperatures.into_iter().enumerate() {
            while let Some(&(j, before_temperature)) = daily_temperatures.last() {
                if before_temperature < temperature {
                    ans[j] = (i - j) as i32;
                    daily_temperatures.pop();
                } else {
                    break;
                }
            }
            daily_temperatures.push((i, temperature));
        }
        ans
    }
}

// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         use std::cmp::Reverse;
//         use std::collections::BinaryHeap;

//         let mut ans = vec![0; temperatures.len()];
//         let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
//         for (i, temperature) in temperatures.into_iter().enumerate() {
//             while let Some(&(before_temperature, j)) = heap.peek() {
//                 if before_temperature.0 < temperature {
//                     ans[j] = (i - j) as i32;
//                     heap.pop();
//                 } else {
//                     break;
//                 }
//             }
//             heap.push((Reverse(temperature), i));
//         }
//         ans
//     }
// }

pub fn run() {
    let input = [
        vec![73, 74, 75, 71, 69, 72, 76, 73],
        vec![30, 40, 50, 60],
        vec![30, 60, 90],
    ];

    for temperatures in input {
        println!("{:?}", Solution::daily_temperatures(temperatures));
    }
}
