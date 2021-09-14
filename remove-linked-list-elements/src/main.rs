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

struct Solution {}
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut hair = ListNode::new(-1);
        let mut point = &mut hair;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val != val {
                point.next = Some(node);
                point = point.next.as_mut().unwrap();
            }
        }
        hair.next
    }
}

fn main() {
    let input = vec![
        (vec![1, 2, 6, 3, 4, 5, 6], 6),
        (vec![], 1),
        (vec![7, 7, 7, 7], 7),
        (vec![5, 7, 6, 7], 7),
        (vec![7, 7, 6, 9, 9], 7),
    ];
    for (head, val) in input {
        let head = get_list_node(head);
        let output = Solution::remove_elements(head, val);
        print_list_node(output);
    }
}

fn get_list_node(mut numbers: Vec<i32>) -> Option<Box<ListNode>> {
    numbers.reverse();
    let mut next = None;
    for n in numbers {
        next = Some(Box::new(ListNode { val: n, next }));
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
