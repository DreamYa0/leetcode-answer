use std::collections::{HashMap, VecDeque};

/// LCR 016. 无重复字符的最长子串
///
/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长连续子字符串 的长度。
///
/// 示例 1:
///
/// 输入: s = "abcabcbb"
/// 输出: 3
/// 解释: 因为无重复字符的最长子字符串是 "abc"，所以其长度为 3。
/// 示例 2:
///
/// 输入: s = "bbbbb"
/// 输出: 1
/// 解释: 因为无重复字符的最长子字符串是 "b"，所以其长度为 1。
/// 示例 3:
///
/// 输入: s = "pwwkew"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
///      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
/// 示例 4:
///
/// 输入: s = ""
/// 输出: 0
///
///
/// 提示：
///
/// 0 <= s.length <= 5 * 104
/// s 由英文字母、数字、符号和空格组成
///
///
/// 注意：本题与主站 3 题相同： https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
///
/// 复杂度分析
///
/// 时间复杂度：O(n)，其中 n 为 s 的长度。注意 left 至多增加 n 次，所以整个二重循环至多循环 O(n) 次。
///
/// 空间复杂度：O(∣Σ∣)，其中 ∣Σ∣ 为字符集合的大小，本题中字符均为 ASCII 字符，所以 ∣Σ∣≤128
pub fn length_of_longest_substring(st: String) -> i32 {
    let s = st.as_bytes();
    // 最长子串长度
    let mut ans = 0;
    // 起点指针
    let mut left = 0;
    // 用来标记窗口内是否存在重复元素，也可以用 HashSet，这里为了效率用的 Vec，数组长度定义128是因为u8的范围是0-127
    let mut window = vec![false; 128];
    for (right, &c) in s.iter().enumerate() {
        let c = c as usize;
        // 缩小窗口的条件是，窗口内存在重复元素
        while window[c] {
            // 把起点指针右移一位，并且把起点指针所在的元素从窗口中移除
            window[s[left] as usize] = false;
            left += 1;
        }

        // 往窗口中添加元素
        window[c] = true;
        // 更新窗口长度最大值
        ans = ans.max(right - left + 1);
    }

    ans as i32
}

/// 485. 最大连续 1 的个数
/// 给定一个二进制数组 nums ， 计算其中最大连续 1 的个数。
///
///
/// 示例 1：
///
/// 输入：nums = [1,1,0,1,1,1]
/// 输出：3
/// 解释：开头的两位和最后的三位都是连续 1 ，所以最大连续 1 的个数是 3.
/// 示例 2:
///
/// 输入：nums = [1,0,1,1,0,1]
/// 输出：2
///
/// 提示：
///
/// 1 <= nums.length <= 105
/// nums[i] 不是 0 就是 1.
///
/// 解题思路：滑动窗口
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    // 定义慢指针
    let mut slow = 0;
    // 最大连续 1 的个数。
    let mut max_len = 0;
    // 当前长度
    let mut cur_len = 0;
    for fast in 0..nums.len() {
        if nums[fast] == 1 {
            cur_len += 1;
        }

        while (slow < fast) && nums[fast] == 0 {
            slow += 1;
            cur_len -= 1;
        }
        max_len = max_len.max(cur_len);
    }
    max_len
}

/// 1438. 绝对差不超过限制的最长连续子数组
///
/// 给你一个整数数组 nums ，和一个表示限制的整数 limit，请你返回最长连续子数组的长度，该子数组中的任意两个元素之间的绝对差必须小于或者等于 limit 。
///
/// 如果不存在满足条件的子数组，则返回 0 。
///
/// 示例 1：
///
/// 输入：nums = [8,2,4,7], limit = 4
///
/// 输出：2
///
/// 解释：所有子数组如下：
///
/// ```
/// [8] 最大绝对差 |8-8| = 0 <= 4.
/// [8,2] 最大绝对差 |8-2| = 6 > 4.
/// [8,2,4] 最大绝对差 |8-2| = 6 > 4.
/// [8,2,4,7] 最大绝对差 |8-2| = 6 > 4.
/// [2] 最大绝对差 |2-2| = 0 <= 4.
/// [2,4] 最大绝对差 |2-4| = 2 <= 4.
/// [2,4,7] 最大绝对差 |2-7| = 5 > 4.
/// [4] 最大绝对差 |4-4| = 0 <= 4.
/// [4,7] 最大绝对差 |4-7| = 3 <= 4.
/// [7] 最大绝对差 |7-7| = 0 <= 4.
/// ```
///
/// 因此，满足题意的最长子数组的长度为 2 。
///
/// 示例 2：
///
/// 输入：nums = [10,1,2,4,7,2], limit = 5
///
/// 输出：4
///
/// 解释：满足题意的最长子数组是 [2,4,7,2]，其最大绝对差 |2-7| = 5 <= 5 。
///
/// 示例 3：
///
/// 输入：nums = [4,2,2,2,4,4,2,2], limit = 0
///
/// 输出：3
///
/// 提示：
///
/// 1 <= nums.length <= 10^5
///
/// 1 <= nums[i] <= 10^9
///
/// 0 <= limit <= 10^9
pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut max_q = VecDeque::new();
    let mut min_q = VecDeque::new();
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;
    while right < nums.len() {
        while !max_q.is_empty() && nums[right] > nums[*max_q.back().unwrap()] {
            max_q.pop_back();
        }
        while !min_q.is_empty() && nums[right] < nums[*min_q.back().unwrap()] {
            min_q.pop_back();
        }
        max_q.push_back(right);
        min_q.push_back(right);
        while nums[*max_q.front().unwrap()] - nums[*min_q.front().unwrap()] > limit {
            if *max_q.front().unwrap() < *min_q.front().unwrap() {
                left = max_q.pop_front().unwrap() + 1;
            } else {
                left = min_q.pop_front().unwrap() + 1;
            }
        }
        res = res.max(right - left + 1);
        right += 1;
    }
    res as i32
}

