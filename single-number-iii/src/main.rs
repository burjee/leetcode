// 0 ^ a = a
// 0 ^ a ^ a = 0

// 步驟一
// xor 陣列全部值，得到 a ^ b

// 步驟二
// (a ^ b) & -(a ^ b) "得到最右邊值為1的bit"

// 步驟二解說
// 假設 a ^ b = 0b11010
// "最右邊值為1的bit" = 0b00010
// 在a ^ b的值中，bit為1代表兩方該位元其中一個值為0，另一個值為1
// 因此 "最右邊值為1的bit" 與兩方(a或b)做&運算，其中一個的值會不為零
// 利用這個原理，在陣列中只與&運算後不為零的數值做^運算，就可以只得到a或b，因為兩方其中一個&運算的值會是0

// 步驟三
// xor 陣列中&運算不為零的值，得到a或b

// 步驟四
// 利用a或b與步驟一的a ^ b得出另一個值

struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut a_xor_b = 0;
        for n in &nums {
            a_xor_b ^= n;
        }

        let right_bit = a_xor_b & -a_xor_b;
        let mut a = 0;
        for n in nums {
            if right_bit & n != 0 {
                a ^= n;
            }
        }
        vec![a, a ^ a_xor_b]
    }
}

fn main() {
    let input = vec![
        vec![1, 2, 1, 3, 2, 5],
        vec![-1, 0],
        vec![0, 1],
        vec![0, 0, 1, 1, 2, 3, 3, 2, 4, 5, 6, 7, 8, 9, 8, 9, 7, 6],
    ];

    for nums in input {
        println!("{:?}", Solution::single_number(nums));
    }
}
