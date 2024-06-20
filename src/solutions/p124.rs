use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = i32::MIN;
        Solution::helper(&root, &mut max);
        max
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Solution::helper(&node.left, max);
            let right = Solution::helper(&node.right, max);
            let mut sum = cmp::max(left, right) + node.val;
            sum = cmp::max(sum, node.val);
            *max = cmp::max(*max, node.val + left + right);
            *max = cmp::max(*max, sum);
            return sum;
        }
        0
    }
}

pub fn run() {
    let input = [
        vec![Some(1), Some(2), Some(3)],
        vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)],
        vec![
            Some(-10),
            Some(9),
            Some(20),
            Some(100),
            None,
            Some(15),
            Some(7),
        ],
        vec![
            Some(100),
            Some(9),
            Some(20),
            Some(-100),
            None,
            Some(15),
            Some(7),
        ],
        vec![Some(-3)],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::max_path_sum(root));
    }
}
