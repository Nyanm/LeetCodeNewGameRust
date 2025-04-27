// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode
{
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode
{
    #[inline]
    fn new(val: i32) -> Self
    {
        ListNode { next: None, val }
    }
}

pub struct Solution;
impl Solution
{
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>
    {
        let mut node_root = ListNode::new(0);
        let mut node_cur = &mut node_root;

        let mut node_l1 = l1.as_ref();
        let mut node_l2 = l2.as_ref();

        let mut carry = 0;

        loop {
            // unwrap value
            let l1_validity = !node_l1.is_none();
            let l2_validity = !node_l2.is_none();
            let l1_val = if l1_validity { node_l1.as_ref().unwrap().val } else { 0 };
            let l2_val = if l2_validity { node_l2.as_ref().unwrap().val } else { 0 };

            // calculate current value
            let cur_val = l1_val + l2_val + carry;
            carry = if cur_val > 9 { 1 } else { 0 };
            let cur_val = cur_val % 10;

            // termination check
            if (carry == 0 && cur_val == 0) && (!l1_validity && !l2_validity) { break; }

            // enable value
            node_cur.next = Some(Box::new(ListNode::new(cur_val)));

            // advance answer, l1 and l2
            node_cur = node_cur.next.as_mut().unwrap();
            if l1_validity { node_l1 = node_l1.unwrap().next.as_ref(); }
            if l2_validity { node_l2 = node_l2.unwrap().next.as_ref(); }
        }

        return node_root.next;
    }
}
