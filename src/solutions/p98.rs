use crate::utils::tree::TreeNode;
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
    let input = [
        vec![
            Some(2147483644),
            Some(-2147483648),
            Some(2147483646),
            None,
            None,
            Some(2147483645),
            Some(2147483647),
        ],
        vec![Some(-2147483648), None, Some(2147483647)],
        vec![
            Some(3),
            Some(1),
            Some(5),
            Some(0),
            Some(2),
            Some(4),
            Some(6),
        ],
        vec![Some(2), Some(2), Some(2)],
        vec![Some(2), Some(1), Some(3)],
        vec![Some(2), Some(5), Some(3)],
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
            Some(7),
            Some(6),
            None,
            Some(9),
            Some(11),
        ],
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
            Some(1),
            None,
            Some(9),
            Some(11),
        ],
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
        println!("{}", Solution::is_valid_bst(root));
    }
}
