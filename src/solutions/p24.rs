use crate::utils::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut a) = head {
            if let Some(mut b) = a.next.take() {
                a.next = Solution::swap_pairs(b.next.take());
                b.next = Some(a);
                Some(b)
            } else {
                Some(a)
            }
        } else {
            None
        }
    }

    pub fn _swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn take(node: Option<&mut Box<ListNode>>) -> Option<Box<ListNode>> {
            if node.is_some() {
                node.unwrap().next.take()
            } else {
                None
            }
        }

        let mut hair = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut take0 = &mut hair;

        loop {
            let mut take1 = take(Some(take0));
            if take1.is_none() {
                break;
            }

            let mut take2 = take(take1.as_mut());
            if take2.is_none() {
                take0.next = take1;
                break;
            }

            let take3 = take(take2.as_mut());

            take1.as_mut().unwrap().next = take3;
            take2.as_mut().unwrap().next = take1;
            take0.next = take2;
            take0 = take0.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        hair.next
    }
}

pub fn run() {
    let input = [
        vec![1, 2, 3, 4, 5],
        vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5],
        vec![100, 90],
        vec![],
        vec![1],
        vec![1, 2, 3, 4],
    ];

    for nums in input {
        let head = ListNode::from_vec(nums);
        ListNode::print(Solution::swap_pairs(head));
    }
}
