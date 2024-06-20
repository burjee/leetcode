use crate::utils::list_node::ListNode;
use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut nodes = VecDeque::new();
        let mut point = head.as_mut().unwrap().next.take();
        while let Some(node) = point.as_mut() {
            let next = node.next.take();
            nodes.push_back(point);
            point = next;
        }
        while nodes.len() > 0 {
            if let Some(right) = nodes.pop_back() {
                head.as_mut().unwrap().next = right;
                head = &mut head.as_mut().unwrap().next;
            };
            if let Some(left) = nodes.pop_front() {
                head.as_mut().unwrap().next = left;
                head = &mut head.as_mut().unwrap().next;
            };
        }
    }
}

pub fn run() {
    let input = [
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5, 6],
        vec![1],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    ];

    for nums in input {
        let mut head = ListNode::from_vec(nums);
        Solution::reorder_list(&mut head);
        ListNode::print(head);
    }
}
