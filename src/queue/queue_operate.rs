use std::collections::{HashMap, VecDeque};

/// 239.滑动窗口最大值
///
/// 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
///
/// 返回 滑动窗口中的最大值 。
///
/// 示例 1：
///
/// 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
///
/// 输出：[3,3,5,5,6,7]
///
/// 解释：
///
/// ```
/// 滑动窗口的位置                最大值
/// ---------------               -----
/// [1  3  -1] -3  5  3  6  7       3
///  1 [3  -1  -3] 5  3  6  7       3
///  1  3 [-1  -3  5] 3  6  7       5
///  1  3  -1 [-3  5  3] 6  7       5
///  1  3  -1  -3 [5  3  6] 7       6
///  1  3  -1  -3  5 [3  6  7]      7
/// ```
/// 示例 2：
///
/// 输入：nums = [1], k = 1
///
/// 输出：[1]
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// -104 <= nums[i] <= 104
///
/// 1 <= k <= nums.length
///
/// 单调队列套路
///
/// 入（元素进入队尾，同时维护队列单调性）
///
/// 出（元素离开队首）
///
/// 记录/维护答案（根据队首）
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut ans = Vec::new();
    // 双端队列
    let mut q = VecDeque::new();
    for (i, &x) in nums.iter().enumerate() {
        // 1. 入
        // 如果队尾元素小于等于当前元素,则弹出队尾元素
        while !q.is_empty() && nums[*q.back().unwrap()] <= x {
            // 维护 q 的单调性
            q.pop_back();
        }
        // 入队
        q.push_back(i);
        // 2. 出，如果队列超出窗口大小,则弹出队首元素
        if i - q[0] >= k {
            // 队首已经离开窗口了
            q.pop_front();
        }
        // 3. 记录答案
        if i >= k - 1 {
            // 由于队首到队尾单调递减，所以窗口最大值就是队首
            ans.push(nums[q[0]]);
        }
    }
    ans
}

/// 073. 买票需要的时间
///
/// 有 n 个人前来排队买票，其中第 0 人站在队伍 最前方 ，第 (n - 1) 人站在队伍 最后方 。
///
/// 给你一个下标从 0 开始的整数数组 tickets ，数组长度为 n ，其中第 i 人想要购买的票数为 tickets[i] 。
///
/// 每个人买票都需要用掉 恰好 1 秒 。一个人 一次只能买一张票 ，如果需要购买更多票，他必须走到  队尾 重新排队（瞬间 发生，不计时间）。如果一个人没有剩下需要买的票，那他将会 离开 队伍。
///
/// 返回位于位置 k（下标从 0 开始）的人完成买票需要的时间（以秒为单位）。
///
/// ```
/// 示例 1：
///
/// 输入：tickets = [2,3,2], k = 2
/// 输出：6
/// 解释：
/// - 第一轮，队伍中的每个人都买到一张票，队伍变为 [1, 2, 1] 。
/// - 第二轮，队伍中的每个都又都买到一张票，队伍变为 [0, 1, 0] 。
/// 位置 2 的人成功买到 2 张票，用掉 3 + 3 = 6 秒。
/// 示例 2：
///
/// 输入：tickets = [5,1,1,1], k = 0
/// 输出：8
/// 解释：
/// - 第一轮，队伍中的每个人都买到一张票，队伍变为 [4, 0, 0, 0] 。
/// - 接下来的 4 轮，只有位置 0 的人在买票。
/// 位置 0 的人成功买到 5 张票，用掉 4 + 1 + 1 + 1 + 1 = 8 秒。
///
/// 提示：
///
/// n == tickets.length
/// 1 <= n <= 100
/// 1 <= tickets[i] <= 100
/// 0 <= k < n
/// ```
pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    // 定义一个数组来存储 下标和需要买的票数
    let mut cnt = Vec::<(i32, i32)>::new();
    for (idx, num) in tickets.iter().enumerate() {
        // 下标和需要买的票数
        cnt.push((idx as i32, *num as i32));
    }
    // 计时器
    let mut timer = 0;
    while !cnt.is_empty() {
        // 弹出头部第一个元素
        let mut frist = cnt.remove(0);
        // 购票数-1
        frist.1 -= 1;
        // 时间+1
        timer += 1;
        if frist.1 != 0 {
            // 如果还有需要购买的票数就重新入队
            cnt.push(frist);
        } else if frist.0 == k {
            // 如果k位置没有需要购买的票后退出循环
            break;
        }
    }
    // 返回时间
    timer
}

