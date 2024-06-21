use std::cmp::{max, min};

pub mod diff_array;
pub mod prefix_and_hash;
pub mod prefix_and_suffix;
pub mod prefix_pro;
pub mod prefix_xor;

/// 53. 最大子数组和
/// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
///
/// 子数组 是数组中的一个连续部分。
///
/// 示例 1：
///
/// 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
/// 输出：6
/// 解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
/// 示例 2：
///
/// 输入：nums = [1]
/// 输出：1
/// 示例 3：
///
/// 输入：nums = [5,4,-1,7,8]
/// 输出：23
///
/// 提示：
///
/// 1 <= nums.length <= 105
/// -104 <= nums[i] <= 104
///
/// 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
///
/// 前缀和解法
/// 思路
/// 由于子数组的元素和等于两个前缀和的差，所以求出 nums\textit{nums}nums 的前缀和，问题就变成 121. 买卖股票的最佳时机 了。本题子数组不能为空，相当于一定要交易一次。
///
/// 我们可以一边遍历数组计算前缀和，一边维护前缀和的最小值（相当于股票最低价格），用当前的前缀和（卖出价格）减去前缀和的最小值（买入价格），就得到了以当前元素结尾的子数组和的最大值（利润），用它来更新答案的最大值（最大利润）。
///
/// 请注意，由于题目要求子数组不能为空，应当先计算前缀和-最小前缀和，再更新最小前缀和。相当于不能在同一天买入股票又卖出股票。
///
/// 如果先更新最小前缀和，再计算前缀和-最小前缀和，就会把空数组的元素和 000 算入答案。
///
/// 示例 1 [−2,1,−3,4,−1,2,1,−5,4] 的计算流程如下：
///
/// i	前缀和	最小前缀和	前缀和-最小前缀和
///
/// 0	−2	0	−2
///
/// 1	−1	−2	1
///
/// 2	−4	−2	−2
///
/// 3	0	−4	4
///
/// 4	−1	−4	3
///
/// 5	1	−4	5
///
/// 6	2	−4	6
///
/// 7	−3	−4	1
///
/// 8	1	−4	5
///
/// 前缀和-最小前缀和的最大值等于 6，即为答案。
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // 定义结果
    let mut res = nums[0];
    // 定义当前和
    let mut cur_sum = 0;
    // 最小前缀和
    let mut min_sum = 0;
    // 遍历数组
    for num in nums {
        // 计算当前和
        cur_sum += num;
        // 更新结果
        res = max(res, cur_sum - min_sum);
        // 更新最小前缀和
        min_sum = min(min_sum, cur_sum);
    }
    res
}

/**
918. 环形子数组的最大和

给定一个长度为 n 的环形整数数组 nums ，返回 nums 的非空 子数组 的最大可能和 。

环形数组 意味着数组的末端将会与开头相连呈环状。形式上， nums[i] 的下一个元素是 nums[(i + 1) % n] ， nums[i] 的前一个元素是 nums[(i - 1 + n) % n] 。

子数组 最多只能包含固定缓冲区 nums 中的每个元素一次。形式上，对于子数组 nums[i], nums[i + 1], ..., nums[j] ，不存在 i <= k1, k2 <= j 其中 k1 % n == k2 % n 。


```
示例 1：

输入：nums = [1,-2,3,-2]
输出：3
解释：从子数组 [3] 得到最大和 3
示例 2：

输入：nums = [5,-3,5]
输出：10
解释：从子数组 [5,5] 得到最大和 5 + 5 = 10
示例 3：

输入：nums = [3,-2,2,-3]
输出：3
解释：从子数组 [3] 和 [3,-2,2] 都可以得到最大和 3


提示：

n == nums.length
1 <= n <= 3 * 104
-3 * 104 <= nums[i] <= 3 * 104
```

解题思路

前置题目：53. 最大子数组和

<img src="https://pic.leetcode.cn/1689750394-drKSAI-lc918-c.png" alt="image.png" style="zoom:50%;" />

答疑
问：为什么当 minS=sum(nums) 时，最小子数组可以是整个数组？

答：用反证法证明。假设最小子数组一定不是整个数组，这意味着 nums 的某个前缀或者后缀是大于 000 的（包含这个前缀/后缀会让 minS 变大），
所以 minS<sum(nums)，矛盾。所以当 minS=sum(nums) 时，最小子数组可以是整个数组。

注：对于 nums=[−1,1,−1]，最小子数组可以取 [−1]，也可以取整个数组 [−1,1,−1]。对于这样的 nums，
最大子数组一定不会跨过边界，只返回 maxS 仍然是正确的。
​​​​​​​
 */
pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut min_sum = nums[0];
    let mut cur_max = 0;
    let mut cur_min = 0;
    let mut total = 0;
    for num in nums {
        cur_max = max(cur_max + num, num);
        max_sum = max(max_sum, cur_max);
        cur_min = min(cur_min + num, num);
        min_sum = min(min_sum, cur_min);
        total += num;
    }
    if max_sum > 0 {
        max(max_sum, total - min_sum)
    } else {
        max_sum
    }
}

/**
 * 1480. 一维数组的动态和
简单
相关标签
相关企业
提示
给你一个数组 nums 。数组「动态和」的计算公式为：runningSum[i] = sum(nums[0]…nums[i]) 。

请返回 nums 的动态和。



示例 1：

输入：nums = [1,2,3,4]
输出：[1,3,6,10]
解释：动态和计算过程为 [1, 1+2, 1+2+3, 1+2+3+4] 。
示例 2：

输入：nums = [1,1,1,1,1]
输出：[1,2,3,4,5]
解释：动态和计算过程为 [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1] 。
示例 3：

输入：nums = [3,1,2,10,1]
输出：[3,4,6,16,17]


提示：

1 <= nums.length <= 1000
-10^6 <= nums[i] <= 10^6
 */
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        ans[i + 1] = ans[i] + nums[i];
    }
    ans.remove(0);
    ans
}

/**
 * 1588. 所有奇数长度子数组的和
简单
相关标签
相关企业
提示
给你一个正整数数组 arr ，请你计算所有可能的奇数长度子数组的和。

子数组 定义为原数组中的一个连续子序列。

请你返回 arr 中 所有奇数长度子数组的和 。



示例 1：

输入：arr = [1,4,2,5,3]
输出：58
解释：所有奇数长度子数组和它们的和为：
[1] = 1
[4] = 4
[2] = 2
[5] = 5
[3] = 3
[1,4,2] = 7
[4,2,5] = 11
[2,5,3] = 10
[1,4,2,5,3] = 15
我们将所有值求和得到 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58
示例 2：

输入：arr = [1,2]
输出：3
解释：总共只有 2 个长度为奇数的子数组，[1] 和 [2]。它们的和为 3 。
示例 3：

输入：arr = [10,11,12]
输出：66


提示：

1 <= arr.length <= 100
1 <= arr[i] <= 1000


进阶：

你可以设计一个 O(n) 时间复杂度的算法解决此问题吗？
 */
pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    // 定义前缀和数组
    let mut prefix = vec![0; arr.len() + 1];
    // 计算前缀和
    for i in 0..arr.len() {
        prefix[i + 1] = prefix[i] + arr[i];
    }
    // 定义结果
    let mut ans = 0;
    for i in 0..arr.len() {
        let mut j = 1;
        while i + j <= arr.len() {
            ans += prefix[i + j] - prefix[i];
            j += 2;
        }
    }
    ans
}

/**
 * 1732. 找到最高海拔
简单
相关标签
相关企业
提示
有一个自行车手打算进行一场公路骑行，这条路线总共由 n + 1 个不同海拔的点组成。自行车手从海拔为 0 的点 0 开始骑行。

给你一个长度为 n 的整数数组 gain ，其中 gain[i] 是点 i 和点 i + 1 的 净海拔高度差（0 <= i < n）。请你返回 最高点的海拔 。



示例 1：

输入：gain = [-5,1,5,0,-7]
输出：1
解释：海拔高度依次为 [0,-5,-4,1,1,-6] 。最高海拔为 1 。
示例 2：

输入：gain = [-4,-3,-2,-1,4,3,2]
输出：0
解释：海拔高度依次为 [0,-4,-7,-9,-10,-6,-3,-1] 。最高海拔为 0 。


提示：

n == gain.length
1 <= n <= 100
-100 <= gain[i] <= 100
 */
pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    // 定义结果数组
    let mut res = vec![0; gain.len() + 1];
    // 遍历数组
    for i in 1..res.len() {
        res[i] = res[i - 1] + gain[i - 1];
    }
    res.iter().max().unwrap().clone()
}

/// 1893. 检查是否区域内所有整数都被覆盖
pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut flags = vec![false; 51];
    for range in ranges {
        let l = range[0].max(left) as usize;
        let r = range[1].min(right) as usize;
        for i in l..=r {
            flags[i] = true;
        }
    }
    for i in (left as usize)..=(right as usize) {
        if flags[i] == false {
            return false;
        }
    }
    true
}

/// 1422. 分割字符串的最大得分
pub fn max_score(s: String) -> i32 {
    // 定义得分
    let mut score = 0;
    let len = s.len();
    let s = s.chars().collect::<Vec<char>>();
    if s[0] == '0' {
        score += 1;
    }
    for i in 1..len {
        if s[i] == '1' {
            score += 1;
        }
    }
    // 定义结果
    let mut ans = score;
    for i in 1..len - 1 {
        if s[i] == '0' {
            score += 1;
        } else {
            score -= 1;
        }
        ans = ans.max(score);
    }
    ans
}

/**
 * 724. 寻找数组的中心下标
简单
相关标签
相关企业
提示
给你一个整数数组 nums ，请计算数组的 中心下标 。

数组 中心下标 是数组的一个下标，其左侧所有元素相加的和等于右侧所有元素相加的和。

如果中心下标位于数组最左端，那么左侧数之和视为 0 ，因为在下标的左侧不存在元素。这一点对于中心下标位于数组最右端同样适用。

如果数组有多个中心下标，应该返回 最靠近左边 的那一个。如果数组不存在中心下标，返回 -1 。



示例 1：

输入：nums = [1, 7, 3, 6, 5, 6]
输出：3
解释：
中心下标是 3 。
左侧数之和 sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11 ，
右侧数之和 sum = nums[4] + nums[5] = 5 + 6 = 11 ，二者相等。
示例 2：

输入：nums = [1, 2, 3]
输出：-1
解释：
数组中不存在满足此条件的中心下标。
示例 3：

输入：nums = [2, 1, -1]
输出：0
解释：
中心下标是 0 。
左侧数之和 sum = 0 ，（下标 0 左侧不存在元素），
右侧数之和 sum = nums[1] + nums[2] = 1 + -1 = 0 。


提示：

1 <= nums.length <= 104
-1000 <= nums[i] <= 1000
 */
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    // 计算整个数组的总和
    let total = nums.iter().sum();
    // 当前和
    let mut sum = 0;
    // 设其左侧元素之和为 sum，则其右侧元素之和为 total−nums[i]−sum。
    // 左右侧元素相等即为 sum=total−nums[i]−sum，即 2×sum + nums[i]=total
    for i in 0..nums.len() {
        if 2 * sum + nums[i] == total {
            return i as i32;
        }
        sum += nums[i]
    }
    return -1;
}