/// 718. 最长重复子数组
///
/// 给两个整数数组 nums1 和 nums2 ，返回 两个数组中 公共的 、长度最长的子数组的长度 。
///
/// 示例 1：
///
/// 输入：nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
///
/// 输出：3
///
/// 解释：长度最长的公共子数组是 [3,2,1] 。
///
/// 示例 2：
///
/// 输入：nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
/// 输出：5
///
/// 提示：
///
/// ```
/// 1 <= nums1.length, nums2.length <= 1000
/// 0 <= nums1[i], nums2[i] <= 100
/// ```
#[allow(unused)]
pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    i32::MIN
}

/// 替换后的最长重复字符
/// 给你一个字符串 s 和一个整数 k 。你可以选择字符串中的任一字符，并将其更改为任何其他大写英文字符。该操作最多可执行 k 次。
///
/// 在执行上述操作后，返回 包含相同字母的最长子字符串的长度。
///
/// 示例 1：
///
/// 输入：s = "ABAB", k = 2
///
/// 输出：4
///
/// 解释：用两个'A'替换为两个'B',反之亦然。
///
/// 示例 2：
///
/// 输入：s = "AABABBA", k = 1
///
/// 输出：4
///
/// 解释：
///
/// 将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
///
/// 子串 "BBBB" 有最长重复字母, 答案为 4。
///
/// 可能存在其他的方法来得到同样的结果。
///
/// 提示：
///
/// 1 <= s.length <= 105
/// s 仅由大写英文字母组成
/// 0 <= k <= s.length
///
/// 解题思路
///
/// 这个问题可以通过滑动窗口的方法来解决，目的是找到最长的子字符串，
/// 其中包含的相同字母数量加上最多 k 次字符更改操作，能够使得整个子字符串全部由相同的字母构成。
#[allow(unused)]
pub fn character_replacement(s: String, k: i32) -> i32 {
    // 循环不变量定义
    //
    // 在使用滑动窗口遍历字符串的过程中，我们定义以下循环不变量来帮助我们理解和保证算法的正确性：
    // 窗口内最频繁出现的字符的出现次数加上 k 应该大于等于窗口的大小。
    // 这意味着，我们可以通过最多 k 次操作，将窗口内的所有其他字符更改为窗口内出现次数最多的字符。
    //
    // 算法步骤与循环不变量的应用
    //
    // 1.初始化：设置两个指针 left = 0 和 right = 0 来表示滑动窗口的左右边界。
    // 使用一个数据结构（如哈希表）来记录窗口内每个字符的出现次数，以及一个变量来维护窗口内出现次数最多的字符的数量 maxCount。
    //
    // 2.保持：向右移动右指针 right 来扩大窗口，并更新窗口内字符的出现次数以及 maxCount。
    // 每次窗口更新后，检查是否可以通过最多 k 次操作将窗口内的所有字符更改为相同字符。
    // 如果可以，更新最长子字符串的长度。如果不可以，则向右移动左指针 left 来缩小窗口，直到窗口满足条件为止。
    //
    // 终止：当右指针遍历完字符串 s 时，算法结束。此时，我们已经考察了所有可能的子字符串，并找到了满足条件的最长子字符串。
    let mut left = 0;
    let mut right = 0;
    // 记录窗口内最大字符的数量
    let mut max_count = 0;
    // 定义字符频次哈希表
    let mut char_freq = vec![0; 26];
    let s = s.chars().collect::<Vec<char>>();
    while right < s.len() {
        // 计数
        char_freq[(s[right] as u8 - 'A' as u8) as usize] += 1;
        // 更新最大字符数量
        max_count = max_count.max(char_freq[(s[right] as u8 - 'A' as u8) as usize]);
        // 如果窗口内的字符数量加上 k 小于窗口的大小，说明窗口内的字符数量加上 k 无法满足窗口的大小
        if right - left + 1 - max_count > k as usize {
            // 减掉即将移出的字符计数
            char_freq[(s[left] as u8 - 'A' as u8) as usize] -= 1;
            // 移动左指针
            left += 1;
        }
        // 移动右指针
        right += 1;
    }
    (right - left) as i32
}

