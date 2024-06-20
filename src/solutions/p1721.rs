use crate::utils::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut point = &head;
        while let Some(node) = point {
            len += 1;
            point = &node.next;
        }

        let mut k1 = k as usize;
        let mut k2 = len - k1 + 1;
        if k1 == k2 {
            return head;
        }
        if k1 > k2 {
            std::mem::swap(&mut k1, &mut k2);
        }

        let mut hair = Box::new(ListNode {
            val: -1,
            next: head,
        });

        let mut n1 = &mut hair;
        for _ in 0..k1 - 1 {
            n1 = n1.next.as_mut().unwrap();
        }
        let mut n2 = n1.next.take();
        let mut n3 = n2.as_mut().unwrap().next.take();
        let mut n4 = n3.as_mut().unwrap();
        for _ in k1 + 1..k2 - 1 {
            n4 = n4.next.as_mut().unwrap();
        }
        let mut n5 = n4.next.take();

        let beside = k1 + 1 == k2;
        if beside {
            n2.as_mut().unwrap().next = n5;
            n4.next = n2;
            n1.next = n3;
        } else {
            n2.as_mut().unwrap().next = n5.as_mut().unwrap().next.take();
            n4.next = n2;
            n5.as_mut().unwrap().next = n3;
            n1.next = n5;
        }

        hair.next
    }

    pub fn _swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        use std::mem::swap;

        let mut len = 0;
        let mut point = &head;
        while let Some(node) = point {
            len += 1;
            point = &node.next;
        }

        let mut k1 = k as usize;
        let mut k2 = len - k1 + 1;
        if k1 == k2 {
            return head;
        }
        if k1 > k2 {
            swap(&mut k1, &mut k2);
        }

        let mut p1 = head.as_mut();
        for _ in 1..k1 {
            p1 = p1.unwrap().next.as_mut();
        }
        let n1 = p1.unwrap();

        let mut p2 = n1.next.as_mut();
        for _ in k1 + 1..k2 {
            p2 = p2.unwrap().next.as_mut();
        }
        let n2 = p2.unwrap();

        swap(&mut n1.val, &mut n2.val);

        head
    }

    pub fn __swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut nodes = vec![];
        while let Some(mut node) = head {
            head = node.next.take();
            nodes.push(node);
        }

        let k1 = k as usize;
        let k2 = nodes.len() - k1;
        nodes.swap(k1 - 1, k2);

        let mut next = None;
        for mut node in nodes.into_iter().rev() {
            node.next = next;
            next = Some(node);
        }
        next
    }
}

pub fn run() {
    let input = [
        (vec![1, 2, 3, 4, 5], 2),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 2),
        (vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5], 5),
        (vec![100, 90], 2),
    ];

    for (nums, k) in input {
        let head = ListNode::from_vec(nums);
        println!(
            "{:?}",
            Solution::swap_nodes(head, k)
                .unwrap_or(Box::new(ListNode::new(-1)))
                .get_vec()
        );
    }
}
