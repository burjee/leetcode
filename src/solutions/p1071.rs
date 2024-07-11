use std::usize;

struct Solution {}
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let range = Solution::gcd(str1.len(), str2.len());
        let divisor = &str1[0..range];

        if !Solution::helper(divisor, &str1) {
            return "".to_string();
        }
        if !Solution::helper(divisor, &str2) {
            return "".to_string();
        }

        divisor.to_string()
    }

    pub fn helper(divisor: &str, strs: &str) -> bool {
        for i in 0..strs.len() / divisor.len() {
            let temp = &strs[i * divisor.len()..(i + 1) * divisor.len()];
            if divisor != temp {
                return false;
            }
        }
        true
    }

    // pub fn gcd_of_strings(mut str1: String, str2: String) -> String {
    //     let range = Solution::gcd(str1.len(), str2.len());
    //     let divisor: String = str1.drain(..range).collect();

    //     if !Solution::helper(divisor.clone(), str1) {
    //         return "".to_string();
    //     }
    //     if !Solution::helper(divisor.clone(), str2) {
    //         return "".to_string();
    //     }

    //     divisor
    // }

    // pub fn helper(divisor: String, mut strs: String) -> bool {
    //     while strs.len() > 0 {
    //         let temp: String = strs.drain(..divisor.len()).collect();
    //         if divisor != temp {
    //             return false;
    //         }
    //     }
    //     true
    // }

    // 輾轉相除法
    pub fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        Solution::gcd(b, a % b)
    }
}

pub fn run() {
    let input = [("ABCABC", "ABC"), ("ABABAB", "ABAB"), ("LEET", "CODE")];

    for (str1, str2) in input {
        println!(
            "{}",
            Solution::gcd_of_strings(str1.to_string(), str2.to_string())
        );
    }
}
