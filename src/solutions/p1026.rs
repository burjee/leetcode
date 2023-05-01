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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root, i32::MAX, i32::MIN)
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let min = min.min(node.val);
            let max = max.max(node.val);
            Solution::helper(&node.left, min, max).max(Solution::helper(&node.right, min, max))
        } else {
            max - min
        }
    }
}

pub fn run() {
    let input = [
        vec![8, 3, 10, 1, 6, -1, 14, -1, -1, 4, 7, 13],
        vec![1, -1, 2, -1, 0, 3],
    ];
    for nums in input {
        let root = get_tree_node(nums);
        println!("{:?}", Solution::max_ancestor_diff(root));
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
