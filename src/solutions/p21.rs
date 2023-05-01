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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
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
    let input = vec![
        (vec![1, 2, 4], vec![1, 3, 4]),
        (vec![], vec![]),
        (vec![], vec![0]),
        (vec![3, 100], vec![0, 1, 3, 3, 4, 5]),
        (vec![1, 3, 5, 7, 9, 11], vec![1, 3, 5, 7, 9, 11]),
        (vec![2, 4, 6, 8, 10, 12], vec![]),
        (vec![2, 6, 8, 9, 11, 15, 26], vec![2, 2, 6, 8, 8, 9, 15]),
    ];

    for nums in input {
        let l1 = get_list_node(nums.0);
        let l2 = get_list_node(nums.1);
        let mut ans = Solution::merge_two_lists(l1, l2);
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

/* recursive
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
    let input = vec![
        (vec![1, 2, 4], vec![1, 3, 4]),
        (vec![], vec![]),
        (vec![], vec![0]),
        (vec![3, 100], vec![0, 1, 3, 3, 4, 5]),
        (vec![1, 3, 5, 7, 9, 11], vec![1, 3, 5, 7, 9, 11]),
        (vec![2, 4, 6, 8, 10, 12], vec![]),
        (vec![2, 6, 8, 9, 11, 15, 26], vec![2, 2, 6, 8, 8, 9, 15]),
    ];

    for nums in input {
        let l1 = get_list_node(nums.0);
        let l2 = get_list_node(nums.1);
        let mut ans = Solution::merge_two_lists(l1, l2);
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
 */