/// 删除子数组的最大得分
///
/// 给你一个正整数数组 nums ，请你从中删除一个含有 若干不同元素 的子数组。删除子数组的 得分 就是子数组各元素之 和 。
///
/// 返回 只删除一个 子数组可获得的 最大得分 。
///
/// 如果数组 b 是数组 a 的一个连续子序列，即如果它等于 a[l],a[l+1],...,a[r] ，那么它就是 a 的一个子数组。
///
/// 示例 1：
///
/// 输入：nums = [4,2,4,5,6]
///
/// 输出：17
///
/// 解释：最优子数组是 [2,4,5,6]
///
/// 示例 2：
///
/// 输入：nums = [5,2,1,2,5,2,1,2,5]
///
/// 输出：8
///
/// 解释：最优子数组是 [5,2,1] 或 [1,2,5]
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// 1 <= nums[i] <= 104
///
/// 解题思路
///
/// 这个问题可以通过动态规划（DP）或双指针加上一些辅助数据结构（如哈希集合）来解决，
/// 旨在找到一个含有若干不同元素的子数组，其和最大，然后从原数组中删除这个子数组。
/// 但考虑到问题的特性和求解过程的需要，我们可以采用一种“前缀和”和“后缀和”结合的方法来简化问题，并利用双指针技术高效寻找解。
/// 这个方法的核心在于，我们先计算每个位置为结束点的子数组最大得分（后缀和）和每个位置为起始点的子数组最大得分（前缀和），
/// 然后通过比较这些得分来找到最优解。
pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    // 定义左指针
    let mut left = 0;
    // 统计数组内元素的频次
    let mut counter = HashMap::new();
    // 当前和
    let mut cur_sum = 0;
    // 定义最大得分
    let mut max_sum = 0;

    for right in 0..nums.len() {
        // 统计元素出现的频次
        *counter.entry(nums[right]).or_insert(0) += 1;
        // 计算和
        cur_sum += nums[right];
        // 当大于1说明窗口内出现重复元素，就需要移动left指针来缩小窗口
        while *counter.get(&nums[right]).unwrap() > 1 {
            // 移动left指针之前需要把即将移出窗口的元素出现的频次减掉
            *counter.get_mut(&nums[left]).unwrap() -= 1;
            // 移动left指针之前需要把即将移出窗口的元素和减掉
            cur_sum -= nums[left];
            // 右移动左指针
            left += 1;
        }
        // 更新最大得分
        max_sum = max_sum.max(cur_sum);
    }
    max_sum
}

/// 最大连续1的个数 II
///
/// 给定一个二进制数组 nums ，如果最多可以翻转一个 0 ，则返回数组中连续 1 的最大个数。
///
/// 示例 1：
///
/// 输入：nums = [1,0,1,1,0]
///
/// 输出：4
///
/// 解释：翻转第一个 0 可以得到最长的连续 1。
///
/// 当翻转以后，最大连续 1 的个数为 4。
///
/// 示例 2:
///
/// 输入：nums = [1,0,1,1,0,1]
///
/// 输出：4
///
///
/// 提示:
///
/// 1 <= nums.length <= 105
///
/// nums[i] 不是 0 就是 1.
///
/// 进阶：如果输入的数字是作为 无限流 逐个输入如何处理？换句话说，内存不能存储下所有从流中输入的数字。您可以有效地解决吗？
pub fn find_max_consecutive_ones_ii(nums: Vec<i32>) -> i32 {
    // 循环不变量定义
    // 在滑动窗口的遍历过程中，我们定义以下循环不变量来帮助我们理解和保证算法的正确性：
    // 窗口内最多包含一个 0。这个循环不变量保证了在任何时刻，我们的窗口（考虑的子数组）要么只包含全 1，
    // 要么包含一个 0 和一些 1，这正好符合题目的要求。
    //
    // 算法步骤与循环不变量的应用
    //
    // 初始化：设置两个指针 left = 0 和 right = 0 来表示滑动窗口的左右边界。同时，维护一个计数器 zeroCount 来记录当前窗口内 0 的数量
    //
    // 保持：向右移动右指针 right 扩大窗口，并在每次移动时更新 zeroCount。如果 zeroCount 超过 1，
    // 则向右移动左指针 left 来缩小窗口，直到 zeroCount 再次等于 1 或小于 1。在这个过程中，我们保持循环不变量不变，即窗口内最多只有一个 0。
    //
    // 终止：当右指针遍历完整个数组时，算法结束。此时，我们已经考察了所有可能的窗口，找到了满足条件的最长窗口。
    //
    // 结果计算：通过维护窗口的最大长度来记录并最终返回数组中连续 1 的最大个数（包括最多翻转一个 0 的情况）。
    // 定义滑动窗口左边界指针
    let mut left = 0;
    // 可反转次数
    let mut k = 1;
    // 定义最大长度
    let mut max_len = 0;
    for right in 0..nums.len() {
        // 如果是0，就减少可反转次数
        if nums[right] == 0 {
            k -= 1;
        }
        while k < 0 {
            if nums[left] == 0 {
                // 如果窗口中的0滑出了窗口，这可反转计数需要加1
                k += 1;
            }
            left += 1;
        }
        max_len = max_len.max(right - left + 1);
    }
    max_len as i32
}

/// 最大连续 1 的个数 III
///
/// 给定一个二进制数组 nums 和一个整数 k，如果可以翻转最多 k 个 0 ，则返回 数组中连续 1 的最大个数 。
///
/// 示例 1：
///
/// 输入：nums = [1,1,1,0,0,0,1,1,1,1,0], K = 2
///
/// 输出：6
///
/// 解释：[1,1,1,0,0,1,1,1,1,1,1]
///
/// 粗体数字从 0 翻转到 1，最长的子数组长度为 6。
///
/// 示例 2：
///
/// 输入：nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], K = 3
///
/// 输出：10
///
/// 解释：[0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
///
/// 粗体数字从 0 翻转到 1，最长的子数组长度为 10。
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// nums[i] 不是 0 就是 1
///
/// 0 <= k <= nums.length
pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    // 可反转次数
    let mut k = k;
    // 定义最大长度
    let mut max_len = 0;
    for right in 0..nums.len() {
        // 如果是0，就减少可反转次数
        if nums[right] == 0 {
            k -= 1;
        }
        while k < 0 {
            if nums[left] == 0 {
                // 如果窗口中的0滑出了窗口，这可反转计数需要加1
                k += 1;
            }
            left += 1;
        }
        max_len = max_len.max(right - left + 1);
    }
    max_len as i32
}

