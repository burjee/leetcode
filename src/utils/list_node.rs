// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut next = None;
        for val in nums.into_iter().rev() {
            next = Some(Box::new(ListNode { val, next }));
        }
        next
    }

    pub fn get_vec(&self) -> Vec<i32> {
        let mut nums = vec![self.val];
        let mut next = &self.next;
        while let Some(node) = next {
            nums.push(node.val);
            next = &node.next;
        }
        nums
    }

    pub fn print(mut list_node: Option<Box<ListNode>>) {
        while let Some(node) = list_node {
            print!("{} ", node.val);
            list_node = node.next;
        }
        println!();
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
