use crate::list::ListNode;
/**
 * 283. 移动零

提示
给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。

请注意 ，必须在不复制数组的情况下原地对数组进行操作。



示例 1:

输入: nums = [0,1,0,3,12]
输出: [1,3,12,0,0]
示例 2:

输入: nums = [0]
输出: [0]


提示:

1 <= nums.length <= 104
-231 <= nums[i] <= 231 - 1


进阶：你能尽量减少完成的操作次数吗？

可以设置一个指针,就是专业收集不是零的数 收集一遍后,后面的一定是0,就再将空出来的位置设置为0,就解决问题了
 */
pub fn move_zeroes(nums: &mut Vec<i32>) {
    // 定义一个指针来记录非零元素的位置
    let mut slow = 0;
    // 开始收集不是零的数
    // [0,1,0,3,12]
    // --> [1,0,0,3,12]
    // --> [1,3,0,0,12]
    // --> [1,3,12,0,0]
    for fast in 0..nums.len() {
        if nums[fast] != 0 {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }

    // 对num中temp位置之前的数据进行排序
    nums[..slow].sort();

    // 收集完毕后,后面自然就都是0了
    for i in slow..nums.len() {
        nums[i] = 0;
    }
}

/**
 * 27. 移除元素
已解答
简单
相关标签
相关企业
提示
给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。

不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并 原地 修改输入数组。

元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。

 

说明:

为什么返回数值是整数，但输出的答案是数组呢?

请注意，输入数组是以「引用」方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。

你可以想象内部操作如下:

// nums 是以“引用”方式传递的。也就是说，不对实参作任何拷贝
int len = removeElement(nums, val);

// 在函数里修改输入数组对于调用者是可见的。
// 根据你的函数返回的长度, 它会打印出数组中 该长度范围内 的所有元素。
for (int i = 0; i < len; i++) {
    print(nums[i]);
}
 

示例 1：

输入：nums = [3,2,2,3], val = 3
输出：2, nums = [2,2]
解释：函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。你不需要考虑数组中超出新长度后面的元素。例如，函数返回的新长度为 2 ，而 nums = [2,2,3,3] 或 nums = [2,2,0,0]，也会被视作正确答案。
示例 2：

输入：nums = [0,1,2,2,3,0,4,2], val = 2
输出：5, nums = [0,1,3,0,4]
解释：函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4。注意这五个元素可为任意顺序。你不需要考虑数组中超出新长度后面的元素。
 

提示：

0 <= nums.length <= 100
0 <= nums[i] <= 50
0 <= val <= 100

双指针法

双指针法（快慢指针法）： 通过一个快指针和慢指针在一个for循环下完成两个for循环的工作。

定义快慢指针

快指针：寻找新数组的元素 ，新数组就是不含有目标元素的数组
慢指针：指向更新 新数组下标的位置
很多同学这道题目做的很懵，就是不理解 快慢指针究竟都是什么含义，所以一定要明确含义，后面的思路就更容易理解了。

很多同学不了解
双指针法（快慢指针法）在数组和链表的操作中是非常常见的，很多考察数组、链表、字符串等操作的面试题，都使用双指针法。
删除过程如下：
<img class="marble" src="https://code-thinking.cdn.bcebos.com/gifs/27.%E7%A7%BB%E9%99%A4%E5%85%83%E7%B4%A0-%E5%8F%8C%E6%8C%87%E9%92%88%E6%B3%95.gif" alt="image.png" style="zoom:50%;" />
 */
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // 定义一个指针用来存放非val的值
    let mut slow = 0;
    // 遍历数组，循环不变量为 slow - fast 之间的元素都不是val
    for fast in 0..nums.len() {
        if nums[fast] != val {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    // 返回temp位置的索引就是数组的长度
    slow as i32
}

/// 26. 删除有序数组中的重复项
/// 给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。
///
/// 考虑 nums 的唯一元素的数量为 k ，你需要做以下事情确保你的题解可以被通过：
///
/// 更改数组 nums ，使 nums 的前 k 个元素包含唯一元素，并按照它们最初在 nums 中出现的顺序排列。nums 的其余元素与 nums 的大小不重要。
/// 返回 k 。
///
/// 示例 1：
///
/// 输入：nums = [1,1,2]
/// 输出：2, nums = [1,2,_]
/// 解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。
/// 示例 2：
///
/// 输入：nums = [0,0,1,1,1,2,2,3,3,4]
/// 输出：5, nums = [0,1,2,3,4]
/// 解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。
///
///
/// 提示：
///
/// 1 <= nums.length <= 3 * 104
/// -104 <= nums[i] <= 104
/// nums 已按 非严格递增 排列
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // 定义一个指针用来存放非val的值
    let mut slow = 0;
    // 遍历数组
    for fast in 0..nums.len() {
        // 如果快指针和慢指针的值不相等，就将快指针的值赋值给慢指针
        if nums[fast] != nums[slow] {
            // 慢指针右移
            slow += 1;
            // 将快指针的值赋值给慢指针
            nums[slow] = nums[fast];
        }
    }
    // 返回slow位置的索引+1就是数组的长度
    slow as i32 + 1
}

/// 876. 链表的中间结点
///
/// 给你单链表的头结点 head ，请你找出并返回链表的中间结点。
///
/// 如果有两个中间结点，则返回第二个中间结点。
///
/// 示例 1：
///
/// 输入：head = [1,2,3,4,5]
///
/// 输出：[3,4,5]
///
/// 解释：链表只有一个中间结点，值为 3 。
///
/// 示例 2：
///
/// 输入：head = [1,2,3,4,5,6]
///
/// 输出：[4,5,6]
///
/// 解释：该链表有两个中间结点，值分别为 3 和 4 ，返回第二个结点。
///
/// 提示：
///
/// 链表的结点数范围是 [1, 100]
///
/// 1 <= Node.val <= 100
////
/// 解题思路
///
/// 考虑借助快慢双指针 fast, slow ，「快指针 fast」每轮走 2 步，「慢指针 slow」每轮走 1 步。fast 的步数恒为 slow 的 2 倍，
/// 因此当快指针遍历完链表时，慢指针就指向链表中间节点。而由于长度为偶数的链表有两个中间节点，因此需要分两种情况考虑：
///
/// 链表长度为奇数： 当 fast 走到链表「尾节点」时，slow 正好走到「中间节点」。
///
/// 链表长度为偶数： 当 fast 走到「null」时（越过「尾节点」后），slow 正好走到「第二个中间节点」。
///
/// 总结以上规律，应在当 fast 遇到或越过尾节点 时跳出循环，并返回 slow 即可。
///
/// <img src="https://pic.leetcode-cn.com/1656953441-Kshqch-figures.gif" alt="链表-删除节点" style="zoom:50%;" />
pub fn middle_node(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    // 快慢指针
    let mut slow = head.clone();
    let mut fast = head;
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = slow.unwrap().next;
        // 快指针走两步，慢指针走一步
        fast = fast.unwrap().next.unwrap().next;
    }
    slow
}