/// 删掉一个元素以后全为 1 的最长子数组
///
/// 给你一个二进制数组 nums ，你需要从中删掉一个元素。
///
/// 请你在删掉元素的结果数组中，返回最长的且只包含 1 的非空子数组的长度。
///
/// 如果不存在这样的子数组，请返回 0 。
///
/// 提示 1：
///
/// 输入：nums = [1,1,0,1]
///
/// 输出：3
///
/// 解释：删掉位置 2 的数后，[1,1,1] 包含 3 个 1 。
///
/// 示例 2：
///
/// 输入：nums = [0,1,1,1,0,1,1,0,1]
///
/// 输出：5
///
/// 解释：删掉位置 4 的数字后，[0,1,1,1,1,1,0,1] 的最长全 1 子数组为 [1,1,1,1,1] 。
///
/// 示例 3：
///
/// 输入：nums = [1,1,1]
///
/// 输出：2
///
/// 解释：你必须要删除一个元素。
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// nums[i] 要么是 0 要么是 1 。
pub fn longest_subarray_ii(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    // 窗口中能包含0的最大个数
    let mut k = 1;
    // 定义最大长度
    let mut max_len = 0;
    for right in 0..nums.len() {
        // 如果是0，就减少可反转次数
        if nums[right] == 0 {
            k -= 1;
        }
        while k < 0 {
            if nums[left] == 0 {
                // 如果窗口中的0滑出了窗口，这可反转计数需要加1
                k += 1;
            }
            left += 1;
        }
        // 因为0是需要被删除的的，所以长度为right - left 而不是 right - left + 1
        max_len = max_len.max(right - left);
    }
    max_len as i32
}

