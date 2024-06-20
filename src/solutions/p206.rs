use crate::utils::list_node::ListNode;

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
    let input = [
        vec![1, 2, 3, 4, 5],
        vec![1, 2],
        vec![1],
        vec![],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    ];

    for nums in input {
        let head = ListNode::from_vec(nums);
        ListNode::print(Solution::reverse_list(head));
    }
}