/**
 * 强化练习 3：逐步求和得到正数的最小值
给你一个整数数组 nums 。你可以选定任意的 正数 startValue 作为初始值。

你需要从左到右遍历 nums 数组，并将 startValue 依次累加上 nums 数组中的值。

请你在确保累加和始终大于等于 1 的前提下，选出一个最小的 正数 作为 startValue 。



示例 1：

输入：nums = [-3,2,-3,4,2]
输出：5
解释：如果你选择 startValue = 4，在第三次累加时，和小于 1 。
                累加求和
startValue = 4 | startValue = 5 | nums
(4 -3 ) = 1  | (5 -3 ) = 2    |  -3
(1 +2 ) = 3  | (2 +2 ) = 4    |   2
(3 -3 ) = 0  | (4 -3 ) = 1    |  -3
(0 +4 ) = 4  | (1 +4 ) = 5    |   4
(4 +2 ) = 6  | (5 +2 ) = 7    |   2
示例 2：

输入：nums = [1,2]
输出：1
解释：最小的 startValue 需要是正数。
示例 3：

输入：nums = [1,-2,-3]
输出：5


提示：

1 <= nums.length <= 100
-100 <= nums[i] <= 100

解题思路：前缀和最小值 + startValue = 1
 */
pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut min_sum = 0;
    for num in nums {
        sum += num;
        // 求前缀和最小值
        min_sum = min(min_sum, sum);
    }
    if min_sum >= 0 {
        1
    } else {
        1 - min_sum
    }
}

/**
 * 1177. 构建回文串检测
中等
相关标签
相关企业
提示
给你一个字符串 s，请你对 s 的子串进行检测。

每次检测，待检子串都可以表示为 queries[i] = [left, right, k]。我们可以 重新排列 子串 s[left], ..., s[right]，并从中选择 最多 k 项替换成任何小写英文字母。

如果在上述检测过程中，子串可以变成回文形式的字符串，那么检测结果为 true，否则结果为 false。

返回答案数组 answer[]，其中 answer[i] 是第 i 个待检子串 queries[i] 的检测结果。

注意：在替换时，子串中的每个字母都必须作为 独立的 项进行计数，也就是说，如果 s[left..right] = "aaa" 且 k = 2，我们只能替换其中的两个字母。（另外，任何检测都不会修改原始字符串 s，可以认为每次检测都是独立的）



示例：

输入：s = "abcda", queries = [[3,3,0],[1,2,0],[0,3,1],[0,3,2],[0,4,1]]
输出：[true,false,false,true,true]
解释：
queries[0] : 子串 = "d"，回文。
queries[1] : 子串 = "bc"，不是回文。
queries[2] : 子串 = "abcd"，只替换 1 个字符是变不成回文串的。
queries[3] : 子串 = "abcd"，可以变成回文的 "abba"。 也可以变成 "baab"，先重新排序变成 "bacd"，然后把 "cd" 替换为 "ab"。
queries[4] : 子串 = "abcda"，可以变成回文的 "abcba"。


提示：

1 <= s.length, queries.length <= 10^5
0 <= queries[i][0] <= queries[i][1] < s.length
0 <= queries[i][2] <= s.length
s 中只有小写英文字母
 */
pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut prefix = vec![vec![0; 26]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..26 {
            // 把前一个数组的值赋值给下一个数组
            prefix[i + 1][j] = prefix[i][j];
        }
        // 对 i 字符串出现的次数进行统计
        prefix[i + 1][s.as_bytes()[i] as usize - 97] += 1;
    }
    let mut res = vec![];
    for query in queries {
        // 字符出现奇数的次数
        let mut odd = 0;
        for i in 0..26 {
            // 计算query[1] 到 query[0] 之间的字符出现的次数的前缀和
            if (prefix[query[1] as usize + 1][i] - prefix[query[0] as usize][i]) % 2 == 1 {
                // 如果出现次数为奇数
                odd += 1;
            }
        }
        res.push(odd / 2 <= query[2]);
    }
    res
}

