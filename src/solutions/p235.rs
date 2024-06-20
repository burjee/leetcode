use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut p: Option<Rc<RefCell<TreeNode>>>,
        mut q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut n = p.as_ref().unwrap().borrow().val;
        let mut m = q.as_ref().unwrap().borrow().val;
        if n > m {
            mem::swap(&mut n, &mut m);
            mem::swap(&mut p, &mut q);
        }

        Solution::helper(&root, &p, &q, n, m)
    }

    pub fn helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
        n: i32,
        m: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == p || root == q {
            return root.clone();
        }

        let node = root.as_ref().unwrap();
        let bor = node.borrow();
        if n < bor.val && bor.val < m {
            return Some(node.clone());
        }
        if n > bor.val {
            return Solution::helper(&bor.right, p, q, n, m);
        } else {
            return Solution::helper(&bor.left, p, q, n, m);
        }
    }
}

pub fn run() {
    let input = [
        (vec![6, 2, 8, 0, 4, 7, 9, -1, -1, 3, 5], 2, 8),
        (vec![6, 2, 8, 0, 4, 7, 9, -1, -1, 3, 5], 2, 4),
        (vec![2, 1], 2, 1),
    ];

    for (nums, n, m) in input {
        let (root, p, q) = get_tree_node(nums, n, m);
        print!("p={} ", p.as_ref().unwrap().borrow().val);
        print!("q={} ", q.as_ref().unwrap().borrow().val);
        let output = Solution::lowest_common_ancestor(root, p, q);
        println!("ancestor={}", output.unwrap().borrow().val);
    }
}

fn get_tree_node(
    nums: Vec<i32>,
    m: i32,
    k: i32,
) -> (
    Option<Rc<RefCell<TreeNode>>>,
    Option<Rc<RefCell<TreeNode>>>,
    Option<Rc<RefCell<TreeNode>>>,
) {
    use std::collections::VecDeque;

    let mut nums = nums.into_iter();
    let root = Rc::new(RefCell::new(TreeNode::new(nums.next().unwrap())));
    let mut p = if root.borrow().val == m {
        Some(Rc::clone(&root))
    } else {
        None
    };
    let mut q = if root.borrow().val == k {
        Some(Rc::clone(&root))
    } else {
        None
    };
    let mut nodes = VecDeque::new();
    nodes.push_back(Rc::clone(&root));
    'outer: while let Some(node) = nodes.pop_front() {
        for i in 0..2 {
            if let Some(n) = nums.next() {
                if n != -1 {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(n)));
                    if i == 0 {
                        node.borrow_mut().left = Some(Rc::clone(&new_node));
                    } else {
                        node.borrow_mut().right = Some(Rc::clone(&new_node));
                    }
                    if n == m {
                        p = Some(Rc::clone(&new_node));
                    }
                    if n == k {
                        q = Some(Rc::clone(&new_node));
                    }
                    nodes.push_back(new_node);
                }
            } else {
                break 'outer;
            }
        }
    }
    (Some(root), p, q)
}
