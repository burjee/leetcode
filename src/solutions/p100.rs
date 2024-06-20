use crate::utils::tree::TreeNode;
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

pub fn run() {
    let input = [
        (
            vec![Some(1), Some(2), Some(3)],
            vec![Some(1), Some(2), Some(3)],
        ),
        (vec![Some(1), Some(2)], vec![Some(1), None, Some(2)]),
        (
            vec![Some(1), Some(2), Some(1)],
            vec![Some(1), Some(1), Some(2)],
        ),
        (
            vec![Some(2), None, Some(3), None, Some(4), None, Some(5)],
            vec![Some(2), None, Some(3), None, Some(4), None, Some(5)],
        ),
        (
            vec![Some(2), None, Some(4), None, Some(5)],
            vec![Some(2), None, Some(3), None, Some(4), None, Some(5)],
        ),
        (
            vec![Some(2), None, Some(3), None, Some(4), None, Some(5)],
            vec![Some(2), None, Some(4), None, Some(5)],
        ),
    ];

    for nums in input {
        let p = TreeNode::from_vec(nums.0);
        let q = TreeNode::from_vec(nums.1);
        println!("{}", Solution::is_same_tree(p, q));
    }
}
