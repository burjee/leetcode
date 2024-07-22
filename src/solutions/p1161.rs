use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums = vec![];
        let mut nodes = std::collections::VecDeque::new();
        let mut level = 0;
        nodes.push_back(root);
        while !nodes.is_empty() {
            for _ in 0..nodes.len() {
                if let Some(node) = nodes.pop_front().unwrap() {
                    let mut node = node.borrow_mut();
                    if level == nums.len() {
                        nums.push(0);
                    }
                    nums[level] += node.val;
                    nodes.push_back(node.left.take());
                    nodes.push_back(node.right.take());
                }
            }
            level += 1;
        }
        nums.iter().enumerate().rev().max_by_key(|(_, v)| **v).unwrap().0 as i32 + 1
    }

    // pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut nums = vec![];
    //     Solution::helper(&root, &mut nums, 0);
    //     nums.iter().enumerate().rev().max_by_key(|(_, v)| **v).unwrap().0 as i32 + 1
    // }

    // pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>, level: usize) {
    //     if let Some(node) = root {
    //         let node = node.borrow();
    //         if level == nums.len() {
    //             nums.push(0);
    //         }
    //         nums[level] += node.val;
    //         Solution::helper(&node.left, nums, level + 1);
    //         Solution::helper(&node.right, nums, level + 1);
    //     }
    // }
}

pub fn run() {
    let input = [
        vec![Some(1), Some(7), Some(0), Some(7), Some(-8), None, None],
        vec![
            Some(989),
            None,
            Some(10250),
            Some(98693),
            Some(-89388),
            None,
            None,
            None,
            Some(-32127),
        ],
        vec![Some(1), Some(1), Some(0), Some(7), Some(-8), Some(-7), Some(9)],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::max_level_sum(root));
    }
}