/// 1208. 尽可能使字符串相等
///
/// 给你两个长度相同的字符串，s 和 t。
///
/// 将 s 中的第 i 个字符变到 t 中的第 i 个字符需要 |s[i] - t[i]| 的开销（开销可能为 0），也就是两个字符的 ASCII 码值的差的绝对值。
///
/// 用于变更字符串的最大预算是 maxCost。在转化字符串时，总开销应当小于等于该预算，这也意味着字符串的转化可能是不完全的。
///
/// 如果你可以将 s 的子字符串转化为它在 t 中对应的子字符串，则返回可以转化的最大长度。
///
/// 如果 s 中没有子字符串可以转化成 t 中对应的子字符串，则返回 0。
///
/// 示例 1：
///
/// 输入：s = "abcd", t = "bcdf", maxCost = 3
///
/// 输出：3
///
/// 解释：s 中的 "abc" 可以变为 "bcd"。开销为 3，所以最大长度为 3。
///
/// 示例 2：
///
/// 输入：s = "abcd", t = "cdef", maxCost = 3
///
/// 输出：1
///
/// 解释：s 中的任一字符要想变成 t 中对应的字符，其开销都是 2。因此，最大长度为 1。
///
/// 示例 3：
///
/// 输入：s = "abcd", t = "acde", maxCost = 0
///
/// 输出：1
///
/// 解释：a -> a, cost = 0，字符串未发生变化，所以最大长度为 1。
///
/// 提示：
///
/// 1 <= s.length, t.length <= 10^5
///
/// 0 <= maxCost <= 10^6
///
/// s 和 t 都只含小写英文字母。
///
/// 解题思路
///
/// 今天这个题目比较难理解，我需要再解释一下。
///
/// 两个长度相等字符串的 s 和 t ，把 i 位置的 s[i] 转成 t[i] 的开销是两者 ASCII 码之差的绝对值。
/// 题目给出了允许的最大预算 maxCost ，求不超过预算的情况下能够转换的最长子串。
/// 比如，对于 s = "abcd", t = "bcdf", cost = 3 而言，我们使用 costs[i] 表示从 s[i]  转成 t[i] 的开销，
/// 那么 costs = [1, 1, 1, 2] 。由于 maxCost = 3， 所以最多允许其前面三个字符进行转换。
///
/// <img src="https://pic.leetcode-cn.com/1612486268-iJXYou-file_1612486268430" />
///
/// 于是题目变成了：已知一个数组 costs ，求：和不超过 maxCost 时最长的子数组的长度。
/// 上面的表达方式跟题目是等价的。对题目抽象之后，是不是跟昨天的每日一题「643. 子数组最大平均数 I」非常像了？也和「424. 替换后的最长重复字符」非常像。
/// 这就是坚持刷每日一题的作用，大家继续坚持啊！
///
/// 抽象之后，我们知道这是一个区间题，求子区间经常使用的方法就是滑动窗口。
/// 我在「424. 替换后的最长重复字符」的题解中已经分享了我珍藏的滑动窗口模板，由于模板是通用的，因此我把当时的题解再拿过来分享给大家。
///
/// 《挑战程序设计竞赛》这本书中把滑动窗口叫做「虫取法」，我觉得非常生动形象。
/// 因为滑动窗口的两个指针移动的过程和虫子爬动的过程非常像：前脚不动，把后脚移动过来；后脚不动，把前脚向前移动。
///
/// 我分享一个滑动窗口的模板，能解决大多数的滑动窗口问题：
///
/// ```
/// def findSubArray(nums):
///     N = len(nums) # 数组/字符串长度
///     left, right = 0, 0 # 双指针，表示当前遍历的区间[left, right]，闭区间
///     sums = 0 # 用于统计 子数组/子区间 是否有效，根据题目可能会改成求和/计数
///     res = 0 # 保存最大的满足题目要求的 子数组/子串 长度
///     while right < N: # 当右边的指针没有搜索到 数组/字符串 的结尾
///         sums += nums[right] # 增加当前右边指针的数字/字符的求和/计数
///         while 区间[left, right]不符合题意：# 此时需要一直移动左指针，直至找到一个符合题意的区间
///             sums -= nums[left] # 移动左指针前需要从counter中减少left位置字符的求和/计数
///             left += 1 # 真正的移动左指针，注意不能跟上面一行代码写反
///         # 到 while 结束时，我们找到了一个符合题意要求的 子数组/子串
///         res = max(res, right - left + 1) # 需要更新结果
///         right += 1 # 移动右指针，去探索新的区间
///     return res
/// ```
/// 滑动窗口中用到了左右两个指针，它们移动的思路是：以右指针作为驱动，拖着左指针向前走。
/// 右指针每次只移动一步，而左指针在内部 while 循环中每次可能移动多步。
/// 右指针是主动前移，探索未知的新区域；左指针是被迫移动，负责寻找满足题意的区间。
///
/// 模板的整体思想是：
///
/// 定义两个指针 left 和 right 分别指向区间的开头和结尾，注意是闭区间；定义 sums 用来统计该区间内的各个字符出现次数；
///
/// 第一重 while 循环是为了判断 right 指针的位置是否超出了数组边界；当 right 每次到了新位置，需要增加 right 指针的求和/计数；
///
/// 第二重 while 循环是让 left 指针向右移动到 [left, right] 区间符合题意的位置；当 left 每次移动到了新位置，需要减少 left 指针的求和/计数；
///
/// 在第二重 while 循环之后，成功找到了一个符合题意的 [left, right] 区间，题目要求最大的区间长度，因此更新 res 为 max(res, 当前区间的长度) 。
///
/// right 指针每次向右移动一步，开始探索新的区间。
///
/// 模板中的 sums 需要根据题目意思具体去修改，本题是求和题目因此把sums 定义成整数用于求和；如果是计数题目，就需要改成字典用于计数。
/// 当左右指针发生变化的时候，都需要更新 sums 。
///
/// 另外一个需要根据题目去修改的是内层 while 循环的判断条件，即： 区间[left, right]不符合题意 。
/// 对于本题而言，就是该区内的和 sums 超过了 maxCost 。
///
/// 刷题心得
///
///读了题目之后，要反应过来这是求一个最长区间的问题，从而想到滑动窗口。
///
/// 滑动窗口是有模板的，理解之后，形成肌肉记忆，下次直接敲出来。
///
/// 坚持刷每日一题，会发现自己在潜移默化中进步。
pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let n = s.len();
    // 计算两个字符串的差值
    let diff = s
        .bytes()
        .zip(t.bytes())
        .map(|(a, b)| (a as i32 - b as i32).abs())
        .collect::<Vec<_>>();
    let (mut l, mut r) = (0, 0);
    let mut cost = 0;
    let mut res = 0;
    while r < n {
        cost += diff[r];
        if cost > max_cost {
            cost -= diff[l];
            l += 1;
        } else {
            res = res.max(r - l + 1);
        }
        r += 1;
    }
    res as i32
}

