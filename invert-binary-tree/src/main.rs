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
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&mut root);
        root
    }

    pub fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let mut left = node.left.take();
            let mut right = node.right.take();
            Solution::helper(&mut left);
            Solution::helper(&mut right);
            node.left = right;
            node.right = left;
        }
    }
}

fn main() {
    let input = vec![
        vec![4, 2, 7, 1, 3, 6, 9],
        vec![2, 1, 3],
        vec![2, 1, 3, 6, 8, 5, 7, 4, 9],
    ];
    for num in input {
        let root = get_tree_node(num);
        let head = Solution::invert_tree(root);
        print_tree_node(head);
    }
}

fn print_tree_node(root: Option<Rc<RefCell<TreeNode>>>) {
    use std::collections::VecDeque;

    let mut nodes = VecDeque::new();
    nodes.push_back(root);
    while let Some(tree) = nodes.pop_front() {
        if let Some(node) = tree {
            print!("{} ", node.borrow().val);
            nodes.push_back(node.borrow_mut().left.take());
            nodes.push_back(node.borrow_mut().right.take());
        }
    }
    println!();
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
