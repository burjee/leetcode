struct Solution {}
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut tri_fib = vec![0, 1, 1];
        for i in 3..=n as usize {
            tri_fib.push(tri_fib[i - 1] + tri_fib[i - 2] + tri_fib[i - 3]);
        }
        tri_fib[n as usize]
    }

    // pub fn tribonacci(n: i32) -> i32 {
    //     Solution::helper(n as usize, &mut vec![-1; 38])
    // }

    // pub fn helper(n: usize, cache: &mut Vec<i32>) -> i32 {
    //     if n == 0 {
    //         0
    //     } else if n == 1 || n == 2 {
    //         1
    //     } else {
    //         if cache[n] == -1 {
    //             cache[n] =
    //                 Solution::helper(n - 1, cache) + Solution::helper(n - 2, cache) + Solution::helper(n - 3, cache);
    //         }
    //         cache[n]
    //     }
    // }
}

pub fn run() {
    let input = [4, 25];

    for n in input {
        println!("{}", Solution::tribonacci(n));
    }
}
