use super::ListNode;

/// 707. 设计链表
/// 你可以选择使用单链表或者双链表，设计并实现自己的链表。
///
/// 单链表中的节点应该具备两个属性：val 和 next 。val 是当前节点的值，next 是指向下一个节点的指针/引用。
///
/// 如果是双向链表，则还需要属性 prev 以指示链表中的上一个节点。假设链表中的所有节点下标从 0 开始。
///
/// 实现 MyLinkedList 类：
///
/// MyLinkedList() 初始化 MyLinkedList 对象。
/// int get(int index) 获取链表中下标为 index 的节点的值。如果下标无效，则返回 -1 。
/// void addAtHead(int val) 将一个值为 val 的节点插入到链表中第一个元素之前。在插入完成后，新节点会成为链表的第一个节点。
/// void addAtTail(int val) 将一个值为 val 的节点追加到链表中作为链表的最后一个元素。
/// void addAtIndex(int index, int val) 将一个值为 val 的节点插入到链表中下标为 index 的节点之前。如果 index 等于链表的长度，那么该节点会被追加到链表的末尾。如果 index 比长度更大，该节点将 不会插入 到链表中。
/// void deleteAtIndex(int index) 如果下标有效，则删除链表中下标为 index 的节点。
///
/// 示例：
///
/// 输入
/// ["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
/// [[], [1], [3], [1, 2], [1], [1], [1]]
/// 输出
/// [null, null, null, null, 2, null, 3]
///
/// 解释
/// MyLinkedList myLinkedList = new MyLinkedList();
/// myLinkedList.addAtHead(1);
/// myLinkedList.addAtTail(3);
/// myLinkedList.addAtIndex(1, 2);    /// 链表变为 1->2->3
/// myLinkedList.get(1);              /// 返回 2
/// myLinkedList.deleteAtIndex(1);    /// 现在，链表变为 1->3
/// myLinkedList.get(1);              /// 返回 3
///
/// 提示：
///
/// 0 <= index, val <= 1000
/// 请不要使用内置的 LinkedList 库。
/// 调用 get、addAtHead、addAtTail、addAtIndex 和 deleteAtIndex 的次数不超过 2000 。
///
/// 思路
/// 如果对链表的基础知识还不太懂，可以看这篇文章：关于链表，你该了解这些！(opens new window)
///
/// 如果对链表的虚拟头结点不清楚，可以看这篇文章：链表：听说用虚拟头节点会方便很多？(opens new window)
///
/// 删除链表节点： 链表-删除节点
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20200806195114541.png" alt="链表-删除节点" style="zoom:50%;" />
///
/// 添加链表节点： 链表-添加节点
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20200806195134331.png" alt="链表-添加节点" style="zoom:50%;" />
/// 这道题目设计链表的五个接口：
///
/// 获取链表第index个节点的数值
/// 在链表的最前面插入一个节点
/// 在链表的最后面插入一个节点
/// 在链表第index个节点前面插入一个节点
/// 删除链表的第index个节点
/// 可以说这五个接口，已经覆盖了链表的常见操作，是练习链表操作非常好的一道题目
///
/// 链表操作的两种方式：
///
/// 直接使用原来的链表来进行操作。
/// 设置一个虚拟头结点在进行操作。
/// 下面采用的设置一个虚拟头结点（这样更方便一些，大家看代码就会感受出来）。
#[derive(Debug)]
pub struct MyLinkedList {
    pub val: i32,
    pub next: Option<Box<MyLinkedList>>,
}

impl MyLinkedList {
    #[allow(dead_code)]
    fn new() -> Self {
        // 增加头节点
        MyLinkedList { val: 0, next: None }
    }

