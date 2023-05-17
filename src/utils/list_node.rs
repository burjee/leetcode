// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut next = None;
        for val in nums.into_iter().rev() {
            next = Some(Box::new(ListNode { val, next }));
        }
        next
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut nums = vec![self.val];
        let mut point = &self.next;
        while let Some(node) = point {
            nums.push(node.val);
            point = &node.next;
        }
        nums
    }
}

// impl From<Vec<i32>> for ListNode {
//     fn from(nums: Vec<i32>) -> Self {
//         let mut next = None;
//         for val in nums.into_iter().rev() {
//             next = Some(Box::new(ListNode { val, next }));
//         }
//         *next.unwrap()
//     }
// }
