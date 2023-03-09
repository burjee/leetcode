// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

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

fn main() {
    let input = [
        (
            vec![3, 5, 1, 6, 2, 9, 8, -1, -1, 7, 4],
            vec![3, 5, 1, 6, 7, 4, 2, -1, -1, -1, -1, -1, -1, 9, 8],
        ),
        (vec![1, 2, 3], vec![1, 3, 2]),
    ];
    for (nums1, nums2) in input {
        let root1 = get_tree_node(nums1);
        let root2 = get_tree_node(nums2);
        println!("{:?}", Solution::leaf_similar(root1, root2));
    }
}

fn get_tree_node(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
    let mut nodes = vec![root.clone()];
    let mut temps = vec![];
    let mut nums = nums.into_iter();
    nums.next();
    'outer: loop {
        for node in nodes {
            if let Some(n) = nums.next() {
                if n != -1 {
                    let left = Rc::new(RefCell::new(TreeNode::new(n)));
                    node.borrow_mut().left = Some(left.clone());
                    temps.push(left);
                }
            } else {
                break 'outer;
            }
            if let Some(n) = nums.next() {
                if n != -1 {
                    let right = Rc::new(RefCell::new(TreeNode::new(n)));
                    node.borrow_mut().right = Some(right.clone());
                    temps.push(right);
                }
            } else {
                break 'outer;
            }
        }
        nodes = temps;
        temps = vec![];
    }
    Some(root)
}
