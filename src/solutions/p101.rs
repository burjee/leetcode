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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let tree = root.unwrap();
        let l = &tree.borrow().left;
        let r = &tree.borrow().right;
        Solution::symmetric_support(l, r)
    }

    pub fn symmetric_support(
        l: &Option<Rc<RefCell<TreeNode>>>,
        r: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (l, r) {
            (Some(n), Some(m)) => {
                let n = n.borrow();
                let m = m.borrow();
                if n.val != m.val {
                    return false;
                }
                Solution::symmetric_support(&n.left, &m.right)
                    && Solution::symmetric_support(&n.right, &m.left)
            }
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

pub fn run() {
    let roots = vec![vec![1, 2, 2, 3, 4, 4, 3], vec![1, 2, 2, -1, 3, -1, 3]];

    for root in roots {
        println!("{}", Solution::is_symmetric(get_tree_node(root)));
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
