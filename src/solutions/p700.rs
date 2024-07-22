use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            return if val == node.borrow().val {
                Some(node)
            } else if val < node.borrow().val {
                Solution::search_bst(node.borrow_mut().left.take(), val)
            } else {
                Solution::search_bst(node.borrow_mut().right.take(), val)
            };
        }
        None
    }
}

pub fn run() {
    let input = [
        (vec![Some(4), Some(2), Some(7), Some(1), Some(3)], 2),
        (vec![Some(4), Some(2), Some(7), Some(1), Some(3)], 5),
    ];

    for (nums, val) in input {
        let root = TreeNode::from_vec(nums);
        TreeNode::print(Solution::search_bst(root, val));
    }
}