/// 674. 最长连续递增序列
///
/// 给定一个未经排序的整数数组，找到最长且 连续递增的子序列，并返回该序列的长度。
///
/// 连续递增的子序列 可以由两个下标 l 和 r（l < r）确定，如果对于每个 l <= i < r，都有 nums[i] < nums[i + 1] ，
/// 那么子序列 [nums[l], nums[l + 1], ..., nums[r - 1], nums[r]] 就是连续递增子序列。
///
/// 示例 1：
///
/// 输入：nums = [1,3,5,4,7]
///
/// 输出：3
///
/// 解释：最长连续递增序列是 [1,3,5], 长度为3。
///
/// 尽管 [1,3,5,7] 也是升序的子序列, 但它不是连续的，因为 5 和 7 在原数组里被 4 隔开。
///
/// 示例 2：
///
/// 输入：nums = [2,2,2,2,2]
///
/// 输出：1
///
/// 解释：最长连续递增序列是 [2], 长度为1。
///
/// 提示：
///
/// 1 <= nums.length <= 104
///
/// -109 <= nums[i] <= 109
pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    // 定义结果
    let mut max_len = 0;
    // 定义慢指针
    let mut slow = 0;
    // 定义快指针
    let mut fast = 0;
    // 循环不变量是 [slow,fast) 单调递增
    while fast < nums.len() {
        if fast > 0 && nums[fast - 1] >= nums[fast] {
            // 不满足单调递增条件，右移慢指针
            slow = fast;
        }
        // 快指针右移，由于是左闭右开区间，所以这里应该先右移再计算长度
        fast += 1;
        // 统计最大长度
        max_len = max_len.max(fast - slow);
    }
    max_len as i32
}

