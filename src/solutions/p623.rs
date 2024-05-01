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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }
        Solution::helper(root.clone(), val, depth);
        root
    }

    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if depth == 2 {
                let mut new_left = TreeNode::new(val);
                let mut new_right = TreeNode::new(val);
                new_left.left = node.left.take();
                new_right.right = node.right.take();
                node.left = Some(Rc::new(RefCell::new(new_left)));
                node.right = Some(Rc::new(RefCell::new(new_right)));
            } else {
                Solution::helper(node.left.clone(), val, depth - 1);
                Solution::helper(node.right.clone(), val, depth - 1);
            }
        }
    }
}

pub fn run() {
    let input = [
        (
            vec![Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)],
            1,
            2,
        ),
        (vec![Some(4), Some(2), None, Some(3), Some(1)], 1, 3),
    ];
    for (root, val, depth) in input {
        let root = get_tree_node(root);
        print_tree_node(Solution::add_one_row(root, val, depth));
    }
}

fn get_tree_node(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }
    use std::collections::VecDeque;

    let mut nums = nums.into_iter();
    let root = Rc::new(RefCell::new(TreeNode::new(nums.next().unwrap().unwrap())));
    let mut nodes = VecDeque::new();
    nodes.push_back(Rc::clone(&root));
    'outer: while let Some(node) = nodes.pop_front() {
        for i in 0..2 {
            if let Some(n) = nums.next() {
                if n.is_some() {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(n.unwrap())));
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
