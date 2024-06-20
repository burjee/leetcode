struct Height {
    heights: Vec<Vec<i32>>,
    flow: Vec<Vec<u8>>,
    m: usize,
    n: usize,
}
impl Height {
    pub fn new(heights: Vec<Vec<i32>>) -> Height {
        let m = heights.len();
        let n = heights[0].len();
        let flow = vec![vec![0; n]; m];
        Height {
            heights,
            flow,
            m,
            n,
        }
    }
}

const PACIFIC: u8 = 0b10;
const ATLANTIC: u8 = 0b01;
const BOTH: u8 = 0b11;

struct Solution {}
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut height = Height::new(heights);
        let mut output = vec![];
        let k = height.m - 1;
        let l = height.n - 1;
        for j in 0..height.n {
            Solution::helper(&mut height, &mut output, 0, 0, j, PACIFIC);
            Solution::helper(&mut height, &mut output, 0, k, j, ATLANTIC);
        }
        for i in 0..height.m {
            Solution::helper(&mut height, &mut output, 0, i, 0, PACIFIC);
            Solution::helper(&mut height, &mut output, 0, i, l, ATLANTIC);
        }
        output
    }

    pub fn helper(
        height: &mut Height,
        output: &mut Vec<Vec<i32>>,
        pre: i32,
        i: usize,
        j: usize,
        ocean: u8,
    ) {
        if height.flow[i][j] & ocean == 0 && height.heights[i][j] >= pre {
            height.flow[i][j] |= ocean;
            if height.flow[i][j] == BOTH {
                output.push(vec![i as i32, j as i32]);
            }
            if i > 0 {
                Solution::helper(height, output, height.heights[i][j], i - 1, j, ocean);
            }
            if i < height.m - 1 {
                Solution::helper(height, output, height.heights[i][j], i + 1, j, ocean);
            }
            if j > 0 {
                Solution::helper(height, output, height.heights[i][j], i, j - 1, ocean);
            }
            if j < height.n - 1 {
                Solution::helper(height, output, height.heights[i][j], i, j + 1, ocean);
            }
        }
    }
}

pub fn run() {
    let input = [
        vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ],
        vec![vec![2, 1], vec![1, 2]],
    ];

    for heights in input {
        println!("{:?}", Solution::pacific_atlantic(heights));
    }
}
