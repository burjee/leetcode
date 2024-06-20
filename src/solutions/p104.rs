use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut node = root.as_ref().unwrap().borrow_mut();
        let left = Solution::max_depth(node.left.take()) + 1;
        let right = Solution::max_depth(node.right.take()) + 1;
        cmp::max(left, right)
    }
}

pub fn run() {
    let input = [
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        vec![
            Some(3),
            Some(1),
            Some(5),
            Some(0),
            Some(2),
            Some(4),
            Some(6),
        ],
        vec![Some(1)],
        vec![],
        vec![Some(2), None, Some(2)],
        vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
        vec![Some(5), Some(1), Some(4), None, None, None, Some(2)],
        vec![Some(2), None, Some(3), None, Some(4), None, Some(5)],
        vec![
            Some(5),
            Some(3),
            Some(8),
            Some(2),
            Some(4),
            Some(7),
            Some(10),
            Some(1),
            None,
            None,
            None,
            Some(6),
            None,
            Some(9),
            Some(11),
        ],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::max_depth(root));
    }
}
