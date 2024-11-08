struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }

        let mut l = 2;
        while l < nums.len() {
            if nums[l - 2] == nums[l] {
                break;
            }
            l += 1;
        }

        let mut r = l + 1;
        while r < nums.len() {
            nums[l] = nums[r];
            if nums[l - 2] == nums[l] {
                while r < nums.len() && nums[l] == nums[r] {
                    r += 1;
                }
                if r == nums.len() {
                    break;
                }
                nums[l] = nums[r];
            }
            l += 1;
            r += 1;
        }

        l as i32
    }
}

pub fn run() {
    let input = [
        vec![1],
        vec![1, 1, 1],
        vec![1, 1, 2],
        vec![1, 1, 2, 2, 2, 2],
        vec![1, 2, 2, 2, 2, 2],
        vec![1, 1, 1, 2, 2, 3],
        vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
        vec![0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3],
        vec![0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3],
        vec![0, 0, 1, 1, 1, 1, 2, 3, 3, 3, 3, 4, 5, 6, 6, 6, 7, 7, 8, 8, 9],
    ];

    for mut nums in input {
        println!("{}: {:?}", Solution::remove_duplicates(&mut nums), nums);
    }
}
