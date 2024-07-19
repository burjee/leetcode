use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root, i32::MIN)
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, mut max: i32) -> i32 {
        let mut n = 0;
        if let Some(node) = root {
            let node = node.borrow();
            if node.val >= max {
                n += 1;
                max = node.val
            }
            n += Solution::helper(&node.left, max) + Solution::helper(&node.right, max)
        }
        n
    }
}

pub fn run() {
    let input = [
        vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)],
        vec![Some(3), Some(3), None, Some(4), Some(2)],
        vec![Some(9), None, Some(3), Some(6)],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::good_nodes(root));
    }
}
