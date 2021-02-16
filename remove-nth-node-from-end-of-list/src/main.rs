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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut hair = Some(Box::new(ListNode::new(-1)));
        let mut nums = Vec::new();
        let mut node_n = 0;
        for _ in 0..n {
            let node = hair.unwrap();
            nums.push(node.val);
            hair = node.next;
        }
        while hair != None {
            let node = hair.unwrap();
            nums.push(node.val);
            hair = node.next;
            node_n += 1;
        }
        let mut ans = Some(Box::new(ListNode::new(nums[0])));
        let mut point = &mut ans;
        for i in 1..nums.len() {
            if i == node_n {
                continue;
            }
            point.as_mut().unwrap().next = Some(Box::new(ListNode::new(nums[i])));
            point = &mut point.as_mut().unwrap().next;
        }
        ans.unwrap().next
    }
    /* leetcode 1900kb sample
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });

        let mut right = dummy.clone();
        let mut left = dummy.as_mut();

        for _ in 0..n {
            right = right.next.unwrap();
        }

        while let Some(node) = right.next {
            right = node;
            left = left.next.as_mut().unwrap();
        }

        left.next = left.next.as_mut().unwrap().next.clone();

        dummy.next
    }
    */
}
fn main() {
    let input = vec![
        (vec![1, 2, 3, 4, 5], 2),
        (vec![1, 2, 3, 4, 5], 5),
        (vec![1], 1),
        (vec![1, 2], 2),
        (vec![5, 6, 85, 8, 6, 52, 54, 85, 8, 7, 4, 5, 6, 6, 8, 8], 5),
        (vec![1, 5, 7, 5, 6, 3, 6, 96, 6, 3, 85, 5, 5, 4, 4, 4], 12),
        (vec![9, 5, 1, 3, 2, 8, 8, 7, 4, 5, 52, 23, 5, 1, 9, 77], 1),
        (vec![5, 6, 85, 8, 6, 52, 54, 85, 8, 7, 4, 5, 6, 6, 8, 8], 7),
    ];

    for value in input {
        let head = get_head(value.0);

        let solution = Solution::remove_nth_from_end(head, value.1);
        let mut node = &solution;
        while let Some(n) = node {
            print!("{}, ", n.val);
            node = &n.next;
        }
        println!("");
    }
}

fn get_head(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    if let Some(&n) = numbers.get(0) {
        head = Some(Box::new(ListNode::new(n)));
    }
    let mut point = &mut head;
    for i in 1..numbers.len() {
        let new_node = Some(Box::new(ListNode::new(numbers[i])));
        let this_node = point.as_mut().unwrap();
        this_node.next = new_node;
        point = &mut this_node.next;
    }
    head
}
