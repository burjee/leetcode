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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root == sub_root {
            return true;
        }
        if root.is_none() {
            return false;
        }
        let mut root = root.as_ref().unwrap().borrow_mut();
        Solution::is_subtree(root.left.take(), sub_root.clone())
            || Solution::is_subtree(root.right.take(), sub_root)
    }
}

fn main() {
    let input = vec![
        (vec![3, 4, 5, 1, 2], vec![4, 1, 2]),
        (vec![3, 4, 5, 1, 2, -1, -1, -1, -1, 0], vec![4, 1, 2]),
    ];
    for (root, sub_root) in input {
        let root = get_tree_node(root);
        let sub_root = get_tree_node(sub_root);
        println!("{}", Solution::is_subtree(root, sub_root));
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
    'outer: while let Some(node) = nodes.pop_front() {
        for i in 0..2 {
            if let Some(n) = nums.next() {
                if n != -1 {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(n)));
                    if i == 0 {
                        node.borrow_mut().left = Some(Rc::clone(&new_node));
                    } else {
                        node.borrow_mut().right = Some(Rc::clone(&new_node));
                    }
                    nodes.push_back(new_node);
                }
            } else {
                break 'outer;
            }
        }
    }
    Some(root)
}
