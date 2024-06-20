use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }
        Solution::helper(root.clone(), val, depth);
        root
    }

    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if depth == 2 {
                let mut new_left = TreeNode::new(val);
                let mut new_right = TreeNode::new(val);
                new_left.left = node.left.take();
                new_right.right = node.right.take();
                node.left = Some(Rc::new(RefCell::new(new_left)));
                node.right = Some(Rc::new(RefCell::new(new_right)));
            } else {
                Solution::helper(node.left.clone(), val, depth - 1);
                Solution::helper(node.right.clone(), val, depth - 1);
            }
        }
    }
}

pub fn run() {
    let input = [
        (
            vec![Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)],
            1,
            2,
        ),
        (vec![Some(4), Some(2), None, Some(3), Some(1)], 1, 3),
    ];

    for (nums, val, depth) in input {
        let root = TreeNode::from_vec(nums);
        TreeNode::print(Solution::add_one_row(root, val, depth));
    }
}
