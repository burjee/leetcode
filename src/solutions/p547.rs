struct Solution {}
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut connected = vec![false; is_connected.len()];
        for i in 0..is_connected.len() {
            if !connected[i] {
                Solution::helper(&is_connected, &mut connected, i);
                ans += 1;
            }
        }
        ans
    }

    pub fn helper(is_connected: &Vec<Vec<i32>>, connected: &mut Vec<bool>, i: usize) {
        connected[i] = true;
        for j in 0..is_connected.len() {
            if is_connected[i][j] == 1 {
                if !connected[j] {
                    Solution::helper(is_connected, connected, j);
                }
            }
        }
    }
}

pub fn run() {
    let input = [
        vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]],
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]],
    ];

    for is_connected in input {
        println!("{}", Solution::find_circle_num(is_connected));
    }
}
