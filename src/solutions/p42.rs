struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut horizontal = 0;
        let mut ans = 0;

        while l < r {
            if height[l] < height[r] {
                horizontal = horizontal.max(height[l]);
                l += 1;
                ans += (horizontal - height[l]).max(0);
            } else {
                horizontal = horizontal.max(height[r]);
                r -= 1;
                ans += (horizontal - height[r]).max(0);
            }
        }

        ans
    }
}

// impl Solution {
//     pub fn trap(height: Vec<i32>) -> i32 {
//         let mut walls = vec![(0, height[0])];
//         let mut ans = 0;
//         for i in 1..height.len() {
//             if height[i] == 0 {
//                 continue;
//             }

//             let mut middle_wall = 0;
//             while let Some(&(j, wall)) = walls.last() {
//                 let len = (i - j - 1) as i32;
//                 if wall <= height[i] {
//                     ans += len * (wall - middle_wall);
//                     walls.pop();
//                     middle_wall = wall;
//                 } else {
//                     ans += len * (height[i] - middle_wall);
//                     break;
//                 }
//             }
//             walls.push((i, height[i]));
//         }

//         ans
//     }
// }

pub fn run() {
    let input = [vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], vec![4, 2, 0, 3, 2, 5]];

    for height in input {
        println!("{}", Solution::trap(height));
    }
}
