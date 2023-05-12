struct Solution {}
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut cache = vec![vec![0; len2 + 1]; len1 + 1];
        for i in (0..len1).rev() {
            for j in (0..len2).rev() {
                if nums1[i] == nums2[j] {
                    cache[i][j] = 1 + cache[i + 1][j + 1];
                } else {
                    cache[i][j] = cache[i][j + 1].max(cache[i + 1][j]);
                }
            }
        }
        cache[0][0]
    }

    pub fn _max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut cache = vec![vec![-1; nums2.len()]; nums1.len()];
        Solution::_helper(&nums1, &nums2, &mut cache, 0, 0)
    }

    pub fn _helper(
        nums1: &Vec<i32>,
        nums2: &Vec<i32>,
        cache: &mut Vec<Vec<i32>>,
        i: usize,
        j: usize,
    ) -> i32 {
        if i == nums1.len() || j == nums2.len() {
            return 0;
        }
        if cache[i][j] != -1 {
            return cache[i][j];
        }
        if nums1[i] == nums2[j] {
            cache[i][j] = 1 + Solution::_helper(nums1, nums2, cache, i + 1, j + 1);
        } else {
            let a = Solution::_helper(nums1, nums2, cache, i, j + 1);
            let b = Solution::_helper(nums1, nums2, cache, i + 1, j);
            cache[i][j] = a.max(b);
        }
        cache[i][j]
    }
}

pub fn run() {
    let input = [
        (vec![1, 4, 2], vec![1, 2, 4]),
        (vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        (vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        (
            vec![2, 3, 4, 3, 1, 4, 2, 5, 3, 2, 5, 1, 4, 3, 3],
            vec![5, 1, 2, 1, 3, 2, 3, 4, 5, 3],
        ),
    ];
    for (nums1, nums2) in input {
        println!("{:?}", Solution::max_uncrossed_lines(nums1, nums2));
    }
}
