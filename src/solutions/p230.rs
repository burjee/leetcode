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
    let input = vec![
        (vec![3, 1, 4, -1, 2], 1),
        (vec![5, 3, 6, 2, 4, -1, -1, 1], 3),
    ];
    for (nums, k) in input {
        let root = get_tree_node(nums);
        println!("{}", Solution::kth_smallest(root, k));
    }
}

fn get_tree_node(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }
    use std::collections::VecDeque;

    let mut nums = nums.into_iter();
    let root = Rc::new(RefCell::new(TreeNode::new(nums.next().unwrap())));
    let mut nodes = VecDeque::new();
    nodes.push_back(Rc::clone(&root));
    while let Some(node) = nodes.pop_front() {
        if let Some(n) = nums.next() {
            if n != -1 {
                let left = Rc::new(RefCell::new(TreeNode::new(n)));
                node.borrow_mut().left = Some(Rc::clone(&left));
                nodes.push_back(left);
            }
        } else {
            break;
        }
        if let Some(n) = nums.next() {
            if n != -1 {
                let right = Rc::new(RefCell::new(TreeNode::new(n)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                nodes.push_back(right);
            }
        } else {
            break;
        }
    }
    Some(root)
}