/// 978. 最长湍流子数组
///
/// 给定一个整数数组 arr ，返回 arr 的 最大湍流子数组的长度 。
///
/// 如果比较符号在子数组中的每个相邻元素对之间翻转，则该子数组是 湍流子数组 。
///
/// ```
/// 更正式地来说，当 arr 的子数组 A[i], A[i+1], ..., A[j] 满足仅满足下列条件时，我们称其为湍流子数组：
/// 若 i <= k < j ：
/// 当 k 为奇数时， A[k] > A[k+1]，且
/// 当 k 为偶数时，A[k] < A[k+1]；
/// 或 若 i <= k < j ：
/// 当 k 为偶数时，A[k] > A[k+1] ，且
/// 当 k 为奇数时， A[k] < A[k+1]。
///
/// 示例 1：
/// 输入：arr = [9,4,2,10,7,8,8,1,9]
/// 输出：5
/// 解释：arr[1] > arr[2] < arr[3] > arr[4] < arr[5]
///
/// 示例 2：
/// 输入：arr = [4,8,12,16]
/// 输出：2
///
/// 示例 3：
/// 输入：arr = [100]
/// 输出：1
///
/// 提示：
/// 1 <= arr.length <= 4 * 104
/// 0 <= arr[i] <= 109
/// ```
///
/// 解题思路
///
/// 首先我们一定要读懂题意，本题中湍流子数组的意思是：一个增长和降低互相交替的子数组，如果在坐标轴上画出来就是个波浪状数组，↗ ↘ ↗ ↘，即这个形状。
///
/// 比如，题目给的示例 1 中的最长湍流子数组为 [4,2,10,7,8]，他就是增长和降低相互交替的，形状是↘ ↗ ↘ ↗。
///
/// 动态规划
///
/// 今天这个题目最合适的做法是动态规划。下面的解释不难，相信你可以看懂；如果有疑问就在评论区提问，我会及时解答。
///
/// 动态规划首先需要我们定义状态是什么，然后根据题意，写出状态转移方程。
///
/// 对于最长连续子数组问题，使用动态规划求解时，我们经常定义状态 dp[i] 为：以 i 位置结尾的最长连续子数组的长度，
/// 因为这个状态可以反映 i 位置及其前面区间的情况。下一个位置 i + 1 可以根据 dp[i] 就知道了前面的情况，
/// 再根据 arr[i + 1] 和 arr[i] 的大小关系，能更新状态 dp[i + 1]。
///
/// 对于本题，如果只定一个状态数组是不够的，因为我们只有区分了 i 位置是在增长还是在降低，才能判断 i + 1 位置是否能续上前面的波浪。
/// 所以，我们需要定义两个状态数组，分别表示以 i 结尾的在增长和降低的最长湍流子数组长度。
///
/// 状态的定义：
///
/// ```
/// 定义 up[i] 表示以位置 i 结尾的，并且 arr[i - 1] < arr[i] 的最长湍流子数组长度。
///
/// 定义 down[i] 表示以位置 i 结尾的，并且 arr[i - 1] > arr[i] 的最长湍流子数组长度。
///
/// up[i] 和 down[i] 初始化都是 1，因为每个数字本身都是一个最小的湍流子数组。
/// ```
///
/// 状态转移方程：
///
/// ```
/// up[i] = down[i - 1] + 1，当 arr[i - 1] < arr[i]；
///
/// down[i] = up[i - 1] + 1，当 arr[i - 1] > arr[i]；
/// ```
///
/// 解释：湍流子数组的增长和降低是交替的。
///
/// 文字的解释会显得苍白和啰嗦，大家直接看图吧。
///
/// <img src="https://pic.leetcode-cn.com/1612746849-AHrNUH-978.gif" />
///
/// 除了动态规划之外，本题还可以用双指针求解。大家可以参考官方题解。
pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut op = vec![0; n];
    for i in 1..n {
        let cmp = arr[i] - arr[i - 1];
        op[i] = if cmp > 0 {
            1
        } else if cmp < 0 {
            -1
        } else {
            0
        };
    }
    let mut answer = 0;
    let mut max = 0;
    for i in 1..n {
        if op[i] != 0 {
            if op[i] != op[i - 1] {
                max += 1;
            } else {
                max = 1;
            }
        } else {
            max = 0;
        }
        answer = answer.max(max);
    }
    answer + 1
}

/// 159. 至多包含两个不同字符的最长子串
///
/// 给你一个字符串 s ，请你找出 至多 包含 两个不同字符 的最长子串，并返回该子串的长度。
///
/// 示例 1：
///
/// 输入：s = "eceba"
///
/// 输出：3
///
/// 解释：满足题目要求的子串是 "ece" ，长度为 3 。
///
/// 示例 2：
///
/// 输入：s = "ccaabbb"
///
/// 输出：5
///
/// 解释：满足题目要求的子串是 "aabbb" ，长度为 5 。
///
/// 提示：
///
/// 1 <= s.length <= 105
///
/// s 由英文字母组成
pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    // 长度小于3直接返回
    if s.len() < 3 {
        return s.len() as i32;
    }

    // 转换为字节数组
    let s = s.as_bytes();
    // 定义hash表来统计s中字符出现的次数
    let mut cnt = vec![0; 128];
    // 计数
    let mut count = 0;
    // 定义最大长度
    let mut max_len = 2;
    // 定义左指针
    let mut left = 0;
    for right in 0..s.len() {
        // 字符计数
        cnt[s[right] as usize] += 1;
        // 如果是第一次出现的字符，计数加1
        if cnt[s[right] as usize] == 1 {
            count += 1;
        }
        // 如果字符数量大于2，就需要移动左指针来缩小窗口
        while count == 3 {
            // 扣减计数
            cnt[s[left] as usize] -= 1;
            // 如果字符未出现，计数减1
            if cnt[s[left] as usize] == 0 {
                count -= 1;
            }
            // 右移动左指针
            left += 1;
        }
        // 更新最大长度
        max_len = max_len.max(right - left + 1);
    }
    max_len as i32
}

/// 340. 至多包含 K 个不同字符的最长子串
///
/// 给你一个字符串 s 和一个整数 k ，请你找出 至多 包含 k 个 不同 字符的最长子串，并返回该子串的长度。
///
/// 示例 1：
///
/// 输入：s = "eceba", k = 2
///
/// 输出：3
///
/// 解释：满足题目要求的子串是 "ece" ，长度为 3 。
///
/// 示例 2：
///
/// 输入：s = "aa", k = 1
///
/// 输出：2
///
/// 解释：满足题目要求的子串是 "aa" ，长度为 2 。
///
/// 提示：
///
/// 1 <= s.length <= 5 * 104
///
/// 0 <= k <= 50
pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
    if k == 0 {
        return 0;
    }

    // 转换为字节数组
    let s = s.as_bytes();
    // 定义hash表来统计s中字符出现的次数
    let mut cnt = vec![0; 128];
    // 计数
    let mut count = 0;
    // 定义最大长度
    let mut max_len = 0;
    // 定义左指针
    let mut left = 0;
    for right in 0..s.len() {
        // 字符计数
        cnt[s[right] as usize] += 1;
        // 如果是第一次出现的字符，计数加1
        if cnt[s[right] as usize] == 1 {
            count += 1;
        }
        // 如果字符数量大于2，就需要移动左指针来缩小窗口
        while count == k + 1 {
            // 扣减计数
            cnt[s[left] as usize] -= 1;
            // 如果字符未出现，计数减1
            if cnt[s[left] as usize] == 0 {
                count -= 1;
            }
            // 右移动左指针
            left += 1;
        }
        // 更新最大长度
        max_len = max_len.max(right - left + 1);
    }
    max_len as i32
}

