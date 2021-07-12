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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(n), Some(m)) => {
                let mut n = n.borrow_mut();
                let mut m = m.borrow_mut();
                if n.val != m.val {
                    return false;
                }
                Solution::is_same_tree(n.left.take(), m.left.take())
                    && Solution::is_same_tree(n.right.take(), m.right.take())
            }
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

fn main() {
    let roots = vec![
        (vec![1, 2, 3], vec![1, 2, 3]),
        (vec![1, 2], vec![1, -1, 2]),
        (vec![1, 2, 1], vec![1, 1, 2]),
        (vec![2, -1, 3, -1, 4, -1, 5], vec![2, -1, 3, -1, 4, -1, 5]),
        (vec![2, -1, 4, -1, 5], vec![2, -1, 3, -1, 4, -1, 5]),
        (vec![2, -1, 3, -1, 4, -1, 5], vec![2, -1, 4, -1, 5]),
    ];

    for root in roots {
        println!(
            "{}",
            Solution::is_same_tree(get_tree_node(root.0), get_tree_node(root.1))
        );
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
