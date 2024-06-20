use crate::utils::list_node::ListNode;

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
    let input = [
        (vec![1, 2, 3, 4, 5], 2, 4),
        (vec![5], 1, 1),
        (vec![1, 2, 3, 4, 5], 1, 5),
        (vec![1, 2, 3, 4, 5], 2, 3),
        (vec![1, 2, 3, 4, 5], 1, 4),
    ];

    for (nums, left, right) in input {
        let head = ListNode::from_vec(nums);
        ListNode::print(Solution::reverse_between(head, left, right));
    }
}
