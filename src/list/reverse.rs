use super::ListNode;

/// 206. 反转链表
/// 给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。
///
/// 示例 1：
///
/// <img src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg" />
///
/// 输入：head = [1,2,3,4,5]
/// 输出：[5,4,3,2,1]
/// 示例 2：
///
/// <img src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg" />
///
/// 输入：head = [1,2]
/// 输出：[2,1]
/// 示例 3：
///
/// 输入：head = []
/// 输出：[]
///
/// 提示：
///
/// 链表中节点的数目范围是 [0, 5000]
/// -5000 <= Node.val <= 5000
///
/// 进阶：链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？
pub fn reverse_list_by_two_pointers(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    // 定义cur节点指向头节点
    let mut cur = head;
    // 定义 pre 节点指向None
    let mut pre = None;
    // 取出cur节点值，临时存放到
    while let Some(mut temp) = cur.take() {
        // 右移cur指针
        cur = temp.next;
        // 反转操作，把头节点的next指向pre指针
        temp.next = pre;
        // 右移pre指针
        pre = Some(temp);
    }
    pre
}
