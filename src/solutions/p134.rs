struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut total_have, mut now_have) = (0, 0);
        let mut ans = 0;
        gas.into_iter()
            .zip(cost.into_iter())
            .enumerate()
            .for_each(|(i, (gas, cost))| {
                total_have += gas - cost;
                now_have += gas - cost;

                if now_have < 0 {
                    now_have = 0;
                    ans = i as i32 + 1;
                }
            });

        if total_have >= 0 {
            ans
        } else {
            -1
        }
    }
}

// impl Solution {
//     pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//         let (mut need, mut have) = (0, 0);
//         let mut ans = -1;
//         gas.into_iter()
//             .zip(cost.into_iter())
//             .enumerate()
//             .for_each(|(i, (gas, cost))| {
//                 let remain_gas = gas - cost;
//                 if remain_gas >= 0 {
//                     if have == 0 {
//                         ans = i as i32;
//                     }
//                     have += remain_gas;
//                 } else {
//                     if have > 0 {
//                         have += remain_gas;
//                         if have < 0 {
//                             need += have;
//                             have = 0;
//                         }
//                     } else {
//                         need += remain_gas;
//                     }
//                 }
//             });

//         if need + have >= 0 {
//             ans
//         } else {
//             -1
//         }
//     }
// }

pub fn run() {
    let input = [
        (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        (vec![2, 3, 4], vec![3, 4, 3]),
    ];

    for (gas, cost) in input {
        println!("{}", Solution::can_complete_circuit(gas, cost));
    }
}
