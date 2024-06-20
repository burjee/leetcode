use crate::utils::tree::TreeNode;
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
    let input = [
        vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ],
        vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)],
    ];

    for nums in input {
        let root = TreeNode::from_vec(nums);
        println!("{}", Solution::is_symmetric(root));
    }
}
