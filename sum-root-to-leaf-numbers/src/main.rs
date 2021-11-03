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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root, 0)
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let val = val * 10 + node.val;
            match (node.left.is_some(), node.right.is_some()) {
                (true, true) => {
                    return Solution::helper(&node.left, val) + Solution::helper(&node.right, val);
                }
                (true, false) => {
                    return Solution::helper(&node.left, val);
                }
                (false, true) => {
                    return Solution::helper(&node.right, val);
                }
                (false, false) => {
                    return val;
                }
            }
        }
        val
    }
}

fn main() {
    let input = vec![
        vec![1],
        vec![1, 2, 3],
        vec![4, 9, 0, 5, 1],
        vec![4, 9, 0, 5, 1, 3, 2],
    ];
    for nums in input {
        let root = get_tree_node(nums);
        println!("{}", Solution::sum_numbers(root));
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
