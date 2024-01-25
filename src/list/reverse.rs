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
///
/// 思路
/// 如果再定义一个新的链表，实现链表元素的反转，其实这是对内存空间的浪费。
///
/// 其实只需要改变链表的next指针的指向，直接将链表反转 ，而不用重新定义一个新的链表，如图所示:
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20210218090901207.png" />
///
/// 206_反转链表
///
/// 之前链表的头节点是元素1， 反转之后头结点就是元素5 ，这里并没有添加或者删除节点，仅仅是改变next指针的方向。
///
/// 那么接下来看一看是如何反转的呢？
///
/// 我们拿有示例中的链表来举例，如动画所示：（纠正：动画应该是先移动pre，在移动cur）
///
/// <img src="https://code-thinking.cdn.bcebos.com/gifs/206.%E7%BF%BB%E8%BD%AC%E9%93%BE%E8%A1%A8.gif" />
///
/// 首先定义一个cur指针，指向头结点，再定义一个pre指针，初始化为null。
///
/// 然后就要开始反转了，首先要把 cur->next 节点用tmp指针保存一下，也就是保存一下这个节点。
///
/// 为什么要保存一下这个节点呢，因为接下来要改变 cur->next 的指向了，将cur->next 指向pre ，此时已经反转了第一个节点了。
///
/// 接下来，就是循环走如下代码逻辑了，继续移动pre和cur指针。
///
/// 最后，cur 指针已经指向了null，循环结束，链表也反转完毕了。 此时我们return pre指针就可以了，pre指针就指向了新的头结点。
/// 时间复杂度: O(n)
/// 空间复杂度: O(1)
pub fn reverse_list_by_two_pointers(
    head: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    // 定义cur节点指向头节点
    let mut cur = head;
    // 定义 pre 节点指向None
    let mut pre = None;
    // 取出cur节点值，临时存放到temp
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

/// 206. 反转链表
/// 递归的解法
/// 递归法相对抽象一些，但是其实和双指针法是一样的逻辑，同样是当cur为空的时候循环结束，不断将cur指向pre的过程。
///
/// 关键是初始化的地方，可能有的同学会不理解， 可以看到双指针法中初始化 cur = head，pre = NULL，在递归法中可以从如下代码看出初始化的逻辑也是一样的，只不过写法变了。
///
/// 具体可以看代码（已经详细注释），双指针法写出来之后，理解如下递归写法就不难了，代码逻辑都是一样的。
/// 时间复杂度: O(n), 要递归处理链表的每个节点
/// 空间复杂度: O(n), 递归调用了 n 层栈空间
/// 我们可以发现，上面的递归写法和双指针法实质上都是从前往后翻转指针指向，其实还有另外一种与双指针法不同思路的递归写法：从后往前翻转指针指向。
pub fn reverse_list_by_recursion(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    fn reverse(
        mut pre: Option<Box<ListNode<i32>>>,
        mut head: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        if let Some(mut temp) = head.take() {
            // 右移cur指针
            let cur = temp.next;
            // 反转链表
            temp.next = pre;
            // 右移pre指针
            pre = Some(temp);
            // 递归
            return reverse(pre, cur);
        }
        pre
    }
    reverse(None, head)
}

/// 24. 两两交换链表中的节点
/// 给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。
///
///
/// 示例 1：
///
/// <img src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" />
///
/// 输入：head = [1,2,3,4]
/// 输出：[2,1,4,3]
/// 示例 2：
///
/// 输入：head = []
/// 输出：[]
/// 示例 3：
///
/// 输入：head = [1]
/// 输出：[1]
///
/// 提示：
///
/// 链表中节点的数目在范围 [0, 100] 内
/// 0 <= Node.val <= 100
///
/// 思路
/// 这道题目正常模拟就可以了。
///
/// 建议使用虚拟头结点，这样会方便很多，要不然每次针对头结点（没有前一个指针指向头结点），还要单独处理。
///
/// 对虚拟头结点的操作，还不熟悉的话，可以看这篇链表：听说用虚拟头节点会方便很多？ (opens new window)。
///
/// 接下来就是交换相邻两个元素了，此时一定要画图，不画图，操作多个指针很容易乱，而且要操作的先后顺序
///
/// 初始时，cur指向虚拟头结点，然后进行如下三步：
///
/// <img src="https://code-thinking.cdn.bcebos.com/pics/24.%E4%B8%A4%E4%B8%A4%E4%BA%A4%E6%8D%A2%E9%93%BE%E8%A1%A8%E4%B8%AD%E7%9A%84%E8%8A%82%E7%82%B91.png" />
///
/// 操作之后，链表如下：
///
/// <img src="https://code-thinking.cdn.bcebos.com/pics/24.%E4%B8%A4%E4%B8%A4%E4%BA%A4%E6%8D%A2%E9%93%BE%E8%A1%A8%E4%B8%AD%E7%9A%84%E8%8A%82%E7%82%B92.png" />
///
/// 看这个可能就更直观一些了：
///
/// <img src="https://code-thinking.cdn.bcebos.com/pics/24.%E4%B8%A4%E4%B8%A4%E4%BA%A4%E6%8D%A2%E9%93%BE%E8%A1%A8%E4%B8%AD%E7%9A%84%E8%8A%82%E7%82%B93.png" />
///
pub fn swap_pairs(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    // 定义虚拟头节点
    let mut dummy_head = Box::new(ListNode::new(0));
    // 虚拟节点的next节点指向head节点
    dummy_head.next = head;
    let mut cur = dummy_head.as_mut();
    // cur.next 或 cur.next.next为空时链表的遍历就停止
    // temp 为节点1的临时保存值
    while let Some(mut temp) = cur.next.take() {
        // next 为节点2
        if let Some(mut next) = temp.next.take() {
            // 节点1的next指向节点3
            temp.next = next.next.take();
            // 节点2的next指向节点1
            next.next = Some(temp);
            // 当前节点的next指向节点2
            cur.next = Some(next);
            // 移动当前指针到节点2
            cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
        } else {
            // 放回
            cur.next = Some(temp);
            // 移动指针
            cur = cur.next.as_mut().unwrap();
        }
    }

    // 放回虚拟指针的next节点
    dummy_head.next
}

/// 面试题 02.07. 链表相交
/// 同：160.链表相交
///
/// 力扣题目链接(opens new window)
///
/// 给你两个单链表的头节点 headA 和 headB ，请你找出并返回两个单链表相交的起始节点。如果两个链表没有交点，返回 null 。
///
/// 图示两个链表在节点 c1 开始相交：
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20211219221657.png" />
///
/// 题目数据 保证 整个链式结构中不存在环。
///
/// 注意，函数返回结果后，链表必须 保持其原始结构 。
///
/// 示例 1：
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20211219221723.png" />
///
/// 示例 2：
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20211219221749.png" />
///
/// 示例 3：
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20211219221812.png" />
///
/// #思路
/// 简单来说，就是求两个链表交点节点的指针。 这里同学们要注意，交点不是数值相等，而是指针相等。
///
/// 为了方便举例，假设节点元素数值相等，则节点指针相等。
///
/// 看如下两个链表，目前curA指向链表A的头结点，curB指向链表B的头结点：
///
/// <img src="https://code-thinking.cdn.bcebos.com/pics/%E9%9D%A2%E8%AF%95%E9%A2%9802.07.%E9%93%BE%E8%A1%A8%E7%9B%B8%E4%BA%A4_1.png" />
/// 
/// 面试题02.07.链表相交_1
///
/// 我们求出两个链表的长度，并求出两个链表长度的差值，然后让curA移动到，和curB 末尾对齐的位置，如图：
/// 
/// <img src="https://code-thinking.cdn.bcebos.com/pics/%E9%9D%A2%E8%AF%95%E9%A2%9802.07.%E9%93%BE%E8%A1%A8%E7%9B%B8%E4%BA%A4_2.png" />
///
/// 面试题02.07.链表相交_2
///
/// 此时我们就可以比较curA和curB是否相同，如果不相同，同时向后移动curA和curB，如果遇到curA == curB，则找到交点。
///
/// 否则循环退出返回空指针。
pub fn get_intersection_node(
    head_a: Option<Box<ListNode<i32>>>,
    head_b: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    let mut cur_a = head_a;
    let mut cur_b = head_b;
    let (mut len_a, mut len_b) = (0, 0);
    while cur_a.is_some() {
        // 遍历 head_a链表，获取链表长度
        len_a += 1;
        cur_a = cur_a.unwrap().next;
    }

    while cur_b.is_some() {
        // 遍历 head_b链表，获取链表长度
        len_b += 1;
        cur_b = cur_b.unwrap().next;
    }

    // 如果head_a链表比head_b短，就交换一下他们的位置
    if len_a < len_b {
        let temp = len_a;
        len_a = len_b;
        len_b = temp;

        let cur_temp = cur_a;
        cur_a = cur_b;
        cur_b = cur_temp;
    }

    // 计算两个链表长度差
    let mut gap = len_a - len_b;
    while gap > 0 {
        // 移动head_a链表的指针，移动到和head_b对齐的位置
        gap -= 1;
        cur_a = cur_a.unwrap().next;
    }

    // 当head_a移动到和head_b对齐的位置之后，开始同时移动两个链表
    while cur_a.is_some() {
        // 如果出现节点相同则返回，遍历结束
        if cur_a==cur_b {
            return cur_a;
        }
        // 遍历head_a链表
        cur_a=cur_a.unwrap().next;
        // 遍历head_b链表
        cur_b=cur_b.unwrap().next;
    }

    // 如果没发现相同的节点则返回None
    None
}

/// 142.环形链表II
///
/// 题意： 给定一个链表，返回链表开始入环的第一个节点。 如果链表无环，则返回 null。
///
/// 为了表示给定链表中的环，使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。 如果 pos 是 -1，则在该链表中没有环。
///
/// 说明：不允许修改给定的链表。
/// 
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20200816110112704.png" />
///
/// #思路
/// 这道题目，不仅考察对链表的操作，而且还需要一些数学运算。
///
/// 主要考察两知识点：
///
/// 判断链表是否环
/// 如果有环，如何找到这个环的入口
/// #判断链表是否有环
/// 可以使用快慢指针法，分别定义 fast 和 slow 指针，从头结点出发，fast指针每次移动两个节点，slow指针每次移动一个节点，如果 fast 和 slow指针在途中相遇 ，说明这个链表有环。
///
/// 为什么fast 走两个节点，slow走一个节点，有环的话，一定会在环内相遇呢，而不是永远的错开呢
///
/// 首先第一点：fast指针一定先进入环中，如果fast指针和slow指针相遇的话，一定是在环中相遇，这是毋庸置疑的。
///
/// 那么来看一下，为什么fast指针和slow指针一定会相遇呢？
///
/// 可以画一个环，然后让 fast指针在任意一个节点开始追赶slow指针。
///
/// 会发现最终都是这种情况， 如下图：
/// 
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20210318162236720.png" />
///
/// fast和slow各自再走一步， fast和slow就相遇了
///
/// 这是因为fast是走两步，slow是走一步，其实相对于slow来说，fast是一个节点一个节点的靠近slow的，所以fast一定可以和slow重合。
///
/// 动画如下：
/// 
/// <img src="https://code-thinking.cdn.bcebos.com/gifs/141.%E7%8E%AF%E5%BD%A2%E9%93%BE%E8%A1%A8.gif" />
///
/// #如果有环，如何找到这个环的入口
/// 此时已经可以判断链表是否有环了，那么接下来要找这个环的入口了。
///
/// 假设从头结点到环形入口节点 的节点数为x。 环形入口节点到 fast指针与slow指针相遇节点 节点数为y。 从相遇节点 再到环形入口节点节点数为 z。 如图所示：
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20220925103433.png" />
///
/// 那么相遇时： slow指针走过的节点数为: x + y， fast指针走过的节点数：x + y + n (y + z)，n为fast指针在环内走了n圈才遇到slow指针， （y+z）为 一圈内节点的个数A。
///
/// 因为fast指针是一步走两个节点，slow指针一步走一个节点， 所以 fast指针走过的节点数 = slow指针走过的节点数 * 2：
///
/// (x + y) * 2 = x + y + n (y + z)
///
/// 两边消掉一个（x+y）: x + y = n (y + z)
///
/// 因为要找环形的入口，那么要求的是x，因为x表示 头结点到 环形入口节点的的距离。
///
/// 所以要求x ，将x单独放在左面：x = n (y + z) - y ,
///
/// 再从n(y+z)中提出一个 （y+z）来，整理公式之后为如下公式：x = (n - 1) (y + z) + z 注意这里n一定是大于等于1的，因为 fast指针至少要多走一圈才能相遇slow指针。
///
/// 这个公式说明什么呢？
///
/// 先拿n为1的情况来举例，意味着fast指针在环形里转了一圈之后，就遇到了 slow指针了。
///
/// 当 n为1的时候，公式就化解为 x = z，
///
/// 这就意味着，从头结点出发一个指针，从相遇节点 也出发一个指针，这两个指针每次只走一个节点， 那么当这两个指针相遇的时候就是 环形入口的节点。
///
/// 也就是在相遇节点处，定义一个指针index1，在头结点处定一个指针index2。
///
/// 让index1和index2同时移动，每次移动一个节点， 那么他们相遇的地方就是 环形入口的节点。
///
/// 动画如下：
/// 
/// <img src="https://code-thinking.cdn.bcebos.com/gifs/142.%E7%8E%AF%E5%BD%A2%E9%93%BE%E8%A1%A8II%EF%BC%88%E6%B1%82%E5%85%A5%E5%8F%A3%EF%BC%89.gif" />
///
/// 那么 n如果大于1是什么情况呢，就是fast指针在环形转n圈之后才遇到 slow指针。
///
/// 其实这种情况和n为1的时候 效果是一样的，一样可以通过这个方法找到 环形的入口节点，只不过，index1 指针在环里 多转了(n-1)圈，然后再遇到index2，相遇点依然是环形的入口节点。
///
///
/// #补充
/// 在推理过程中，大家可能有一个疑问就是：为什么第一次在环中相遇，slow的 步数 是 x+y 而不是 x + 若干环的长度 + y 呢？
///
/// 即文章链表：环找到了，那入口呢？ (opens new window)中如下的地方：
/// 
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20210318165123581.png" />
///
/// 首先slow进环的时候，fast一定是先进环来了。
///
/// 如果slow进环入口，fast也在环入口，那么把这个环展开成直线，就是如下图的样子：
/// 
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/2021031816503266.png" />
///
/// 可以看出如果slow 和 fast同时在环入口开始走，一定会在环入口3相遇，slow走了一圈，fast走了两圈。
///
/// 重点来了，slow进环的时候，fast一定是在环的任意一个位置，如图：
/// 
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/2021031816515727.png" />
///
/// 那么fast指针走到环入口3的时候，已经走了k + n 个节点，slow相应的应该走了(k + n) / 2 个节点。
///
/// 因为k是小于n的（图中可以看出），所以(k + n) / 2 一定小于n。
///
/// 也就是说slow一定没有走到环入口3，而fast已经到环入口3了。
///
/// 这说明什么呢？
///
/// 在slow开始走的那一环已经和fast相遇了。
///
/// 那有同学又说了，为什么fast不能跳过去呢？ 在刚刚已经说过一次了，fast相对于slow是一次移动一个节点，所以不可能跳过去。
///
/// 好了，这次把为什么第一次在环中相遇，slow的 步数 是 x+y 而不是 x + 若干环的长度 + y ，用数学推理了一下，算是对链表：环找到了，那入口呢？ (opens new window)的补充。
///
/// #总结
/// 这次可以说把环形链表这道题目的各个细节，完完整整的证明了一遍，说这是全网最详细讲解不为过吧。
#[allow(unused_variables)]
pub fn detect_cycle(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {

    None
}