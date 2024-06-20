use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root, false)
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let mut sum = 0;
            if node.left.is_some() {
                sum += Solution::helper(&node.left, true);
            }
            if node.right.is_some() {
                sum += Solution::helper(&node.right, false);
            }
            if node.left.is_none() && node.right.is_none() && is_left {
                sum += node.val;
            }
            return sum;
        }
        0
    }
}

pub fn run() {
    let input = [
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        vec![Some(1)],
        vec![Some(1), Some(2)],
        vec![Some(1), Some(2), None, Some(3)],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::sum_of_left_leaves(root));
    }
}
