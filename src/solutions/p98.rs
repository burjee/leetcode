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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.as_ref().unwrap();
        Solution::is_left_valid(&node.borrow().left, -2147483648, node.borrow().val, true)
            && Solution::is_right_valid(&node.borrow().right, node.borrow().val, 2147483647, true)
    }

    pub fn is_left_valid(
        tree: &Option<Rc<RefCell<TreeNode>>>,
        min: i32,
        max: i32,
        is_rim: bool,
    ) -> bool {
        if let Some(node) = tree {
            if node.borrow().val >= max || (!is_rim && node.borrow().val <= min) {
                return false;
            }
            return Solution::is_left_valid(&node.borrow().left, min, node.borrow().val, is_rim)
                && Solution::is_right_valid(&node.borrow().right, node.borrow().val, max, false);
        }
        true
    }

    pub fn is_right_valid(
        tree: &Option<Rc<RefCell<TreeNode>>>,
        min: i32,
        max: i32,
        is_rim: bool,
    ) -> bool {
        if let Some(node) = tree {
            if node.borrow().val <= min || (!is_rim && node.borrow().val >= max) {
                return false;
            }
            return Solution::is_left_valid(&node.borrow().left, min, node.borrow().val, false)
                && Solution::is_right_valid(&node.borrow().right, node.borrow().val, max, is_rim);
        }
        true
    }
}

pub fn run() {
    let roots = vec![
        vec![
            2147483644,
            -2147483648,
            2147483646,
            -1,
            -1,
            2147483645,
            2147483647,
        ],
        vec![-2147483648, -1, 2147483647],
        vec![3, 1, 5, 0, 2, 4, 6],
        vec![2, 2, 2],
        vec![2, 1, 3],
        vec![2, 5, 3],
        vec![5, 1, 4, -1, -1, 3, 6],
        vec![5, 1, 4, -1, -1, -1, 2],
        vec![2, -1, 3, -1, 4, -1, 5],
        vec![5, 3, 8, 2, 4, 7, 10, 1, -1, -1, -1, 6, -1, 9, 11],
        vec![5, 3, 8, 2, 4, 7, 10, 1, -1, -1, 7, 6, -1, 9, 11],
        vec![5, 3, 8, 2, 4, 7, 10, 1, -1, -1, -1, 1, -1, 9, 11],
        vec![5, 3, 8, 2, 4, 7, 10, 1, -1, -1, -1, 6, -1, 9, 11],
    ];

    for root in roots {
        println!("{}", Solution::is_valid_bst(get_tree_node(root)));
    }
}

fn get_tree_node(numbers: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
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
