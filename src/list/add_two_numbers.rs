use super::ListNode;

/// 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
///
/// 请你将两个数相加，并以相同形式返回一个表示和的链表。
/// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
///
/// 示例 1：
/// 输入：l1 = [2,4,3], l2 = [5,6,4]
/// 输出：[7,0,8]
/// 解释：342 + 465 = 807.
///
/// 示例 2：
/// 输入：l1 = [0], l2 = [0]
/// 输出：[0]
///
/// 示例 3：
/// 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// 输出：[8,9,9,9,0,0,0,1]
///
/// 提示：
/// 每个链表中的节点数在范围 [1, 100] 内
/// 0 <= Node.val <= 9
/// 题目数据保证列表表示的数字不含前导零
pub fn add_two_numbers(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    do_sum(l1, l2, 0)
}

/// 递归求和
fn do_sum(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
    mut carry: i32,
) -> Option<Box<ListNode<i32>>> {
    if l1.is_some() || l2.is_some() || carry > 0 {
        Some(Box::new(ListNode {
            next: do_sum(
                l1.and_then(|f| {
                    carry += f.val;
                    f.next
                }),
                l2.and_then(|f| {
                    carry += f.val;
                    f.next
                }),
                // 商作为进位的树
                carry / 10,
            ),
            // 余数作为当前节点的值
            val: carry % 10,
        }))
    } else {
        None
    }
}

/// 遍历求和
#[allow(dead_code)]
fn do_sum_2(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    // 定义一个新链表的虚拟头节点
    let mut dummy_head = Box::new(ListNode::new(0));
    // 定义当前节点
    let mut cur = &mut dummy_head;
    // 定义一个进位数指针
    let mut carry = 0;
    // 定义两个链表的指针
    let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
    while l1.is_some() || l2.is_some() {
        let mut sum = carry;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next.as_ref();
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next.as_ref();
        }
        carry = sum / 10;
        cur.next = Some(Box::new(ListNode::new(sum % 10)));
        cur = cur.next.as_mut().unwrap();
    }
    if carry > 0 {
        // 如果进位数不为0，将进位数作为新节点的值
        cur.next = Some(Box::new(ListNode::new(carry)));
    }
    dummy_head.next
}

#[allow(dead_code)]
fn reverse(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    // 定义前驱节点
    let mut prev = None;
    // 定义当前节点
    let mut cur = head;
    while let Some(mut node) = cur.take() {
        // 保存当前节点的下一个节点
        let next = node.next;
        // 将当前节点的下一个节点指向前驱节点
        node.next = prev;
        // 将当前节点作为前驱节点
        prev = Some(node);
        // 将下一个节点作为当前节点
        cur = next;
    }
    prev
}

pub fn add_two_numbers_2(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    reverse(do_sum(reverse(l1), reverse(l2), 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        };

        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        };

        let mut numbers = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        // 遍历链表
        let mut vec = Vec::new();
        while let Some(node) = numbers {
            vec.push(node.val);
            numbers = node.next;
        }
        // 反转向量
        vec.reverse();
        println!("{:?}", vec)
    }
}
