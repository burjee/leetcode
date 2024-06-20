use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root, i32::MAX, i32::MIN)
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let min = min.min(node.val);
            let max = max.max(node.val);
            Solution::helper(&node.left, min, max).max(Solution::helper(&node.right, min, max))
        } else {
            max - min
        }
    }
}

pub fn run() {
    let input = [
        vec![
            Some(8),
            Some(3),
            Some(10),
            Some(1),
            Some(6),
            None,
            Some(14),
            None,
            None,
            Some(4),
            Some(7),
            Some(13),
        ],
        vec![Some(1), None, Some(2), None, Some(0), Some(3)],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{:?}", Solution::max_ancestor_diff(root));
    }
}
