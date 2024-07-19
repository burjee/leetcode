use crate::utils::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut odd_head = head;
        let mut even_head = odd_head.as_mut().unwrap().next.take();
        let mut p1 = odd_head.as_mut();
        let mut p2 = even_head.as_mut();
        while let Some(node2) = p2 {
            let mut node1 = p1.unwrap();
            node1.next = node2.next.take();
            p1 = Some(node2);
            p2 = node1.next.as_mut();
        }

        p1 = odd_head.as_mut();
        while let Some(node) = p1 {
            if node.next.is_none() {
                node.next = even_head;
                break;
            }
            p1 = node.next.as_mut();
        }

        odd_head
    }

    // pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     if head.is_none() {
    //         return head;
    //     }

    //     let mut odd_head = Some(Box::new(ListNode::new(-1)));
    //     let mut even_head = Some(Box::new(ListNode::new(-1)));
    //     let mut odd_p = odd_head.as_mut();
    //     let mut even_p = even_head.as_mut();
    //     let mut i = 0;
    //     while let Some(mut node) = head {
    //         i += 1;
    //         head = node.next.take();
    //         if i % 2 == 1 {
    //             let p = odd_p.unwrap();
    //             p.next = Some(node);
    //             odd_p = p.next.as_mut();
    //         } else {
    //             let p = even_p.unwrap();
    //             p.next = Some(node);
    //             even_p = p.next.as_mut();
    //         }
    //     }
    //     odd_p.unwrap().next = even_head.unwrap().next;

    //     odd_head.unwrap().next
    // }
}

pub fn run() {
    let input = [vec![1, 2, 3, 4, 5], vec![2, 1, 3, 5, 6, 4, 7]];

    for nums in input {
        let head = ListNode::from_vec(nums);
        ListNode::print(Solution::odd_even_list(head));
    }
}
