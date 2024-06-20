use crate::utils::tree::TreeNode;
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
        (
            vec![
                Some(10),
                Some(5),
                Some(15),
                Some(3),
                Some(7),
                None,
                Some(18),
            ],
            7,
            15,
        ),
        (
            vec![
                Some(10),
                Some(5),
                Some(15),
                Some(3),
                Some(7),
                Some(13),
                Some(18),
                Some(1),
                None,
                Some(6),
            ],
            6,
            10,
        ),
    ];

    for (nums, low, high) in input {
        let root = TreeNode::from_vec(nums);
        println!("{:?}", Solution::range_sum_bst(root, low, high));
    }
}

/* stack
use crate::utils::tree::TreeNode;
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

 */
