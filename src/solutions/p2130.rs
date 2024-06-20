use crate::utils::list_node::ListNode;

struct Solution {}
impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut len = 0;
        let mut point = &head;
        while let Some(node) = point {
            len += 1;
            point = &node.next;
        }

        // 分割一半並反轉
        // - > - > - > - > - > - >
        // 變成
        // < - < - < - - > - > - >
        let mut right = head;
        let mut left = None;
        for _ in 0..len / 2 {
            let mut node = right.unwrap();
            right = node.next.take();
            node.next = left;
            left = Some(node);
        }

        let mut max = 0;
        while let (Some(l), Some(r)) = (left, right) {
            max = max.max(l.val + r.val);
            left = l.next;
            right = r.next;
        }
        max
    }

    pub fn _pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut nums = vec![];
        while let Some(node) = head {
            nums.push(node.val);
            head = node.next;
        }

        let len = nums.len();
        let mut max = 0;
        for i in 0..len / 2 {
            max = max.max(nums[i] + nums[len - i - 1]);
        }
        max
    }
}

pub fn run() {
    let input = [vec![5, 4, 2, 1], vec![4, 2, 2, 3], vec![1, 100000]];

    for nums in input {
        let head = ListNode::from_vec(nums);
        println!("{:?}", Solution::pair_sum(head));
    }
}
