use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut nodes = std::collections::VecDeque::new();
        let mut level = 0;
        nodes.push_back(root);
        while !nodes.is_empty() {
            for _ in 0..nodes.len() {
                if let Some(node) = nodes.pop_front().unwrap() {
                    let mut node = node.borrow_mut();
                    if level == ans.len() {
                        ans.push(node.val);
                    }
                    nodes.push_back(node.right.take());
                    nodes.push_back(node.left.take());
                }
            }
            level += 1;
        }
        ans
    }

    // pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut ans = vec![];
    //     Solution::helper(&root, &mut ans, 0);
    //     ans
    // }

    // pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>, level: usize) {
    //     if let Some(node) = root {
    //         let node = node.borrow();
    //         if level == ans.len() {
    //             ans.push(node.val);
    //         }
    //         Solution::helper(&node.right, ans, level + 1);
    //         Solution::helper(&node.left, ans, level + 1);
    //     }
    // }
}

pub fn run() {
    let input = [
        vec![Some(1), Some(2), Some(3), None, Some(5), None, Some(4)],
        vec![Some(1), None, Some(3)],
        vec![],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{:?}", Solution::right_side_view(root));
    }
}
