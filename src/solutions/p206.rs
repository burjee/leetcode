// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn _new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution {}
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut next = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = next;
            next = Some(node);
        }
        next
    }
}

pub fn run() {
    let input = vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 2],
        vec![1],
        vec![],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    ];
    for head in input {
        let output = Solution::reverse_list(get_list_node(head));
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
