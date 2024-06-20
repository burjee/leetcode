use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leaf1 = vec![];
        let mut leaf2 = vec![];
        Solution::helper(root1, &mut leaf1);
        Solution::helper(root2, &mut leaf2);

        leaf1 == leaf2
    }

    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, leaf: &mut Vec<i32>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if node.left.is_none() && node.right.is_none() {
                leaf.push(node.val);
            }
            if node.left.is_some() {
                Solution::helper(node.left.take(), leaf)
            }
            if node.right.is_some() {
                Solution::helper(node.right.take(), leaf)
            }
        }
    }
}

pub fn run() {
    let input = [
        (
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(9),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ],
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(7),
                Some(4),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(9),
                Some(8),
            ],
        ),
        (
            vec![Some(1), Some(2), Some(3)],
            vec![Some(1), Some(3), Some(2)],
        ),
    ];
    for (nums1, nums2) in input {
        let root1 = TreeNode::from_vec(nums1);
        let root2 = TreeNode::from_vec(nums2);
        println!("{:?}", Solution::leaf_similar(root1, root2));
    }
}
