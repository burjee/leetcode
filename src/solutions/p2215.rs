use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        // or use std::iter::FromIterator; HashSet::from_iter(nums1);
        let mut set1: HashSet<i32> = nums1.into_iter().collect();
        let mut set2: HashSet<i32> = nums2.into_iter().collect();
        for k in set2.clone() {
            if set1.contains(&k) {
                set1.remove(&k);
                set2.remove(&k);
            }
        }
        vec![set1.into_iter().collect(), set2.into_iter().collect()]

        // HashSet可以使用 - 來計算差集，兩邊的set都需要使用引用來計算
        // 因此可以不使用迴圈改為:
        //  vec![
        //    (&set1 - &set2).into_iter().collect(),
        //    (&set2 - &set1).into_iter().collect(),
        // ]
        // 解答
    }
}

pub fn run() {
    let inputs = [
        (vec![1, 2, 3], vec![2, 4, 6]),
        (vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
    ];
    for (nums1, nums2) in inputs {
        println!("{:?}", Solution::find_difference(nums1, nums2));
    }
}
