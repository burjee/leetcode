use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut nums = vec![];
        Solution::helper(&root, &mut nums);
        nums[k as usize - 1]
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Solution::helper(&node.left, nums);
            nums.push(node.val);
            Solution::helper(&node.right, nums);
        }
    }
}

pub fn run() {
    let input = [
        (vec![Some(3), Some(1), Some(4), None, Some(2)], 1),
        (
            vec![
                Some(5),
                Some(3),
                Some(6),
                Some(2),
                Some(4),
                None,
                None,
                Some(1),
            ],
            3,
        ),
    ];

    for (nums, k) in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::kth_smallest(root, k));
    }
}
