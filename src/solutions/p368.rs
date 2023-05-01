struct Solution {}
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let mut subset = vec![vec![]; nums.len()];
        let mut max_len = 0;
        for i in 0..nums.len() {
            let mut max_sub: Option<usize> = None;
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    if let Some(sub) = max_sub {
                        if subset[j].len() > subset[sub].len() {
                            max_sub = Some(j);
                        }
                    } else {
                        max_sub = Some(j);
                    }
                }
            }
            if let Some(sub) = max_sub {
                let arr = subset[sub].clone();
                subset[i].extend(arr);
            }

            subset[i].push(nums[i]);
            if subset[i].len() > subset[max_len].len() {
                max_len = i;
            }
        }
        subset[max_len].clone()
    }
}

pub fn run() {
    let input = vec![
        vec![1, 2, 3],
        vec![1, 2, 4, 8],
        vec![1, 2, 3, 6, 8, 9, 12, 15, 16, 18],
        vec![2, 6, 33, 66],
    ];
    for nums in input {
        println!("{:?}", Solution::largest_divisible_subset(nums));
    }
}
