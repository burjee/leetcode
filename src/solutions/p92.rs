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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }
        let mut list = head;
        let mut head = ListNode::new(-1);
        let mut point = &mut head;
        let mut i = 0;
        let mut v = vec![];
        while let Some(node) = list {
            i += 1;
            if i == right {
                v.push(node.val);
                while let Some(n) = v.pop() {
                    let new_node = ListNode::new(n);
                    point.next = Some(Box::new(new_node));
                    point = point.next.as_mut().unwrap();
                }
                point.next = node.next;
                break;
            } else if i >= left {
                v.push(node.val);
            } else {
                let new_node = ListNode::new(node.val);
                point.next = Some(Box::new(new_node));
                point = point.next.as_mut().unwrap();
            }
            list = node.next;
        }
        head.next
    }
}

pub fn run() {
    let inputs = vec![
        (vec![1, 2, 3, 4, 5], 2, 4),
        (vec![5], 1, 1),
        (vec![1, 2, 3, 4, 5], 1, 5),
        (vec![1, 2, 3, 4, 5], 2, 3),
        (vec![1, 2, 3, 4, 5], 1, 4),
    ];
    for input in inputs {
        let mut output = Solution::reverse_between(get_list_node(input.0), input.1, input.2);
        while let Some(node) = output {
            print!("{} ", node.val);
            output = node.next;
        }
        println!();
    }
}

fn get_list_node(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(-1);
    let mut point = &mut head;
    for n in numbers {
        let node = Some(Box::new(ListNode::new(n)));
        point.next = node;
        point = point.next.as_mut().unwrap();
    }
    head.next
}
