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
use std::collections::HashMap;
use std::rc::Rc;

// 依序讀取inorder，按照preorder的深度(建成字典)判斷連接
// 深度差一，連接左
// 深度為負，推入堆疊
// 深度差多，開始與堆疊連接右，連接完成與深度差多的節點連接左
// 深度差多，堆疊深度若比深度差多的節點還淺，停止堆疊連接，直接與深度差多的節點連接左
// 讀取完畢若堆疊尚存，連接右，回傳堆疊底
// 堆疊消耗完畢，回傳最後使用節點
//
//     3          [] 3
//       20
//    15    17
//  4   6  9
//
//     3          [3] 4
//       20
//    15    17
//  4   6  9
//
//     3          [3] 15
//       20
//    15    17
//   /
//  4   6  9
//
//     3          [3,15] 6
//       20
//    15    17
//   /
//  4   6  9
//
//     3          [3] 20
//       20
//      /
//    15    17
//   / \
//  4   6  9
//
//     3          [3,20] 9
//       20
//      /
//    15    17
//   / \
//  4   6  9
//
//     3          [3,20] 17
//       20
//      /
//    15    17
//   / \    /
//  4   6  9
//
//     3          []
//      \
//       20
//      /  \
//    15    17
//   / \    /
//  4   6  9

struct Solution {}
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes: Vec<TreeNode> = vec![];
        let mut node = TreeNode::new(inorder[0]);
        let mut dict = HashMap::new();
        for (i, n) in preorder.into_iter().enumerate() {
            dict.insert(n, i);
        }
        for n in inorder.into_iter().skip(1) {
            let lv0 = dict.get(&node.val).unwrap();
            let lv1 = dict.get(&n).unwrap();
            if lv0 > lv1 {
                if lv0 - lv1 == 1 {
                    // 深度差一
                    let mut _node = TreeNode::new(n);
                    _node.left = Some(Rc::new(RefCell::new(node)));
                    node = _node;
                } else {
                    // 深度差多
                    let mut _node = node;
                    while let Some(n) = nodes.last() {
                        let lv2 = dict.get(&n.val).unwrap();
                        if lv2 < lv1 {
                            break;
                        }
                        let mut n = nodes.pop().unwrap();
                        n.right = Some(Rc::new(RefCell::new(_node)));
                        _node = n;
                    }
                    let mut root = TreeNode::new(n);
                    root.left = Some(Rc::new(RefCell::new(_node)));
                    node = root;
                }
            } else {
                // 深度差負
                nodes.push(node);
                node = TreeNode::new(n);
            }
        }
        if nodes.len() > 0 {
            let mut _node = node;
            for mut n in nodes.into_iter().rev() {
                n.right = Some(Rc::new(RefCell::new(_node)));
                _node = n;
            }
            return Some(Rc::new(RefCell::new(_node)));
        }
        Some(Rc::new(RefCell::new(node)))
    }
}

pub fn run() {
    let input = vec![
        (vec![3, 1, 2, 4, 5, 7, 6], vec![6, 7, 5, 4, 2, 1, 3]),
        (vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
        (vec![3, 20, 15, 4, 6, 7, 9], vec![3, 4, 15, 6, 20, 9, 7]),
        (vec![3, 9, 1, 20, 15, 7], vec![1, 9, 3, 15, 20, 7]),
        (
            vec![3, 9, 1, 4, 6, 8, 20, 15, 7],
            vec![1, 9, 6, 4, 8, 3, 15, 20, 7],
        ),
        (vec![-1], vec![-1]),
        // [3,1,null,2,null,4,null,5,null,7,null,6]
        // [3,9,20,null,null,15,7]
        // [3,null,20,15,7,4,6,9]
        // [3,9,20,1,null,15,7]
        // [3,9,20,1,4,15,7,null,null,6,8]
    ];

    for (preorder, inorder) in input {
        println!("{:?}", Solution::build_tree(preorder, inorder));
    }
}
