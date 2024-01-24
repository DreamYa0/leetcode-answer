/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

/// 203. 移除链表元素
/// 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。
///
///
/// 示例 1：
///
/// <img src="https://assets.leetcode.com/uploads/2021/03/06/removelinked-list.jpg" alt="">
/// 输入：head = [1,2,6,3,4,5,6], val = 6
/// 输出：[1,2,3,4,5]
/// 示例 2：
///
/// 输入：head = [], val = 1
/// 输出：[]
/// 示例 3：
///
/// 输入：head = [7,7,7,7], val = 7
/// 输出：[]
///
///
/// 提示：
///
/// 列表中的节点数目在范围 [0, 104] 内
/// 1 <= Node.val <= 50
/// 0 <= val <= 50
pub fn remove_elements(head: Option<Box<ListNode<i32>>>, val: i32) -> Option<Box<ListNode<i32>>> {
    // 使用虚拟头节点
    let mut dummy_head = Box::new(ListNode::new(0));
    dummy_head.next = head;
    // 可变借用
    let mut cur = dummy_head.as_mut();
    while let Some(nxt) = cur.next.take() {
        // 如果当前节点的值等于 val，那么就把当前节点的下一个节点赋值给当前节点的下一个节点
        if nxt.val == val {
            // 这里不需要手动释放内存，因为 nxt 是一个 Box，当它超出作用域时，会自动释放内存
            // 如果是需要删除的节点，则把cur.next指向nxt.next
            cur.next = nxt.next;
        } else {
            // 如果不是删除的节点，由于cur.next已经被取出了，所以需要重新放回去
            cur.next = Some(nxt);
            // 继续向下一个节点移动
            cur = cur.next.as_mut().unwrap();
        }
    }
    // 返回虚拟头节点的下一个节点
    dummy_head.next
}

/// 19. 删除链表的倒数第 N 个结点
/// 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
///
///
/// 示例 1：
///
/// <img src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" alt="">
///
/// 输入：head = [1,2,3,4,5], n = 2
/// 输出：[1,2,3,5]
/// 示例 2：
///
/// 输入：head = [1], n = 1
/// 输出：[]
/// 示例 3：
///
/// 输入：head = [1,2], n = 1
/// 输出：[1]
///
/// 提示：
///
/// 链表中结点的数目为 sz
/// 1 <= sz <= 30
/// 0 <= Node.val <= 100
/// 1 <= n <= sz
///
/// 进阶：你能尝试使用一趟扫描实现吗？
pub fn remove_nth_from_end(head: Option<Box<ListNode<i32>>>, n: i32) -> Option<Box<ListNode<i32>>> {
    if n <= 0 {
        return head;
    }
    // 定义起始位置
    let mut len = 0;
    let mut cur = head.as_ref();
    // 第一遍遍历确定链表的长度
    while cur.is_some() {
        // 确定链表的长度
        len += 1;
        // 遍历链表
        cur = cur.unwrap().next.as_ref();
    }

    // 定义虚拟头节点
    let mut dummy_head = Box::new(ListNode::new(0));
    // 把虚拟头节点的next指向head
    dummy_head.next = head;
    // 定义可变借用，定义虚拟头节点为当前节点
    let mut cur = dummy_head.as_mut();
    // 定义索引
    let mut i = 0;
    while let Some(nxt) = cur.next.take() {
        if len - n == i {
            // 如果是需要删除的节点，则把cur.next指向nxt.next
            cur.next = nxt.next;
        } else {
            // 如果不是删除的节点，由于cur.next已经被取出了，所以需要重新放回去
            cur.next = Some(nxt);
            // 继续向下一个节点移动
            cur = cur.next.as_mut().unwrap();
        }
        i += 1;
    }
    // 返回虚拟头节点的下一个节点
    dummy_head.next
}

