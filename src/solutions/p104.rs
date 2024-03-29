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
use std::cmp;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut node = root.as_ref().unwrap().borrow_mut();
        let left = Solution::max_depth(node.left.take()) + 1;
        let right = Solution::max_depth(node.right.take()) + 1;
        cmp::max(left, right)
    }
}

pub fn run() {
    let roots = vec![
        vec![3, 9, 20, -1, -1, 15, 7],
        vec![3, 1, 5, 0, 2, 4, 6],
        vec![1],
        vec![],
        vec![2, -1, 2],
        vec![5, 1, 4, -1, -1, 3, 6],
        vec![5, 1, 4, -1, -1, -1, 2],
        vec![2, -1, 3, -1, 4, -1, 5],
        vec![5, 3, 8, 2, 4, 7, 10, 1, -1, -1, -1, 6, -1, 9, 11],
    ];

    for root in roots {
        println!("{:?}", Solution::max_depth(get_tree_node(root)));
    }
}

fn get_tree_node(numbers: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if numbers.len() == 0 {
        return None;
    }

    let mut nums = numbers.into_iter();
    let root = Rc::new(RefCell::new(TreeNode::new(nums.next().unwrap())));
    let mut nodes = vec![Rc::clone(&root)];
    let mut temps = vec![];
    'outer: loop {
        for node in nodes {
            if let Some(n) = nums.next() {
                if n != -1 {
                    let left = Rc::new(RefCell::new(TreeNode::new(n)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    temps.push(Rc::clone(&left));
                }
            } else {
                break 'outer;
            }
            if let Some(n) = nums.next() {
                if n != -1 {
                    let right = Rc::new(RefCell::new(TreeNode::new(n)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    temps.push(Rc::clone(&right));
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
