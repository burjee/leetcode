use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Solution::helper(&root, target_sum as i64, vec![])
    }

    pub fn helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i64,
        mut pre_sum: Vec<i64>,
    ) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let mut n = if node.val as i64 == target_sum { 1 } else { 0 };
            for sum in pre_sum.iter_mut() {
                *sum += node.val as i64;
                if *sum == target_sum {
                    n += 1;
                }
            }
            pre_sum.push(node.val as i64);
            return n
                + Solution::helper(&node.left, target_sum, pre_sum.clone())
                + Solution::helper(&node.right, target_sum, pre_sum);
        }
        0
    }

    // pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    //     Solution::helper1(&root, target_sum as i64)
    // }

    // pub fn helper1(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> i32 {
    //     if let Some(node) = root {
    //         let node = node.borrow();
    //         let val = node.val as i64;
    //         let n = if val == target_sum { 1 } else { 0 };
    //         return n
    //             + Solution::helper1(&node.left, target_sum)
    //             + Solution::helper1(&node.right, target_sum)
    //             + Solution::helper2(&node.left, target_sum - val)
    //             + Solution::helper2(&node.right, target_sum - val);
    //     }
    //     0
    // }

    // pub fn helper2(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> i32 {
    //     if let Some(node) = root {
    //         let node = node.borrow();
    //         let val = node.val as i64;
    //         let n = if val == target_sum { 1 } else { 0 };
    //         return n
    //             + Solution::helper2(&node.left, target_sum - val)
    //             + Solution::helper2(&node.right, target_sum - val);
    //     }
    //     0
    // }
}

pub fn run() {
    let input = [
        (
            vec![
                Some(10),
                Some(5),
                Some(-3),
                Some(3),
                Some(2),
                None,
                Some(11),
                Some(3),
                Some(-2),
                None,
                Some(1),
            ],
            8,
        ),
        (
            vec![
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                Some(5),
                Some(1),
            ],
            22,
        ),
        (vec![Some(1)], 1),
        (
            vec![
                Some(1000000000),
                Some(1000000000),
                None,
                Some(294967296),
                None,
                Some(1000000000),
                None,
                Some(1000000000),
                None,
                Some(1000000000),
            ],
            0,
        ),
        (
            vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
                None,
                Some(4),
                None,
                Some(5),
            ],
            3,
        ),
    ];

    for (nums, target_sum) in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::path_sum(root, target_sum));
    }
}
