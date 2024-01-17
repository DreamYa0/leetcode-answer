#[derive(Debug)]
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

/// 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut tail = &mut dummy_head;
    let (mut l1_end, mut l2_end, mut carry) = (false, false, 0);
    loop {
        let (l1_val, next_l1) = match l1 {
            Some(node) => (node.val, node.next),
            None => {
                l1_end = true;
                (0, None)
            }
        };
        let (l2_val, next_l2) = match l2 {
            Some(node) => (node.val, node.next),
            None => {
                l2_end = true;
                (0, None)
            }
        };
        if l1_end && l2_end && carry == 0 {
            break;
        }
        let sum = carry + l1_val + l2_val;
        carry = sum / 10;
        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap();
        l1 = next_l1;
        l2 = next_l2;
    }
    dummy_head.next
}

/// 采用递归的写法
pub fn add_two_numbers_2(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => {
            if carry > 0 {
                Some(Box::new(ListNode::new(carry)))
            } else {
                None
            }
        }
        (Some(node), None) | (None, Some(node)) => {
            let sum = node.val + carry;
            Some(Box::new(ListNode {
                val: sum % 10,
                next: add_two_numbers_2(node.next, None, sum / 10),
            }))
        }
        (Some(node1), Some(node2)) => {
            let sum = node1.val + node2.val + carry;
            Some(Box::new(ListNode {
                val: sum % 10,
                next: add_two_numbers_2(node1.next, node2.next, sum / 10),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        };

        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        };

        let numbers = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        println!("{:?}", numbers)
    }
}
