use crate::utils::list_node::ListNode;

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

pub fn run() {
    let input = [
        (vec![1, 2, 6, 3, 4, 5, 6], 6),
        (vec![], 1),
        (vec![7, 7, 7, 7], 7),
        (vec![5, 7, 6, 7], 7),
        (vec![7, 7, 6, 9, 9], 7),
    ];

    for (nums, val) in input {
        let head = ListNode::from_vec(nums);
        ListNode::print(Solution::remove_elements(head, val));
    }
}