/// 80. 删除有序数组中的重复项 II
///
/// 给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使得出现次数超过两次的元素只出现两次 ，返回删除后数组的新长度。
///
/// 不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。
///
/// 说明：
///
/// 为什么返回数值是整数，但输出的答案是数组呢？
///
/// 请注意，输入数组是以「引用」方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。
///
/// 你可以想象内部操作如下:
///
/// nums 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
///
/// ```
/// int len = removeDuplicates(nums);
///
/// /// 在函数里修改输入数组对于调用者是可见的。
/// /// 根据你的函数返回的长度, 它会打印出数组中 该长度范围内 的所有元素。
/// for (int i = 0; i < len; i++) {
///     print(nums[i]);
/// }
/// ```
///
/// 示例 1：
///
/// 输入：nums = [1,1,1,2,2,3]
///
/// 输出：5, nums = [1,1,2,2,3]
///
/// 解释：函数应返回新长度 length = 5, 并且原数组的前五个元素被修改为 1, 1, 2, 2, 3。 不需要考虑数组中超出新长度后面的元素。
///
/// 示例 2：
///
/// 输入：nums = [0,0,1,1,1,1,2,3,3]
///
/// 输出：7, nums = [0,0,1,1,2,3,3]
///
/// 解释：函数应返回新长度 length = 7, 并且原数组的前七个元素被修改为 0, 0, 1, 1, 2, 3, 3。不需要考虑数组中超出新长度后面的元素。
///
/// 提示：
///
/// 1 <= nums.length <= 3 * 104
///
/// -104 <= nums[i] <= 104
///
/// nums 已按升序排列
pub fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32 {
    // 定义慢指针
    let mut flow = 0;
    // 遍历数组 , 循环不变量是 [0,len) 区间内的元素是符合条件的
    for fast in 0..nums.len() {
        // 如果慢指针小于2 或者 当前元素不等于慢指针前两个元素
        if flow < 2 || nums[fast] != nums[flow - 2] {
            // 交换数据
            nums[flow] = nums[fast];
            // 慢指针右移
            flow += 1;
        }
    }
    // 返回慢指针的位置
    flow as i32
}

/**
 * 2511. 最多可以摧毁的敌人城堡数目

给你一个长度为 n ，下标从 0 开始的整数数组 forts ，表示一些城堡。forts[i] 可以是 -1 ，0 或者 1 ，其中：

-1 表示第 i 个位置 没有 城堡。
0 表示第 i 个位置有一个 敌人 的城堡。
1 表示第 i 个位置有一个你控制的城堡。
现在，你需要决定，将你的军队从某个你控制的城堡位置 i 移动到一个空的位置 j ，满足：

0 <= i, j <= n - 1
军队经过的位置 只有 敌人的城堡。正式的，对于所有 min(i,j) < k < max(i,j) 的 k ，都满足 forts[k] == 0 。
当军队移动时，所有途中经过的敌人城堡都会被 摧毁 。

请你返回 最多 可以摧毁的敌人城堡数目。如果 无法 移动你的军队，或者没有你控制的城堡，请返回 0 。



示例 1：

输入：forts = [1,0,0,-1,0,0,0,0,1]
输出：4
解释：
- 将军队从位置 0 移动到位置 3 ，摧毁 2 个敌人城堡，位置分别在 1 和 2 。
- 将军队从位置 8 移动到位置 3 ，摧毁 4 个敌人城堡。
4 是最多可以摧毁的敌人城堡数目，所以我们返回 4 。
示例 2：

输入：forts = [0,0,1,-1]
输出：0
解释：由于无法摧毁敌人的城堡，所以返回 0 。


提示：

1 <= forts.length <= 1000
-1 <= forts[i] <= 1
 */
