use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

    pub fn from_vec(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mut nums = nums.into_iter();
        let root = Rc::new(RefCell::new(TreeNode::new(nums.next().unwrap().unwrap())));
        let mut nodes = VecDeque::new();
        nodes.push_back(Rc::clone(&root));
        'outer: while let Some(node) = nodes.pop_front() {
            for i in 0..2 {
                if let Some(num) = nums.next() {
                    if let Some(val) = num {
                        let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
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

    pub fn print(root: Option<Rc<RefCell<TreeNode>>>) {
        let mut nodes = VecDeque::new();
        nodes.push_back(root);
        while let Some(node) = nodes.pop_front() {
            if let Some(node) = node {
                print!("{} ", node.borrow().val);
                nodes.push_back(node.borrow_mut().left.take());
                nodes.push_back(node.borrow_mut().right.take());
            }
        }
        println!();
    }
}