    #[allow(dead_code)]
    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        let mut i = 0;
        let mut cur = &self.next;
        while let Some(node) = cur {
            if i == index {
                return node.val;
            }
            i += 1;
            cur = &node.next;
        }
        -1
    }

    #[allow(dead_code)]
    fn add_at_head(&mut self, val: i32) {
        // 注意：必须先将新的节点的next指向虚拟头节点的next，再将虚拟头节点的next指向新的节点，虚拟头节点的next才是真正的头节点
        let new_node = Box::new(MyLinkedList {
            val,
            next: self.next.take(),
        });
        self.next = Some(new_node);
    }

    #[allow(dead_code)]
    fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList { val, next: None });
        let mut last_node = &mut self.next;
        while let Some(node) = last_node {
            // 一直遍历到node.next指向None，即最后一个节点
            last_node = &mut node.next;
        }
        // 将最后一个节点的next指向新的节点
        *last_node = Some(new_node);
    }

    #[allow(dead_code)]
    fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            // 头部插入节点
            self.add_at_head(val);
        } else {
            // 起始位置
            let mut i = 0;
            // 把头节点设置为当前节点
            let mut cur = &mut self.next;
            while let Some(node) = cur {
                // 遍历到index的前一个节点
                if i + 1 == index {
                    let new_node = Box::new(MyLinkedList {
                        val,
                        // 新节点的next指向cur节点的next
                        next: node.next.take(),
                    });
                    // cur节点的next指向新的节点
                    node.next = Some(new_node);
                    break;
                }
                // 移动i指针
                i += 1;
                // 遍历链表
                cur = &mut node.next;
            }
        }
    }

    #[allow(dead_code)]
    fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }

        let mut i = 0;
        let mut cur = self;
        while let Some(node) = cur.next.take() {
            if i == index {
                // 等价于 cur.next.next
                cur.next = node.next;
                break;
            }
            i += 1;
            // 不满足删除条件需重新放回
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
    }
}

/// 21. 合并两个有序链表
/// 
/// 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
///
/// 示例 1：
///
/// 输入：l1 = [1,2,4], l2 = [1,3,4]
/// 
/// 输出：[1,1,2,3,4,4]
/// 
/// 示例 2：
///
/// 输入：l1 = [], l2 = []
/// 
/// 输出：[]
/// 
/// 示例 3：
///
/// 输入：l1 = [], l2 = [0]
/// 
/// 输出：[0]
///
/// 提示：
///
/// 两个链表的节点数目范围是 [0, 50]
/// 
/// -100 <= Node.val <= 100
/// 
/// l1 和 l2 均按 非递减顺序 排列
///
/// 递归解法
/// 
/// 根据以上规律考虑本题目：
///
/// 终止条件：当两个链表都为空时，表示我们对链表已合并完成。
/// 
/// 如何递归：我们判断 l1 和 l2 头结点哪个更小，然后较小结点的 next 指针指向其余结点的合并结果。（调用递归）
pub fn merge_two_lists(
    list1: Option<Box<ListNode<i32>>>,
    list2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    if list1.is_none() {
        return list2;
    } else if list2.is_none() {
        return list1;
    } else if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
        let mut list1 = list1;
        list1.as_mut().unwrap().next = merge_two_lists(list1.as_mut().unwrap().next.take(), list2);
        list1
    } else {
        let mut list2 = list2;
        list2.as_mut().unwrap().next = merge_two_lists(list1, list2.as_mut().unwrap().next.take());
        list2
    }
}

/// 23. 合并 K 个升序链表
/// 给你一个链表数组，每个链表都已经按升序排列。
///
/// 请你将所有链表合并到一个升序链表中，返回合并后的链表。
///
///
/// 示例 1：
///
/// 输入：lists = [[1,4,5],[1,3,4],[2,6]]
/// 
/// 输出：[1,1,2,3,4,4,5,6]
/// 
/// 解释：链表数组如下：
/// 
/// ```
/// [
///   1->4->5,
///   1->3->4,
///   2->6
/// ]
/// ```
/// 
/// 将它们合并到一个有序链表中得到。
/// 
/// 1->1->2->3->4->4->5->6
/// 
/// 示例 2：
///
/// 输入：lists = []
/// 
/// 输出：[]
/// 
/// 示例 3：
///
/// 输入：lists = [[]]
/// 
/// 输出：[]
///
/// 提示：
///
/// k == lists.length
/// 
/// 0 <= k <= 10^4
/// 
/// ```
/// 0 <= lists[i].length <= 500
/// ```
/// 
/// -10^4 <= lists[i][j] <= 10^4
/// 
/// lists[i] 按 升序 排列
/// 
/// lists[i].length 的总和不超过 10^4
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode<i32>>>>) -> Option<Box<ListNode<i32>>> {
    let mut res = None;
    for list in lists {
        res = merge_two_lists(res, list);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let l1 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(4))),
            })),
        };
        let l2 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        };
        let res = merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
        assert_eq!(res, Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(4))),
                        })),
                    })),
                })),
            })),
        })));
    }
}