/// 073. 买票需要的时间
///
/// 有 n 个人前来排队买票，其中第 0 人站在队伍 最前方 ，第 (n - 1) 人站在队伍 最后方 。
///
/// 给你一个下标从 0 开始的整数数组 tickets ，数组长度为 n ，其中第 i 人想要购买的票数为 tickets[i] 。
///
/// 每个人买票都需要用掉 恰好 1 秒 。一个人 一次只能买一张票 ，如果需要购买更多票，他必须走到  队尾 重新排队（瞬间 发生，不计时间）。如果一个人没有剩下需要买的票，那他将会 离开 队伍。
///
/// 返回位于位置 k（下标从 0 开始）的人完成买票需要的时间（以秒为单位）。
///
/// ```
/// 示例 1：
///
/// 输入：tickets = [2,3,2], k = 2
/// 输出：6
/// 解释：
/// - 第一轮，队伍中的每个人都买到一张票，队伍变为 [1, 2, 1] 。
/// - 第二轮，队伍中的每个都又都买到一张票，队伍变为 [0, 1, 0] 。
/// 位置 2 的人成功买到 2 张票，用掉 3 + 3 = 6 秒。
/// 示例 2：
///
/// 输入：tickets = [5,1,1,1], k = 0
/// 输出：8
/// 解释：
/// - 第一轮，队伍中的每个人都买到一张票，队伍变为 [4, 0, 0, 0] 。
/// - 接下来的 4 轮，只有位置 0 的人在买票。
/// 位置 0 的人成功买到 5 张票，用掉 4 + 1 + 1 + 1 + 1 = 8 秒。
///
/// 提示：
///
/// n == tickets.length
/// 1 <= n <= 100
/// 1 <= tickets[i] <= 100
/// 0 <= k < n
/// ```
pub fn time_required_to_buy_ii(tickets: Vec<i32>, k: i32) -> i32 {
    // 定义计时器
    let mut timer = 0;
    // k位置需要购买的数量
    let m = tickets[k as usize];
    for (idx, v) in tickets.iter().enumerate() {
        if idx as i32 <= k {
            // 索引小于等于k的可以购买到m次
            timer += m.min(*v);
        } else {
            // 索引下标大于k的可以购买m-1次
            timer += (m - 1).min(*v)
        }
    }
    timer
}

/**
387. 字符串中的第一个唯一字符

给定一个字符串 s ，找到 它的第一个不重复的字符，并返回它的索引 。如果不存在，则返回 -1 。


```
示例 1：

输入: s = "leetcode"
输出: 0
示例 2:

输入: s = "loveleetcode"
输出: 2
示例 3:

输入: s = "aabb"
输出: -1


提示:

1 <= s.length <= 105
s 只包含小写字母
```
 */
pub fn first_uniq_char(s: String) -> i32 {
    let s = s.into_bytes();
    // 定义哈希表
    let mut cnt = vec![-2; 130];
    // 定义队列
    let mut queue = VecDeque::with_capacity(s.len());
    for (i, v) in s.iter().enumerate() {
        if cnt[*v as usize] == -2 {
            // 说明哈希表中还不存在 v，则把坐标放入哈希表中
            cnt[*v as usize] = i as i32;
            queue.push_back((*v, i));
        } else {
            // 如果v出现重复，则把哈希表值设值为-1
            cnt[*v as usize] = -1;
            while !queue.is_empty() && cnt[queue.front().unwrap().0 as usize] == -1 {
                // 把重复数据从队头全部弹出
                queue.pop_front();
            }
        }
    }
    if queue.is_empty() {
        -1
    } else {
        queue.pop_front().unwrap().1 as i32
    }
}

/**
LCR 169. 招式拆解 II

某套连招动作记作仅由小写字母组成的序列 arr，其中 arr[i] 第 i 个招式的名字。请返回第一个只出现一次的招式名称，如不存在请返回空格。

```
示例 1：

输入：arr = "abbccdeff"
输出：'a'
示例 2：

输入：arr = "ccdd"
输出：' '


限制：

0 <= arr.length <= 50000
```
 */
pub fn dismantling_action(arr: String) -> char {
    let s = arr.chars().collect::<Vec<char>>();
    // 定义哈希表来统计字符出现的次数
    let mut hash = HashMap::with_capacity(s.len());
    // 定义队列
    let mut queue = VecDeque::with_capacity(s.len());
    for c in s {
        if !hash.contains_key(&c) {
            // false 表示未重复
            hash.insert(c, false);
            // 入队
            queue.push_back(c);
        } else {
            // true 表示重复
            hash.insert(c, true);
            // 把队列中重复的数据全部弹出
            while !queue.is_empty() && *hash.get(&queue.front().unwrap()).unwrap() {
                // 从队首弹出
                queue.pop_front();
            }
        }
    }
    if queue.is_empty() {
        ' '
    } else {
        queue.pop_front().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let max_sliding_window = max_sliding_window(nums, k);
        println!("{:?}", max_sliding_window)
    }

    #[test]
    fn test_time_required_to_buy() {
        let tickets = vec![2, 3, 2];
        let k = 2;
        let time_required_to_buy = time_required_to_buy(tickets, k);
        println!("{:?}", time_required_to_buy)
    }

    #[test]
    fn test_time_required_to_buy_ii() {
        let tickets = vec![2, 3, 2];
        let k = 2;
        let time_required_to_buy = time_required_to_buy_ii(tickets, k);
        println!("{:?}", time_required_to_buy)
    }

    #[test]
    fn test_first_uniq_char() {
        let s = "leetcode".to_string();
        let first_uniq_char = first_uniq_char(s);
        assert_eq!(first_uniq_char, 0);
    }

    #[test]
    fn test_dismantling_action() {
        let arr = "abbccdeff".to_string();
        let dismantling_action = dismantling_action(arr);
        assert_eq!(dismantling_action, 'a');
    }
}
