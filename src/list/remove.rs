use super::ListNode;

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

/// 83. 删除排序链表中的重复元素
///
/// 给定一个已排序的链表的头 head ， 删除所有重复的元素，使每个元素只出现一次 。返回 已排序的链表 。
///
/// 示例 1：
///
/// <img src="https://assets.leetcode.com/uploads/2021/01/04/list1.jpg" />
///
/// 输入：head = [1,1,2]
///
/// 输出：[1,2]
///
/// 示例 2：
///
/// <img src="https://assets.leetcode.com/uploads/2021/01/04/list2.jpg" />
///
/// 输入：head = [1,1,2,3,3]
///
/// 输出：[1,2,3]
///
/// 提示：
///
/// 链表中节点数目在范围 [0, 300] 内
///
/// -100 <= Node.val <= 100
///
/// 题目数据保证链表已经按升序 排列
pub fn delete_duplicates(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    if head.is_none() {
        return head;
    }
    let mut head = head;
    // 对头节点的可变借用
    let mut cur = head.as_mut().unwrap();
    while let Some(next) = cur.next.take() {
        // 先夺
        if next.val == cur.val {
            cur.next = next.next;
        } else {
            // 如果不重复，则放回去
            cur.next = Some(next);
            // 慢指针前进
            cur = cur.next.as_mut().unwrap();
        }
    }
    head
}

/// 82. 删除排序链表中的重复元素 II
/// 已解答
/// 中等
/// 相关标签
/// 相关企业
/// 给定一个已排序的链表的头 head ， 删除原始链表中所有重复数字的节点，只留下不同的数字 。返回 已排序的链表 。
///
/// 示例 1：
///
/// 输入：head = [1,2,3,3,4,4,5]
/// 
/// 输出：[1,2,5]
/// 
/// 示例 2：
///
/// 输入：head = [1,1,1,2,3]
/// 
/// 输出：[2,3]
///
/// 提示：
///
/// 链表中节点数目在范围 [0, 300] 内
/// 
/// -100 <= Node.val <= 100
/// 
/// 题目数据保证链表已经按升序 排列
/// 
/// 方法二：一次遍历
/// 
/// 这里说的一次遍历，是说一边遍历、一边统计相邻节点的值是否相等，如果值相等就继续后移找到值不等的位置，然后删除值相等的这个区间。
///
/// 其实思路很简单，跟递归方法中的 while 语句跳过所有值相等的节点的思路是一样的：
/// 如果 cur.val == cur.next.val  说明两个相邻的节点值相等，所以继续后移，
/// 一直找到 cur.val != cur.next.val  ，此时的 cur.next  就是值不等的节点。
///
/// 比如： 1 -> 2 -> 2 -> 2 -> 3，我们用一个 pre 指向 1；当 cur 指向第一个 2 的时候，发现 cur.val == cur.next.val  ，
/// 所以出现了值重复的节点啊，所以 cur 一直后移到最后一个 2 的时候，发现 cur.val != cur.next.val  ，
/// 此时 cur.next = 3 ，所以 pre.next = cur.next ，即让1 的 next 节点是 3，就把中间的所有 2 都删除了。
/// 
/// 代码中用到了一个常用的技巧：dummy 节点，也叫做 哑节点。它在链表的迭代写法中非常常见，
/// 因为对于本题而言，我们可能会删除头结点 head，为了维护一个不变的头节点，所以我们添加了 dummy，
/// 让dummy.next = head，这样即使 head 被删了，那么会操作 dummy.next 指向新的链表头部，所以最终返回的也是 dummy.next。
/// 
/// 时间复杂度：O(N)，对链表每个节点遍历了一次。
/// 
/// 空间复杂度：O(1)，只使用了常量的空间。
pub fn delete_duplicates_2(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    if head.is_none() {
        return head;
    }
    // 创建一个虚拟节点作为新链表的头部
    let mut dummy_head = Box::new(ListNode::new(0));
    // 定义当前节点
    let mut pre = dummy_head.as_mut();
    // `prev_val` 用于记录前一个节点的值
    let mut prev_val = None;
    let mut cur = head;
    while let Some(mut cur_temp) = cur {
        // 取出当前节点，并将 `head` 移动到下一个节点
        cur = cur_temp.next.take();
        // 在潜在移动 `node` 之前，存储当前节点的值
        let node_val = cur_temp.val;
        if cur.as_ref().map_or(true, |next| next.val != node_val)
            && prev_val.map_or(true, |val| val != node_val)
        {
            // 如果当前节点是唯一的（值不等于下一个节点或前一个节点的值），则将其添加到新链表中
            pre.next = Some(cur_temp);
            // 移动pre节点
            pre = pre.next.as_mut().unwrap();
        }

        // 更新 `prev_val` 为当前节点的值
        prev_val = Some(node_val);
    }
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