/// 2730. 找到最长的半重复子字符串
pub fn longest_semi_repetitive_substring(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    // 定义慢指针
    let mut slow = 0;
    // 字符串 s 中相邻字符是相等的的有多少对
    let mut k = 0;
    // 定义结果,初始化为1是因为当只有一个字符串的时候返回1
    let mut ans = 1;
    // 遍历字符数组
    for fast in 1..s.len() {
        if s[fast] == s[fast - 1] {
            // 移动右指针 fast，并统计相邻相同的情况出现了多少次
            k += 1;
            // 如果k大于1，慢指针右移
            if k > 1 {
                // 慢指针右移
                slow += 1;
                // 如果 k>1，则不断移动左指针 slow 直到 s[slow]=s[slow-1]，此时将一对相同的字符移到窗口之外。然后将 k 置为 1
                while s[slow] != s[slow - 1] {
                    slow += 1;
                }
                k = 1;
            }
        }
        // 然后统计子串长度 right−left+1 的最大值
        ans = ans.max(fast - slow + 1);
    }
    ans as i32
}

/// 904. 水果成篮
///
/// 此题的本质其实是求最长的连续子数组，使得子数组中最多包含两个不同的元素。
///
/// 最大滑窗模板：给定数组 nums，定义滑窗的左右边界 i, j，求满足某个条件的滑窗的最大长度。
///
/// ```
/// while j < len(nums):
///    判断[i, j]是否满足条件
///    while 不满足条件：
///        i += 1 （最保守的压缩i，一旦满足条件了就退出压缩i的过程，使得滑窗尽可能的大）
///    不断更新结果（注意在while外更新！）
///    j += 1
/// ```
///
/// 最小滑窗模板：给定数组 nums，定义滑窗的左右边界 i, j，求满足某个条件的滑窗的最小长度。
///
/// ```
/// while j < len(nums):
///    判断[i, j]是否满足条件
///    while 满足条件：
///        不断更新结果(注意在while内更新！)
///        i += 1 （最大程度的压缩i，使得滑窗尽可能的小）
///    j += 1
/// ```
pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    // 定义哈希表来存储水果种类和出现次数 K为水果种类，V为出现次数
    let mut hash = HashMap::<i32, i32>::new();
    // 定义慢指针
    let mut slow = 0;
    // 定义结果,初始化为1是因为当fruits的长度为1的时候返回1
    let mut ans = 1;
    for fast in 0..fruits.len() {
        hash.insert(
            fruits[fast],
            hash.get(&fruits[fast]).or(Some(&0)).unwrap() + 1,
        );
        while hash.keys().len() > 2 {
            if *hash.get(&fruits[slow]).unwrap() == 1 {
                hash.remove(&fruits[slow]);
            } else {
                hash.insert(fruits[slow], hash.get(&fruits[slow]).unwrap() - 1);
            }
            slow += 1;
        }
        ans = ans.max(fast - slow + 1);
    }
    ans as i32
}

/// 2958. 最多 K 个重复元素的最长子数组
pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    // 定义哈希表来存储某个数和出现次数 K为数，V为出现次数
    let mut hash = HashMap::<i32, i32>::new();
    // 定义慢指针
    let mut slow = 0;
    // 定义结果,初始化为1是因为当fruits的长度为1的时候返回1
    let mut ans = 1;
    for fast in 0..nums.len() {
        hash.insert(nums[fast], hash.get(&nums[fast]).or(Some(&0)).unwrap() + 1);

        while *hash.get(&nums[fast]).unwrap() > k {
            // 如果某一数到频次超过了k，则把slow指针位置的数据移除,直到移除到nums[fast]的频次小于等于k为止
            hash.insert(nums[slow], hash.get(&nums[slow]).unwrap() - 1);
            slow += 1;
        }
        ans = ans.max(fast - slow + 1);
    }
    ans as i32
}

/// 2024. 考试的最大困扰度
pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let s = answer_key.chars().collect::<Vec<char>>();
    // 定义T出现的次数
    let mut t = 0;
    // 定义F出现的次数
    let mut f = 0;
    // 定义慢指针
    let mut slow = 0;
    // 最大 连续 'T' 或者 'F' 的数目
    let mut ans = 0;
    for fast in 0..s.len() {
        if s[fast] == 'T' {
            t += 1;
        } else {
            f += 1;
        }
        // 如果T和F出现的次数最小的都超过了K，就需要移动slow指针
        while t.min(f) > k {
            if s[slow] == 'T' {
                t -= 1;
            } else {
                f -= 1;
            }
            slow += 1;
        }
        ans = ans.max(fast - slow + 1);
    }
    ans as i32
}

