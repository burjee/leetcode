use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut nodes = vec![root];
        let mut temps = vec![];
        let mut output = vec![];
        let mut zigzag = false;

        while !nodes.is_empty() {
            let mut vals = vec![];
            for node in nodes {
                if let Some(n) = node {
                    let mut n = n.borrow_mut();
                    vals.push(n.val);
                    temps.push(n.left.take());
                    temps.push(n.right.take());
                }
            }
            if zigzag {
                vals.reverse();
            }
            if !vals.is_empty() {
                output.push(vals);
            }
            zigzag = !zigzag;
            nodes = temps;
            temps = vec![];
        }
        output
    }
}

pub fn run() {
    let input = [
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        vec![
            Some(3),
            Some(1),
            Some(5),
            Some(0),
            Some(2),
            Some(4),
            Some(6),
        ],
        vec![Some(1)],
        vec![],
        vec![Some(2), None, Some(2)],
        vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
        vec![Some(5), Some(1), Some(4), None, None, None, Some(2)],
        vec![Some(2), None, Some(3), None, Some(4), None, Some(5)],
        vec![
            Some(5),
            Some(3),
            Some(8),
            Some(2),
            Some(4),
            Some(7),
            Some(10),
            Some(1),
            None,
            None,
            None,
            Some(6),
            None,
            Some(9),
            Some(11),
        ],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{:?}", Solution::zigzag_level_order(root));
    }
}

/* zigzag
struct Solution {}
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut nodes = vec![root];
        let mut temps = vec![];
        let mut output = vec![];

        while !nodes.is_empty() {
            let mut vals = vec![];
            for node in nodes.into_iter().rev() {
                if let Some(n) = node {
                    let mut n = n.borrow_mut();
                    vals.push(n.val);
                    temps.push(n.left.take());
                    temps.push(n.right.take());
                }
            }
            if !vals.is_empty() {
                output.push(vals);
            }
            nodes = temps;
            temps = vec![];
            let mut vals = vec![];
            for node in nodes.into_iter().rev() {
                if let Some(n) = node {
                    let mut n = n.borrow_mut();
                    vals.push(n.val);
                    temps.push(n.right.take());
                    temps.push(n.left.take());
                }
            }
            if !vals.is_empty() {
                output.push(vals);
            }
            nodes = temps;
            temps = vec![];
        }
        output
    }
}

pub fn run() {
    let input = [
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        vec![
            Some(3),
            Some(1),
            Some(5),
            Some(0),
            Some(2),
            Some(4),
            Some(6),
        ],
        vec![Some(1)],
        vec![],
        vec![Some(2), None, Some(2)],
        vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
        vec![Some(5), Some(1), Some(4), None, None, None, Some(2)],
        vec![Some(2), None, Some(3), None, Some(4), None, Some(5)],
        vec![
            Some(5),
            Some(3),
            Some(8),
            Some(2),
            Some(4),
            Some(7),
            Some(10),
            Some(1),
            None,
            None,
            None,
            Some(6),
            None,
            Some(9),
            Some(11),
        ],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{:?}", Solution::zigzag_level_order(root));
    }
}
 */
