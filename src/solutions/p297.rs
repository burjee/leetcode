use crate::utils::tree::TreeNode;
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
pub fn run() {
    let input = [
        vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)],
        vec![],
        vec![Some(1)],
        vec![Some(1), Some(2)],
        vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
        ],
    ];

    let codec = Codec::new();
    for nums in input {
        let root = TreeNode::from_vec(nums);
        let data = codec.serialize(root);
        println!("{}", data);
        let root = codec.deserialize(data);
        TreeNode::print(root);
    }
}
