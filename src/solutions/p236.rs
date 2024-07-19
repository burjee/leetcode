use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        Solution::helper(&root, p.unwrap().borrow().val, q.unwrap().borrow().val, &mut ans);
        ans
    }

    pub fn helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
        ans: &mut Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        if ans.is_none() {
            if let Some(node) = root {
                let node = node.borrow();
                let mut n = if node.val == p || node.val == q { 1 } else { 0 };
                n += Solution::helper(&node.left, p, q, ans) + Solution::helper(&node.right, p, q, ans);
                if n == 2 {
                    *ans = Some(Rc::new(RefCell::new(TreeNode::new(node.val))));
                    n = 0
                }
                return n;
            }
        }
        0
    }

    // pub fn lowest_common_ancestor(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     p: Option<Rc<RefCell<TreeNode>>>,
    //     q: Option<Rc<RefCell<TreeNode>>>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     Solution::helper(&root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    // }

    // pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
    //     if let Some(node) = root {
    //         if node.borrow().val == p || node.borrow().val == q {
    //             return Some(Rc::new(RefCell::new(TreeNode::new(node.borrow().val))));
    //         }
    //         let left = Solution::helper(&node.borrow().left, p, q);
    //         let right = Solution::helper(&node.borrow().right, p, q);
    //         match (left, right) {
    //             (Some(_), Some(_)) => return Some(Rc::new(RefCell::new(TreeNode::new(node.borrow().val)))),
    //             (Some(node), None) | (None, Some(node)) => {
    //                 return Some(Rc::new(RefCell::new(TreeNode::new(node.borrow().val))))
    //             }
    //             _ => return None,
    //         }
    //     }
    //     None
    // }
}

pub fn run() {
    let input = [
        (
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ],
            vec![Some(5)],
            vec![Some(1)],
        ),
        (
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ],
            vec![Some(5)],
            vec![Some(4)],
        ),
        (vec![Some(1), Some(2)], vec![Some(1)], vec![Some(2)]),
    ];

    for (nums, p, q) in input {
        let root = TreeNode::from_vec(nums);
        let p = TreeNode::from_vec(p);
        let q = TreeNode::from_vec(q);
        TreeNode::print(Solution::lowest_common_ancestor(root, p, q));
    }
}
