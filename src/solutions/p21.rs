use crate::utils::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(-1));
        let mut point = &mut head;
        loop {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        point.next = Some(n1.clone());
                        l1 = n1.next;
                        l2 = Some(n2);
                    } else {
                        point.next = Some(n2.clone());
                        l2 = n2.next;
                        l1 = Some(n1);
                    }
                    point = point.next.as_mut().unwrap();
                }
                (Some(n1), None) => {
                    point.next = Some(n1);
                    break;
                }
                (None, Some(n2)) => {
                    point.next = Some(n2);
                    break;
                }
                (None, None) => break,
            }
        }
        head.next
    }
}

pub fn run() {
    let input = [
        (vec![1, 2, 4], vec![1, 3, 4]),
        (vec![], vec![]),
        (vec![], vec![0]),
        (vec![3, 100], vec![0, 1, 3, 3, 4, 5]),
        (vec![1, 3, 5, 7, 9, 11], vec![1, 3, 5, 7, 9, 11]),
        (vec![2, 4, 6, 8, 10, 12], vec![]),
        (vec![2, 6, 8, 9, 11, 15, 26], vec![2, 2, 6, 8, 8, 9, 15]),
    ];

    for nums in input {
        let l1 = ListNode::from_vec(nums.0);
        let l2 = ListNode::from_vec(nums.1);
        ListNode::print(Solution::merge_two_lists(l1, l2));
    }
}

/* recursive

struct Solution {}
impl Solution {
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> ListNode {
        ListNode { val, next }
    }
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n1), Some(n2)) => Some(Box::new(if n1.val < n2.val {
                Solution::new(n1.val, Solution::merge_two_lists(n1.next, Some(n2)))
            } else {
                Solution::new(n2.val, Solution::merge_two_lists(Some(n1), n2.next))
            })),
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            (None, None) => None,
        }
    }
}

pub fn run() {
    let input = [
        (vec![1, 2, 4], vec![1, 3, 4]),
        (vec![], vec![]),
        (vec![], vec![0]),
        (vec![3, 100], vec![0, 1, 3, 3, 4, 5]),
        (vec![1, 3, 5, 7, 9, 11], vec![1, 3, 5, 7, 9, 11]),
        (vec![2, 4, 6, 8, 10, 12], vec![]),
        (vec![2, 6, 8, 9, 11, 15, 26], vec![2, 2, 6, 8, 8, 9, 15]),
    ];

    for nums in input {
        let l1 = ListNode::from_vec(nums.0);
        let l2 = ListNode::from_vec(nums.1);
        ListNode::print(Solution::merge_two_lists(l1, l2));
    }
}

 */