/// 19. 删除链表的倒数第 N 个结点
/// 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
///
///
/// 示例 1：
///
/// <img src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" alt="">
///
/// 输入：head = [1,2,3,4,5], n = 2
/// 输出：[1,2,3,5]
/// 示例 2：
///
/// 输入：head = [1], n = 1
/// 输出：[]
/// 示例 3：
///
/// 输入：head = [1,2], n = 1
/// 输出：[1]
///
/// 提示：
///
/// 链表中结点的数目为 sz
/// 1 <= sz <= 30
/// 0 <= Node.val <= 100
/// 1 <= n <= sz
///
/// 进阶：你能尝试使用一趟扫描实现吗？
///
///
/// 思路
/// 双指针的经典应用，如果要删除倒数第n个节点，让fast移动n步，然后让fast和slow同时移动，直到fast指向链表末尾。删掉slow所指向的节点就可以了。
///
/// 思路是这样的，但要注意一些细节。
///
/// 分为如下几步：
///
/// 首先这里我推荐大家使用虚拟头结点，这样方便处理删除实际头结点的逻辑，如果虚拟头结点不清楚，可以看这篇： 链表：听说用虚拟头节点会方便很多？(opens new window)
///
/// 定义fast指针和slow指针，初始值为虚拟头结点，如图：
///
/// <img src="https://code-thinking.cdn.bcebos.com/pics/19.%E5%88%A0%E9%99%A4%E9%93%BE%E8%A1%A8%E7%9A%84%E5%80%92%E6%95%B0%E7%AC%ACN%E4%B8%AA%E8%8A%82%E7%82%B9.png" alt="">
///
///
/// fast首先走n + 1步 ，为什么是n+1呢，因为只有这样同时移动的时候slow才能指向删除节点的上一个节点（方便做删除操作），如图：
///
/// <img src="https://code-thinking.cdn.bcebos.com/pics/19.%E5%88%A0%E9%99%A4%E9%93%BE%E8%A1%A8%E7%9A%84%E5%80%92%E6%95%B0%E7%AC%ACN%E4%B8%AA%E8%8A%82%E7%82%B91.png" alt="">
///
/// fast和slow同时移动，直到fast指向末尾，如题：
///
/// <img src="https://code-thinking.cdn.bcebos.com/pics/19.%E5%88%A0%E9%99%A4%E9%93%BE%E8%A1%A8%E7%9A%84%E5%80%92%E6%95%B0%E7%AC%ACN%E4%B8%AA%E8%8A%82%E7%82%B92.png" alt="">
///
/// 删除slow指向的下一个节点，如图：
///
/// <img src="https://code-thinking.cdn.bcebos.com/pics/19.%E5%88%A0%E9%99%A4%E9%93%BE%E8%A1%A8%E7%9A%84%E5%80%92%E6%95%B0%E7%AC%ACN%E4%B8%AA%E8%8A%82%E7%82%B93.pngg" alt="">
pub fn remove_nth_from_end_by_two_pointers(
    head: Option<Box<ListNode<i32>>>,
    mut n: i32,
) -> Option<Box<ListNode<i32>>> {
    // 定义虚拟头节点
    let mut dummy_head = Box::new(ListNode::new(0));
    // 把虚拟头节点的next指向头节点
    dummy_head.next = head;
    // 定义快指针
    let mut fast = &dummy_head.clone();
    // 定义慢指针
    let mut slow = &mut dummy_head;
    while n > 0 {
        // 让快指针先移动 n+1，这里是先移动在递减n，所以循环n次实际是移动了n+1
        fast = fast.next.as_ref().unwrap();
        n -= 1;
    }

    // 同时移动快慢指针
    while fast.next.is_some() {
        // 循环结束的条件是 fast指针指向None
        fast = fast.next.as_ref().unwrap();
        slow = slow.next.as_mut().unwrap();
    }
    slow.next = slow.next.as_mut().unwrap().next.take();
    dummy_head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_elements() {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut node2 = Some(Box::new(ListNode::new(2)));
        let mut node3 = Some(Box::new(ListNode::new(6)));
        let mut node4 = Some(Box::new(ListNode::new(3)));
        let mut node5 = Some(Box::new(ListNode::new(4)));
        let mut node6 = Some(Box::new(ListNode::new(5)));
        let node7 = Some(Box::new(ListNode::new(6)));
        node6.as_mut().unwrap().next = node7;
        node5.as_mut().unwrap().next = node6;
        node4.as_mut().unwrap().next = node5;
        node3.as_mut().unwrap().next = node4;
        node2.as_mut().unwrap().next = node3;
        head.as_mut().unwrap().next = node2;
        let res = remove_elements(head, 6);
        println!("{:?}", res)
    }

    #[test]
    fn test_remove_nth_from_end() {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut node2 = Some(Box::new(ListNode::new(2)));
        let mut node3 = Some(Box::new(ListNode::new(3)));
        let mut node4 = Some(Box::new(ListNode::new(4)));
        let node5 = Some(Box::new(ListNode::new(5)));
        node4.as_mut().unwrap().next = node5;
        node3.as_mut().unwrap().next = node4;
        node2.as_mut().unwrap().next = node3;
        head.as_mut().unwrap().next = node2;
        let res = remove_nth_from_end(head, 2);
        println!("{:?}", res)
    }
}
