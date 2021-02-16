struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut nums = nums;
        nums.sort();
        if nums.len() < 3 {
            return ans;
        }

        let mut i = 0;
        let mut l;
        let mut r;
        while i < nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }
            l = i + 1;
            r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum < 0 {
                    l += 1;
                } else if sum > 0 {
                    r -= 1;
                } else {
                    ans.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                }
            }
            while i < nums.len() - 2 && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }
        ans
    }
}

fn main() {
    let numbers = vec![
        vec![0, 0, 0],
        vec![-1, 0, 1, 2, -1, -4],
        vec![-1, 0, 1, 2, -1, -4, 0, 0, 2, 8],
        vec![],
        vec![0],
    ];
    for num in numbers {
        println!("ans: {:?}", Solution::three_sum(num));
    }
}