pub fn capture_forts(forts: Vec<i32>) -> i32 {
    // 定义慢指针
    let mut slow = 0;
    // 定义最大值
    let mut max = 0;
    for fast in 0..forts.len() {
        if forts[fast] == 1 || forts[fast] == -1 {
            // 当前0个数
            let mut cur = 0;
            // 判断不相等是因为 如果 slow指向的是1 那么 fast就需要指向-1 ，或 slow指向的是-1 那么 fast就需要指向1
            if forts[slow] != forts[fast] && forts[slow] != 0 && forts[fast] != 0 {
                // 统计slow指针和fast指针之间的0的个数
                forts[slow..fast].iter().for_each(|&x| {
                    if x == 0 {
                        cur += 1;
                    }
                });
                max = max.max(cur);
                // 计算完长度之后需要把slow指针移动到fast指针处，等待计算下一段满足条件的长度值
                slow = fast;
            } else if forts[slow] == forts[fast] && forts[slow] != 0 && forts[fast] != 0 {
                // 如果遇到 slow==1 fast==1 或 slow==-1 fast==-1 需要把slow指针移动到fast指针处，等待计算下一段满足条件的长度值
                slow = fast;
            } else if forts[slow] != forts[fast] && forts[slow] == 0 && forts[fast] == 1 {
                // 如果遇到 slow==0 fast==1 需要把slow指针移动到fast指针处，等待计算下一段满足条件的长度值
                slow = fast;
            }
        }
    }
    max as i32
}

/**
 * LCR 139. 训练计划 I

教练使用整数数组 actions 记录一系列核心肌群训练项目编号。
为增强训练趣味性，需要将所有奇数编号训练项目调整至偶数编号训练项目之前。
请将调整后的训练项目编号以 数组 形式返回。



示例 1：

输入：actions = [1,2,3,4,5]
输出：[1,3,5,2,4]
解释：为正确答案之一


提示：

0 <= actions.length <= 50000
0 <= actions[i] <= 10000

[1, 2, 3, 4, 5, 6, 7, 8, 9] -> [1, 3, 5, 7, 9, 2, 4, 6, 8]
 */
pub fn training_plan(actions: Vec<i32>) -> Vec<i32> {
    let mut actions = actions;
    // 定义慢指针
    let mut slow = 0;
    for fast in 0..actions.len() {
        // 如果发现fast指针指向的是奇数则向左移动这些奇数
        if actions[fast] % 2 != 0 {
            // 交换slow指针和fast指针指向的元素
            let temp = actions[slow];
            actions[slow] = actions[fast];
            actions[fast] = temp;
            slow += 1;
        }
    }
    actions
}

/**
 * 905. 按奇偶排序数组

给你一个整数数组 nums，将 nums 中的的所有偶数元素移动到数组的前面，后跟所有奇数元素。

返回满足此条件的 任一数组 作为答案。



示例 1：

输入：nums = [3,1,2,4]
输出：[2,4,3,1]
解释：[4,2,3,1]、[2,4,1,3] 和 [4,2,1,3] 也会被视作正确答案。
示例 2：

输入：nums = [0]
输出：[0]


提示：

1 <= nums.length <= 5000
0 <= nums[i] <= 5000
 */
pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    // 定义慢指针
    let mut slow = 0;
    for fast in 0..nums.len() {
        // 如果nums[fast]为偶数就和nums[slow]交换元素位置
        if nums[fast] % 2 == 0 {
            let temp = nums[slow];
            nums[slow] = nums[fast];
            nums[fast] = temp;
            slow += 1;
        }
    }
    nums
}

