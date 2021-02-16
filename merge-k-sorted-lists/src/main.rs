use std::collections::BTreeMap;

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut tree = BTreeMap::new();
        for mut list in lists {
            while let Some(node) = list {
                let count = tree.entry(node.val).or_insert(0);
                *count += 1;
                list = node.next;
            }
        }
        let mut head = Box::new(ListNode::new(-1));
        let mut point = &mut head;
        for (k, v) in tree {
            let node = Some(Box::new(ListNode::new(k)));
            for _ in 0..v {
                point.next = node.clone();
                point = point.next.as_mut().unwrap();
            }
        }
        head.next
    }
}

fn main() {
    let lists = vec![
        vec![vec![1, 2, 4], vec![1, 3, 4], vec![1, 3, 4], vec![1, 3, 4]],
        vec![vec![], vec![]],
        vec![],
        vec![vec![], vec![0]],
        vec![vec![3, 100], vec![], vec![0, 1, 3, 3, 4, 5]],
        vec![vec![1, 3, 5, 7, 9, 11], vec![1, 3, 5, 7, 9, 11]],
        vec![vec![2, 4, 6, 8, 10, 12], vec![]],
        vec![vec![2, 2, 6, 8, 8, 9, 15]],
    ];

    for list in lists {
        let mut stack = Vec::new();
        for nums in list {
            stack.push(get_list_node(nums));
        }
        let mut ans = Solution::merge_k_lists(stack);
        while let Some(n) = ans {
            print!("{}, ", n.val);
            ans = n.next;
        }
        println!("");
    }
}

fn get_list_node(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(-1));
    let mut point = head.as_mut();
    for n in numbers {
        let node = Some(Box::new(ListNode { val: n, next: None }));
        point.next = node;
        point = point.next.as_mut().unwrap();
    }
    head.next
}
