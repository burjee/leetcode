use crate::utils::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut c = 0;
        let mut p = &head;
        while let Some(node) = p {
            p = &node.next;
            c += 1;
        }

        if c == 1 {
            return None;
        }

        c /= 2;
        let mut p = head.as_mut();
        for _ in 0..c - 1 {
            p = p.unwrap().next.as_mut();
        }
        let mut node = p.unwrap();
        node.next = node.next.take().unwrap().next;

        head
    }
}

pub fn run() {
    let input = [vec![1, 3, 4, 7, 1, 2, 6], vec![1, 2, 3, 4], vec![2, 1]];

    for nums in input {
        let head = ListNode::from_vec(nums);
        ListNode::print(Solution::delete_middle(head));
    }
}
