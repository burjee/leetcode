use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Solution::helper1(&root, &sub_root)
    }

    pub fn helper1(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        if Solution::helper2(&root, &sub_root) {
            return true;
        }
        let root = root.as_ref().unwrap().borrow();
        Solution::helper1(&root.left, sub_root) || Solution::helper1(&root.right, sub_root)
    }

    pub fn helper2(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root == sub_root {
            if root.is_none() {
                return true;
            }
            let root_borrow = root.as_ref().unwrap().borrow();
            let sub_borrow = sub_root.as_ref().unwrap().borrow();
            return Solution::helper2(&root_borrow.left, &sub_borrow.left)
                && Solution::helper2(&root_borrow.right, &sub_borrow.right);
        }
        false
    }
}

pub fn run() {
    let input = [
        (
            vec![Some(3), Some(4), Some(5), Some(1), Some(2)],
            vec![Some(4), Some(1), Some(2)],
        ),
        (
            vec![
                Some(3),
                Some(4),
                Some(5),
                Some(1),
                Some(2),
                None,
                None,
                None,
                None,
                Some(0),
            ],
            vec![Some(4), Some(1), Some(2)],
        ),
    ];

    for (nums1, nums2) in input {
        let root = TreeNode::from_vec(nums1);
        let sub_root = TreeNode::from_vec(nums2);
        println!("{}", Solution::is_subtree(root, sub_root));
    }
}

/* tree
use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root == sub_root {
            return true;
        }
        if root.is_none() {
            return false;
        }
        let mut root = root.as_ref().unwrap().borrow_mut();
        Solution::is_subtree(root.left.take(), sub_root.clone())
            || Solution::is_subtree(root.right.take(), sub_root)
    }
}

 */
