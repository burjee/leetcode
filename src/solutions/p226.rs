use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&mut root);
        root
    }

    pub fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let mut left = node.left.take();
            let mut right = node.right.take();
            Solution::helper(&mut left);
            Solution::helper(&mut right);
            node.left = right;
            node.right = left;
        }
    }
}

pub fn run() {
    let input = [
        vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ],
        vec![Some(2), Some(1), Some(3)],
        vec![
            Some(2),
            Some(1),
            Some(3),
            Some(6),
            Some(8),
            Some(5),
            Some(7),
            Some(4),
            Some(9),
        ],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        TreeNode::print(Solution::invert_tree(root));
    }
}
