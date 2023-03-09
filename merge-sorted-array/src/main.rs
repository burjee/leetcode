struct Solution {}
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut l = m + n - 1;
        let mut m = m - 1;
        let mut n = n - 1;

        while m >= 0 && n >= 0 {
            if nums1[m as usize] > nums2[n as usize] {
                nums1[l as usize] = nums1[m as usize];
                m -= 1;
                l -= 1;
            } else {
                nums1[l as usize] = nums2[n as usize];
                n -= 1;
                l -= 1;
            }
        }

        while n >= 0 {
            nums1[l as usize] = nums2[n as usize];
            n -= 1;
            l -= 1;
        }
    }

    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //     let mut l = m + n;
    //     let mut m = m - 1;
    //     let mut n = n - 1;

    //     while l > 0 {
    //         l -= 1;
    //         if m < 0 {
    //             nums1[l as usize] = nums2[n as usize];
    //             n -= 1;
    //         } else if n < 0 {
    //             nums1[l as usize] = nums1[m as usize];
    //             m -= 1;
    //         } else if nums1[m as usize] > nums2[n as usize] {
    //             nums1[l as usize] = nums1[m as usize];
    //             m -= 1;
    //         } else {
    //             nums1[l as usize] = nums2[n as usize];
    //             n -= 1;
    //         }
    //     }
    // }

    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //     nums1.truncate(m as usize);
    //     nums1.append(nums2);
    //     nums1.sort();
    // }
}

fn main() {
    let input = [
        (vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3),
        (vec![1], 1, vec![], 0),
        (vec![0], 0, vec![1], 1),
    ];
    for (mut nums1, m, mut nums2, n) in input {
        Solution::merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1);
    }
}
