use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node = root.unwrap();
        let node = node.borrow();
        Solution::helper(&node.left, true, 0).max(Solution::helper(&node.right, false, 0))
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, is_left: bool, count: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            return if is_left {
                Solution::helper(&node.left, true, 0).max(Solution::helper(&node.right, false, count + 1))
            } else {
                Solution::helper(&node.left, true, count + 1).max(Solution::helper(&node.right, false, 0))
            };
        }
        count
    }
}

pub fn run() {
    let input = [
        vec![
            Some(1),
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            None,
            Some(1),
        ],
        vec![
            Some(1),
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
        ],
        vec![Some(1)],
        vec![Some(1), Some(1)],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::longest_zig_zag(root));
    }
}
