use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn delete_node(mut root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::find_key(&mut root, key);
        root
    }

    pub fn find_key(root: &mut Option<Rc<RefCell<TreeNode>>>, key: i32) {
        if let Some(node) = root {
            if key < node.borrow().val {
                Solution::find_key(&mut node.borrow_mut().left, key);
            } else if key > node.borrow().val {
                Solution::find_key(&mut node.borrow_mut().right, key);
            } else {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                match (left, right) {
                    (None, None) => *root = None,
                    (tree_node, None) | (None, tree_node) => *root = tree_node,
                    (mut left, right) => {
                        let new_node = Solution::find_max(&mut left);
                        new_node.borrow_mut().left = left;
                        new_node.borrow_mut().right = right;
                        root.replace(new_node);
                    }
                }
            }
        }
    }

    pub fn find_max(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
        let node = root.as_ref().unwrap();
        if node.borrow().right.is_none() {
            let new_node = root.take().unwrap();
            *root = new_node.borrow_mut().left.take();
            new_node
        } else {
            Solution::find_max(&mut node.borrow_mut().right)
        }
    }
}

pub fn run() {
    let input = [
        (vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 2),
        (vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 3),
        (vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 0),
        (vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 5),
        (vec![], 0),
    ];

    for (nums, key) in input {
        let root = TreeNode::from_vec(nums);
        TreeNode::print(Solution::delete_node(root, key));
    }
}
