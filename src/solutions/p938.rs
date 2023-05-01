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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let val = node.val;
            if val >= low && val <= high {
                return val
                    + Solution::range_sum_bst(node.left.take(), low, high)
                    + Solution::range_sum_bst(node.right.take(), low, high);
            } else if val > high {
                return Solution::range_sum_bst(node.left.take(), low, high);
            } else if val < low {
                return Solution::range_sum_bst(node.right.take(), low, high);
            }
        }
        0
    }
}

pub fn run() {
    let input = [
        (vec![10, 5, 15, 3, 7, -1, 18], 7, 15),
        (vec![10, 5, 15, 3, 7, 13, 18, 1, -1, 6], 6, 10),
    ];
    for (nums, low, high) in input {
        let root = get_tree_node(nums);
        println!("{:?}", Solution::range_sum_bst(root, low, high));
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

/* stack
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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut nodes = vec![root];
        let mut sum = 0;
        while let Some(node) = nodes.pop() {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                if n.val >= low && n.val <= high {
                    sum += n.val;
                    nodes.push(n.left.take());
                    nodes.push(n.right.take());
                } else if n.val > high {
                    nodes.push(n.left.take());
                } else if n.val < low {
                    nodes.push(n.right.take());
                }
            }
        }
        sum
    }
}

pub fn run() {
    let input = [
        (vec![10, 5, 15, 3, 7, -1, 18], 7, 15),
        (vec![10, 5, 15, 3, 7, 13, 18, 1, -1, 6], 6, 10),
    ];
    for (nums, low, high) in input {
        let root = get_tree_node(nums);
        println!("{:?}", Solution::range_sum_bst(root, low, high));
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
 */