/// 1004. 最大连续1的个数 III
/// https://leetcode.cn/problems/max-consecutive-ones-iii/
pub fn longest_ones_ii(nums: Vec<i32>, k: i32) -> i32 {
    // 数组中连续 1 的最大个数
    let mut ans = 0;
    // 记录0的个数
    let mut zero = 0;
    // 定义慢指针
    let mut slow = 0;
    for fast in 0..nums.len() {
        if nums[fast] == 0 {
            zero += 1;
        }
        while zero > k {
            // 如果零的个数大于k，就需要右移slow指针
            if nums[slow] == 0 {
                // 零被移出去了
                zero -= 1;
            }
            // slow指针右移
            slow += 1;
        }
        // 记录大小
        ans = ans.max(fast - slow + 1);
    }
    ans as i32
}

/// 2401. 最长优雅子数组
/// https://leetcode.cn/problems/longest-nice-subarray/
/// 进一步地，既然这些数对应的集合的交集为空，我们可以用滑动窗口优化上述过程，
/// 右边加入 nums[right]，左边移出 nums[left]。
/// 如果 or 与新加入的 nums[right] 有交集，则不断从 or 中去掉集合 nums[left]，直到 or 与 nums[right] 交集为空。
pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    // 最长 的优雅子数组的长度
    let mut ans = 0;
    // 定义慢指针
    let mut slow = 0;
    let mut or = 0;
    for (fast, v) in nums.iter().enumerate() {
        // 有交集
        while (or & v) > 0 {
            // 从 or_ 中去掉集合 nums[left]
            or ^= nums[slow];
            slow += 1;
        }
        // 把集合 x 并入 or_ 中
        or |= v;
        ans = ans.max(fast - slow + 1);
    }
    ans as i32
}

/// 594. 最长和谐子序列
///
/// https://leetcode.cn/problems/longest-harmonious-subsequence/
/// 
/// 不定长滑动窗口（求最长/最大）
pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    // 此题是求长度，虽然排序后元素顺序发生变化，但是对最终结果没有影响
    nums.sort();
    // 最长的和谐子序列的长度
    let mut ans = 0;
    // 定义 slow指针
    let mut slow = 0;
    for fast in 1..nums.len() {
        while slow < fast && nums[fast] - nums[slow] > 1 {
            slow += 1;
        }
        if nums[fast] - nums[slow] == 1 {
            ans = ans.max(fast - slow + 1);
        }
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let st = "pwwkew".to_string();
        let length = length_of_longest_substring(st);
        println!("{:?}", length)
    }

    #[test]
    fn test_find_max_consecutive_ones() {
        let nums = [1, 0, 1, 1, 0, 1].to_vec();
        let find_max_consecutive_ones = find_max_consecutive_ones(nums);
        println!("{:?}", find_max_consecutive_ones)
    }

    #[test]
    fn test_longest_subarray() {
        let nums = vec![8, 2, 4, 7];
        let limit = 4;
        let longest_subarray = longest_subarray(nums, limit);
        println!("{:?}", longest_subarray)
    }

    #[test]
    fn test_character_replacement() {
        let s = "AABABBA".to_string();
        let k = 1;
        let character_replacement = character_replacement(s, k);
        println!("{:?}", character_replacement)
    }

    #[test]
    fn test_find_max_consecutive_ones_ii() {
        let nums = vec![1, 0, 1, 1, 0];
        let find_max_consecutive_ones = find_max_consecutive_ones_ii(nums);
        println!("{:?}", find_max_consecutive_ones)
    }

    #[test]
    fn test_longest_ones() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let k = 2;
        let longest_ones = longest_ones(nums, k);
        println!("{:?}", longest_ones)
    }

    #[test]
    fn test_longest_subarray_ii() {
        let nums = vec![1, 1, 0, 1];
        let longest_subarray_ii = longest_subarray_ii(nums);
        println!("{:?}", longest_subarray_ii)
    }

    #[test]
    fn test_equal_substring() {
        let s = "abcd".to_string();
        let t = "bcdf".to_string();
        let max_cost = 3;
        let equal_substring = equal_substring(s, t, max_cost);
        println!("{:?}", equal_substring)
    }

    #[test]
    fn test_max_turbulence_size() {
        let arr = vec![9, 4, 2, 10, 7, 8, 8, 1, 9];
        let max_turbulence_size = max_turbulence_size(arr);
        println!("{:?}", max_turbulence_size)
    }

    #[test]
    fn test_length_of_longest_substring_two_distinct() {
        let s = "eceba".to_string();
        let num = length_of_longest_substring_two_distinct(s);
        println!("{:?}", num)
    }

    #[test]
    fn test_length_of_longest_substring_k_distinct() {
        let s = "ab".to_string();
        let k = 1;
        let num = length_of_longest_substring_k_distinct(s, k);
        println!("{:?}", num)
    }

    #[test]
    fn test_longest_semi_repetitive_substring() {
        let s = "52233".to_string();
        let longest = longest_semi_repetitive_substring(s);
        assert_eq!(longest, 4);
    }

    #[test]
    fn test_total_fruit() {
        let fruits = vec![1, 2, 1];
        let total_fruit = total_fruit(fruits);
        assert_eq!(total_fruit, 3);
    }

    #[test]
    fn test_max_subarray_length() {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
        let k = 2;
        let max_subarray_length = max_subarray_length(nums, k);
        assert_eq!(max_subarray_length, 6);
    }

    #[test]
    fn test_max_consecutive_answers() {
        let answer_key = "TTFF".to_string();
        let k = 2;
        let max_consecutive_answers = max_consecutive_answers(answer_key, k);
        assert_eq!(max_consecutive_answers, 4);
    }

    #[test]
    fn test_find_lhs() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        let find_lhs = find_lhs(nums);
        assert_eq!(find_lhs, 5);
    }
}