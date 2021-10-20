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

struct Codec {}
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, s: &mut String) {
            if let Some(node) = root {
                let node = node.borrow();
                *s += &format!("{},", node.val);
                helper(&node.left, s);
                helper(&node.right, s);
            } else {
                *s += "n,";
            }
        }

        let mut s = String::new();
        helper(&root, &mut s);
        s.pop();
        s
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(chars: &mut std::str::Chars) -> Option<Rc<RefCell<TreeNode>>> {
            let mut val = 0;
            let mut sign = 1;
            while let Some(c) = chars.next() {
                match c {
                    'n' => {
                        chars.next();
                        return None;
                    }
                    ',' => {
                        let mut node = TreeNode::new(val * sign);
                        node.left = helper(chars);
                        node.right = helper(chars);
                        return Some(Rc::new(RefCell::new(node)));
                    }
                    '-' => {
                        sign = -1;
                    }
                    _ => {
                        let n = (c as u8 - b'0') as i32;
                        val = val * 10 + n;
                    }
                }
            }
            None
        }

        helper(&mut data.chars())
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
fn main() {
    let input = vec![
        vec![1, 2, 3, -1, -1, 4, 5],
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    ];
    let codec = Codec::new();
    for nums in input {
        let root = get_tree_node(nums);
        let data = codec.serialize(root);
        println!("{}", data);
        let root = codec.deserialize(data);
        print_tree_node(root);
    }
}

fn print_tree_node(root: Option<Rc<RefCell<TreeNode>>>) {
    use std::collections::VecDeque;
    let mut nodes = VecDeque::new();
    nodes.push_back(root);
    while let Some(tree) = nodes.pop_front() {
        if let Some(node) = tree {
            print!("{} ", node.borrow().val);
            nodes.push_back(node.borrow_mut().left.take());
            nodes.push_back(node.borrow_mut().right.take());
        }
    }
    println!();
}

fn get_tree_node(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }
    use std::collections::VecDeque;

    let mut nums = nums.into_iter();
    let root = Rc::new(RefCell::new(TreeNode::new(nums.next().unwrap())));
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
                    nodes.push_back(new_node);
                }
            } else {
                break 'outer;
            }
        }
    }
    Some(root)
}
