// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
    let input = vec![
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5, 6],
        vec![1],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    ];

    for list in input {
        let mut head = get_list_node(list);
        Solution::reorder_list(&mut head);
        print_list_node(head);
    }
}

fn get_list_node(mut numbers: Vec<i32>) -> Option<Box<ListNode>> {
    numbers.reverse();
    let mut next = None;
    for n in numbers {
        let node = Some(Box::new(ListNode { val: n, next: next }));
        next = node;
    }
    next
}

fn print_list_node(mut head: Option<Box<ListNode>>) {
    while let Some(node) = head {
        print!("{} ", node.val);
        head = node.next;
    }
    println!();
}