/**
 * LCR 180. 文件组合
简单
相关标签
相关企业
待传输文件被切分成多个部分，按照原排列顺序，每部分文件编号均为一个 正整数（至少含有两个文件）。
传输要求为：连续文件编号总和为接收方指定数字 target 的所有文件。请返回所有符合该要求的文件传输组合列表。

注意，返回时需遵循以下规则：

每种组合按照文件编号 升序 排列；
不同组合按照第一个文件编号 升序 排列。


示例 1：

输入：target = 12
输出：[[3, 4, 5]]
解释：在上述示例中，存在一个连续正整数序列的和为 12，为 [3, 4, 5]。
示例 2：

输入：target = 18
输出：[[3,4,5,6],[5,6,7]]
解释：在上述示例中，存在两个连续正整数序列的和分别为 18，分别为 [3, 4, 5, 6] 和 [5, 6, 7]。


提示：

1 <= target <= 10^5
 */
pub fn file_combination(target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut slow = 1;
    let mut sum = 0;
    for fast in 1..target {
        sum += fast;
        // 当sum超过target时，slow右移缩小窗口
        while slow < fast && sum > target {
            sum -= slow;
            slow += 1;
        }
        // 当sum等于target时，将slow到fast的连续正整数序列加入res
        if sum == target {
            let mut temp = vec![];
            for i in slow..=fast {
                temp.push(i);
            }
            res.push(temp);
        }
    }
    res
}

/**
 * 696. 计数二进制子串
简单
相关标签
相关企业
提示
给定一个字符串 s，统计并返回具有相同数量 0 和 1 的非空（连续）子字符串的数量，并且这些子字符串中的所有 0 和所有 1 都是成组连续的。

重复出现（不同位置）的子串也要统计它们出现的次数。


示例 1：

输入：s = "00110011"
输出：6
解释：6 个子串满足具有相同数量的连续 1 和 0 ："0011"、"01"、"1100"、"10"、"0011" 和 "01" 。
注意，一些重复出现的子串（不同位置）要统计它们出现的次数。
另外，"00110011" 不是有效的子串，因为所有的 0（还有 1 ）没有组合在一起。
示例 2：

输入：s = "10101"
输出：4
解释：有 4 个子串："10"、"01"、"10"、"01" ，具有相同数量的连续 1 和 0 。


提示：

1 <= s.length <= 105
s[i] 为 '0' 或 '1'
 */
pub fn count_binary_substrings(s: String) -> i32 {
    let mut res = 0;
    let mut pre = 0;
    let mut cur = 1;
    let s = s.as_bytes();
    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            cur += 1;
        } else {
            res += cur.min(pre);
            pre = cur;
            cur = 1;
        }
    }
    res + cur.min(pre)
}

/**
 * 2903. 找出满足差值条件的下标 I
简单
相关标签
相关企业
提示
给你一个下标从 0 开始、长度为 n 的整数数组 nums ，以及整数 indexDifference 和整数 valueDifference 。

你的任务是从范围 [0, n - 1] 内找出  2 个满足下述所有条件的下标 i 和 j ：

abs(i - j) >= indexDifference 且
abs(nums[i] - nums[j]) >= valueDifference
返回整数数组 answer。如果存在满足题目要求的两个下标，则 answer = [i, j] ；否则，answer = [-1, -1] 。
如果存在多组可供选择的下标对，只需要返回其中任意一组即可。

注意：i 和 j 可能 相等 。



示例 1：

输入：nums = [5,1,4,1], indexDifference = 2, valueDifference = 4
输出：[0,3]
解释：在示例中，可以选择 i = 0 和 j = 3 。
abs(0 - 3) >= 2 且 abs(nums[0] - nums[3]) >= 4 。
因此，[0,3] 是一个符合题目要求的答案。
[3,0] 也是符合题目要求的答案。
示例 2：

输入：nums = [2,1], indexDifference = 0, valueDifference = 0
输出：[0,0]
解释：
在示例中，可以选择 i = 0 和 j = 0 。
abs(0 - 0) >= 0 且 abs(nums[0] - nums[0]) >= 0 。
因此，[0,0] 是一个符合题目要求的答案。
[0,1]、[1,0] 和 [1,1] 也是符合题目要求的答案。
示例 3：

输入：nums = [1,2,3], indexDifference = 2, valueDifference = 4
输出：[-1,-1]
解释：在示例中，可以证明无法找出 2 个满足所有条件的下标。
因此，返回 [-1,-1] 。


提示：

1 <= n == nums.length <= 100
0 <= nums[i] <= 50
0 <= indexDifference <= 100
0 <= valueDifference <= 50

思路：
不妨设 i≤j−indexDifference。
类似 121. 买卖股票的最佳时机，我们可以在枚举 j 的同时，维护 nums[i] 的最大值 mx 和最小值 mn。

那么只要满足下面两个条件中的一个，就可以返回答案了。

mx − nums[j] ≥ valueDifference
nums[j] − mn ≥ valueDifference
代码实现时，可以维护最大值的下标 maxIdx\textit{maxIdx}maxIdx 和最小值的下标 minIdx。

答疑
问：为什么不用算绝对值？如果 mx<nums[j] 并且 ∣mx − nums[j]∣ ≥ valueDifference，不就错过答案了吗？
答：不会的，如果出现这种情况，那么一定会有 nums[j] − mn ≥ valueDifference。

 */
pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    // 维护最大值和最小值坐标
    let mut max_idx = 0;
    let mut min_idx = 0;
    for fast in index_difference as usize..nums.len() {
        let slow = (fast as i32 - index_difference) as usize;
        if nums[slow] > nums[max_idx] {
            max_idx = slow;
        } else if nums[slow] < nums[min_idx] {
            min_idx = slow;
        }
        if nums[max_idx] - nums[fast] >= value_difference {
            return vec![max_idx as i32, fast as i32];
        } else if nums[fast] - nums[min_idx] >= value_difference {
            return vec![min_idx as i32, fast as i32];
        }
    }
    vec![-1, -1]
}

/**
 * 287. 寻找重复数
中等
相关标签
相关企业
给定一个包含 n + 1 个整数的数组 nums ，其数字都在 [1, n] 范围内（包括 1 和 n），可知至少存在一个重复的整数。

假设 nums 只有 一个重复的整数 ，返回 这个重复的数 。

你设计的解决方案必须 不修改 数组 nums 且只用常量级 O(1) 的额外空间。



示例 1：

输入：nums = [1,3,4,2,2]
输出：2
示例 2：

输入：nums = [3,1,3,4,2]
输出：3
示例 3 :

输入：nums = [3,3,3,3,3]
输出：3




提示：

1 <= n <= 105
nums.length == n + 1
1 <= nums[i] <= n
nums 中 只有一个整数 出现 两次或多次 ，其余整数均只出现 一次


进阶：

如何证明 nums 中至少存在一个重复的数字?
你可以设计一个线性级时间复杂度 O(n) 的解决方案吗？
 */
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut slow = 0;
    let mut fast = 1;
    while fast < nums.len() {
        if nums[slow] == nums[fast] {
            return nums[slow];
        }
        slow += 1;
        fast += 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 12, 3];
        move_zeroes(&mut nums);
        println!("{:?}", nums)
    }

    #[test]
    fn test_remove_element() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let val = 0;
        let len = remove_element(&mut nums, val);
        println!("{:?}", len);
    }

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let len = remove_duplicates(&mut nums);
        println!("{:?}", nums);
        println!("{:?}", len);
    }

    #[test]
    fn test_middle_node() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        let res = middle_node(head);
        println!("{:?}", res)
    }

    #[test]
    fn test_find_length_of_lcis() {
        let nums = vec![1, 3, 5, 4, 7];
        let res = find_length_of_lcis(nums);
        println!("{:?}", res)
    }

    #[test]
    fn test_capture_forts() {
        let forts = vec![0, 0, 1, -1];
        let res = capture_forts(forts);
        println!("{:?}", res)
    }

    #[test]
    fn test_training_plan() {
        let actions = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let res = training_plan(actions);
        println!("{:?}", res)
    }

    #[test]
    fn test_sort_array_by_parity() {
        let nums = vec![3, 1, 2, 4];
        let res = sort_array_by_parity(nums);
        println!("{:?}", res)
    }

    #[test]
    fn test_file_combination() {
        let target = 12;
        let res = file_combination(target);
        assert_eq!(res, vec![vec![3, 4, 5]]);
    }

    #[test]
    fn test_count_binary_substrings() {
        let s = "00110011".to_string();
        let res = count_binary_substrings(s);
        assert_eq!(res, 6);
    }
}
