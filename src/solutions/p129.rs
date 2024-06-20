use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root, 0)
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let val = val * 10 + node.val;
            match (node.left.is_some(), node.right.is_some()) {
                (true, true) => {
                    return Solution::helper(&node.left, val) + Solution::helper(&node.right, val);
                }
                (true, false) => {
                    return Solution::helper(&node.left, val);
                }
                (false, true) => {
                    return Solution::helper(&node.right, val);
                }
                (false, false) => {
                    return val;
                }
            }
        }
        val
    }
}

pub fn run() {
    let input = [
        vec![Some(1)],
        vec![Some(1), Some(2), Some(3)],
        vec![Some(4), Some(9), Some(0), Some(5), Some(1)],
        vec![
            Some(4),
            Some(9),
            Some(0),
            Some(5),
            Some(1),
            Some(3),
            Some(2),
        ],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::sum_numbers(root));
    }
}
