use crate::utils::list_node::ListNode;

pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = ListNode::new(-1);
        let mut point = &mut head;
        let mut temp = 0;
        while l1.is_some() || l2.is_some() || temp > 0 {
            if let Some(n1) = l1 {
                temp += n1.val;
                l1 = n1.next;
            }
            if let Some(n2) = l2 {
                temp += n2.val;
                l2 = n2.next;
            }
            point.next = Some(Box::new(ListNode::new(temp % 10)));
            temp = temp / 10;
            point = point.next.as_mut().unwrap();
            if temp == 0 && (l1.is_none() || l2.is_none()) {
                if l1.is_some() {
                    point.next = l1;
                    break;
                }
                if l2.is_some() {
                    point.next = l2;
                    break;
                }
            }
        }
        head.next
    }
}

pub fn run() {
    let input = [
        (vec![2, 4, 3], vec![5, 6, 4]),
        (vec![0], vec![0]),
        (vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9]),
        (vec![9, 9, 9, 9, 9, 9, 9], vec![1]),
        (vec![1, 2, 3], vec![1, 1, 1, 1, 1, 1, 1, 1, 1]),
    ];

    for input in input {
        let l1 = ListNode::from_vec(input.0);
        let l2 = ListNode::from_vec(input.1);
        ListNode::print(Solution::add_two_numbers(l1, l2));
    }
}