/**
 * 2559. 统计范围内的元音字符串数
中等
相关标签
相关企业
提示
给你一个下标从 0 开始的字符串数组 words 以及一个二维整数数组 queries 。

每个查询 queries[i] = [li, ri] 会要求我们统计在 words 中下标在 li 到 ri 范围内（包含 这两个值）并且以元音开头和结尾的字符串的数目。

返回一个整数数组，其中数组的第 i 个元素对应第 i 个查询的答案。

注意：元音字母是 'a'、'e'、'i'、'o' 和 'u' 。



示例 1：

输入：words = ["aba","bcb","ece","aa","e"], queries = [[0,2],[1,4],[1,1]]
输出：[2,3,0]
解释：以元音开头和结尾的字符串是 "aba"、"ece"、"aa" 和 "e" 。
查询 [0,2] 结果为 2（字符串 "aba" 和 "ece"）。
查询 [1,4] 结果为 3（字符串 "ece"、"aa"、"e"）。
查询 [1,1] 结果为 0 。
返回结果 [2,3,0] 。
示例 2：

输入：words = ["a","e","i"], queries = [[0,2],[0,1],[2,2]]
输出：[3,2,1]
解释：每个字符串都满足这一条件，所以返回 [3,2,1] 。


提示：

1 <= words.length <= 105
1 <= words[i].length <= 40
words[i] 仅由小写英文字母组成
sum(words[i].length) <= 3 * 105
1 <= queries.length <= 105
0 <= queries[j][0] <= queries[j][1] < words.length
 */
pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    // 定义元音数组
    let vowel = vec![b'a', b'e', b'i', b'o', b'u'];
    // word中前后字符都是元音字符时用1表示，否则用0表示
    let mut word = vec![0; words.len()];
    for (idx, w) in words.iter().enumerate() {
        let w = w.as_bytes();
        if vowel.contains(&w[0]) && vowel.contains(&w[w.len() - 1]) {
            // 如果首尾字符都是元音字符就把对应位置设置为1
            word[idx] = 1;
        }
    }
    // 统计前缀和
    let mut prefix = vec![0; words.len() + 1];
    for i in 0..words.len() {
        prefix[i + 1] = prefix[i] + word[i];
    }
    // 定义结果数组
    let mut res = vec![];
    for query in queries {
        let left = query[0] as usize;
        let right = query[1] as usize;
        // 计算前缀和
        res.push(prefix[right + 1] - prefix[left]);
    }
    res
}

/**
 * 2389. 和有限的最长子序列
简单
相关标签
相关企业
提示
给你一个长度为 n 的整数数组 nums ，和一个长度为 m 的整数数组 queries 。

返回一个长度为 m 的数组 answer ，其中 answer[i] 是 nums 中 元素之和小于等于 queries[i] 的 子序列 的 最大 长度  。

子序列 是由一个数组删除某些元素（也可以不删除）但不改变剩余元素顺序得到的一个数组。



示例 1：

输入：nums = [4,5,2,1], queries = [3,10,21]
输出：[2,3,4]
解释：queries 对应的 answer 如下：
- 子序列 [2,1] 的和小于或等于 3 。可以证明满足题目要求的子序列的最大长度是 2 ，所以 answer[0] = 2 。
- 子序列 [4,5,1] 的和小于或等于 10 。可以证明满足题目要求的子序列的最大长度是 3 ，所以 answer[1] = 3 。
- 子序列 [4,5,2,1] 的和小于或等于 21 。可以证明满足题目要求的子序列的最大长度是 4 ，所以 answer[2] = 4 。
示例 2：

输入：nums = [2,3,4,5], queries = [1]
输出：[0]
解释：空子序列是唯一一个满足元素和小于或等于 1 的子序列，所以 answer[0] = 0 。


提示：

n == nums.length
m == queries.length
1 <= n, m <= 1000
1 <= nums[i], queries[i] <= 106
 */
pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    // 前缀和数组
    let mut prefix = vec![0; nums.len() + 1];
    let mut nums = nums;
    // 先排序
    nums.sort();
    // 计算前缀和
    nums.iter().enumerate().for_each(|(i, &num)| {
        prefix[i + 1] = prefix[i] + num;
    });
    // 定义结果数组
    let mut res = vec![];
    for i in 0..queries.len() {
        let target = queries[i];
        // 在前缀和数组中二分查找
        let mut left = 0;
        let mut right = prefix.len();
        while left < right {
            let mid = left + (right - left) / 2;
            // 右边界二分搜索
            if prefix[mid] == target {
                // 当相等时继续向左收缩左边界
                left = mid + 1;
            } else if prefix[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        res.push(left as i32 - 1);
    }
    res
}

/**
 * 1524. 和为奇数的子数组数目
中等
相关标签
相关企业
提示
给你一个整数数组 arr 。请你返回和为 奇数 的子数组数目。

由于答案可能会很大，请你将结果对 10^9 + 7 取余后返回。



示例 1：

输入：arr = [1,3,5]
输出：4
解释：所有的子数组为 [[1],[1,3],[1,3,5],[3],[3,5],[5]] 。
所有子数组的和为 [1,4,9,3,8,5].
奇数和包括 [1,9,3,5] ，所以答案为 4 。
示例 2 ：

输入：arr = [2,4,6]
输出：0
解释：所有子数组为 [[2],[2,4],[2,4,6],[4],[4,6],[6]] 。
所有子数组和为 [2,6,12,4,10,6] 。
所有子数组和都是偶数，所以答案为 0 。
示例 3：

输入：arr = [1,2,3,4,5,6,7]
输出：16
示例 4：

输入：arr = [100,100,99,99]
输出：4
示例 5：

输入：arr = [7]
输出：1


提示：

1 <= arr.length <= 10^5
1 <= arr[i] <= 100
 */
pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    // 定义前缀和数组
    let mut prefix = vec![0; arr.len() + 1];
    // 计算前缀和
    for i in 0..arr.len() {
        prefix[i + 1] = prefix[i] + arr[i];
    }
    // 定义奇数
    let mut odd = 0;
    // 定义偶数
    let mut even = 0;
    // 定义结果
    let mut res:i64 = 0;
    // 遍历数组
    for i in 0..arr.len() {
        if prefix[i + 1] % 2 == 0 {
            even += 1;
            res += odd;
        } else {
            odd += 1;
            res += even + 1;
        }
    }
    (res % 1000000007) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(nums), 6);
    }

    #[test]
    fn test_running_sum() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(running_sum(nums), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_sum_odd_length_subarrays() {
        let arr = vec![1, 4, 2, 5, 3];
        assert_eq!(sum_odd_length_subarrays(arr), 58);
    }

    #[test]
    fn test_largest_altitude() {
        let gain = vec![-5, 1, 5, 0, -7];
        assert_eq!(largest_altitude(gain), 1);
    }

    #[test]
    fn test_is_covered() {
        let ranges = vec![vec![1, 10], vec![10, 20]];
        let left = 21;
        let right = 21;
        assert_eq!(is_covered(ranges, left, right), false);
    }

    #[test]
    fn test_max_score() {
        let s = "011101".to_string();
        assert_eq!(max_score(s), 5);
    }

    #[test]
    fn test_pivot_index() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(nums), 3);
    }

    #[test]
    fn test_max_subarray_sum_circular() {
        let nums = vec![1, -2, 3, -2];
        assert_eq!(max_subarray_sum_circular(nums), 3);
    }

    #[test]
    fn test_can_make_pali_queries() {
        let s = "abcda".to_string();
        let queries = vec![
            vec![3, 3, 0],
            vec![1, 2, 0],
            vec![0, 3, 1],
            vec![0, 3, 2],
            vec![0, 4, 1],
        ];
        assert_eq!(
            can_make_pali_queries(s, queries),
            vec![true, false, false, true, true]
        );
    }

    #[test]
    fn test_vowel_strings() {
        let words = vec!["aba", "bcb", "ece", "aa", "e"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
        assert_eq!(vowel_strings(words, queries), vec![2, 3, 0]);
    }

    #[test]
    fn test_answer_queries() {
        let nums = vec![4, 5, 2, 1];
        let queries = vec![3, 10, 21];
        assert_eq!(answer_queries(nums, queries), vec![2, 3, 4]);
    }

    #[test]
    fn test_num_of_subarrays() {
        let arr = vec![1, 3, 5];
        assert_eq!(num_of_subarrays(arr), 4);
    }
}
