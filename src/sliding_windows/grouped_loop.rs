/**
 * 1446. 连续字符
简单
相关标签
相关企业
提示
给你一个字符串 s ，字符串的「能量」定义为：只包含一种字符的最长非空子字符串的长度。

请你返回字符串 s 的 能量。

 

示例 1：

输入：s = "leetcode"
输出：2
解释：子字符串 "ee" 长度为 2 ，只包含字符 'e' 。
示例 2：

输入：s = "abbcccddddeeeeedcba"
输出：5
解释：子字符串 "eeeee" 长度为 5 ，只包含字符 'e' 。
 

提示：

1 <= s.length <= 500
s 只包含小写英文字母。
 */
pub fn max_power(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    // 定义左指针
    let mut left = 0;
    // 定义结果
    let mut ans = 1;
    while left < s.len() {
        // 记录开始位置
        let start = left;
        // 开始位置已满足，从下一个位置开始判断
        left += 1;
        while left < s.len() && s[left - 1] == s[left] {
            left += 1;
        }
        ans = ans.max(left - start);
    }
    ans as i32
}

/**
 * 1869. 哪种连续子字符串更长
简单
相关标签
相关企业
提示
给你一个二进制字符串 s 。如果字符串中由 1 组成的 最长 连续子字符串 严格长于 由 0 组成的 最长 连续子字符串，返回 true ；否则，返回 false 。

例如，s = "110100010" 中，由 1 组成的最长连续子字符串的长度是 2 ，由 0 组成的最长连续子字符串的长度是 3 。
注意，如果字符串中不存在 0 ，此时认为由 0 组成的最长连续子字符串的长度是 0 。字符串中不存在 1 的情况也适用此规则。

 

示例 1：

输入：s = "1101"
输出：true
解释：
由 1 组成的最长连续子字符串的长度是 2："1101"
由 0 组成的最长连续子字符串的长度是 1："1101"
由 1 组成的子字符串更长，故返回 true 。
示例 2：

输入：s = "111000"
输出：false
解释：
由 1 组成的最长连续子字符串的长度是 3："111000"
由 0 组成的最长连续子字符串的长度是 3："111000"
由 1 组成的子字符串不比由 0 组成的子字符串长，故返回 false 。
示例 3：

输入：s = "110100010"
输出：false
解释：
由 1 组成的最长连续子字符串的长度是 2："110100010"
由 0 组成的最长连续子字符串的长度是 3："110100010"
由 1 组成的子字符串不比由 0 组成的子字符串长，故返回 false 。
 

提示：

1 <= s.length <= 100
s[i] 不是 '0' 就是 '1'
 */
pub fn check_zero_ones(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    // 定义0的连续最长长度
    let mut zero_len = 0;
    // 定义1的连续最长长度
    let mut one_len = 0;
    // 定义指针,指向起始位置
    let mut i = 0;
    while i < s.len() {
        // 记录开始位置
        let start = i;
        i += 1;
        while i < s.len() && s[i - 1] == s[i] {
            i += 1;
        }
        if s[i - 1] == '0' {
            zero_len = zero_len.max(i - start);
        } else {
            one_len = one_len.max(i - start);
        }
    }
    one_len > zero_len
}

/**
 * 1957. 删除字符使字符串变好

简单
相关标签
相关企业
提示
一个字符串如果没有 三个连续 相同字符，那么它就是一个 好字符串 。

给你一个字符串 s ，请你从 s 删除 最少 的字符，使它变成一个 好字符串 。

请你返回删除后的字符串。题目数据保证答案总是 唯一的 。

 

示例 1：

输入：s = "leeetcode"
输出："leetcode"
解释：
从第一组 'e' 里面删除一个 'e' ，得到 "leetcode" 。
没有连续三个相同字符，所以返回 "leetcode" 。
示例 2：

输入：s = "aaabaaaa"
输出："aabaa"
解释：
从第一组 'a' 里面删除一个 'a' ，得到 "aabaaaa" 。
从第二组 'a' 里面删除两个 'a' ，得到 "aabaa" 。
没有连续三个相同字符，所以返回 "aabaa" 。
示例 3：

输入：s = "aab"
输出："aab"
解释：没有连续三个相同字符，所以返回 "aab" 。
 

提示：

1 <= s.length <= 105
s 只包含小写英文字母。
 */
pub fn make_fancy_string(s: String) -> String {
    let mut s = s.chars().collect::<Vec<char>>();
    // 定义指针
    let mut i = 0;
    while i < s.len() {
        // 记录开始位置
        let start = i;
        i += 1;
        while i < s.len() && s[i - 1] == s[i] {
            i += 1;
        }
        // 记录重复字符长度
        let mut len = i - start;
        // 如果重复字符超过2个就删除掉多余的
        while len > 2 {
            // 从start位置删除
            s.remove(start);
            // char长度减1
            len -= 1;
            // 指针减1
            i -= 1;
        }
    }
    s.iter().collect()
}

/**
 * 2038. 如果相邻两个颜色均相同则删除当前颜色

中等
相关标签
相关企业
提示
总共有 n 个颜色片段排成一列，每个颜色片段要么是 'A' 要么是 'B' 。给你一个长度为 n 的字符串 colors ，其中 colors[i] 表示第 i 个颜色片段的颜色。

Alice 和 Bob 在玩一个游戏，他们 轮流 从这个字符串中删除颜色。Alice 先手 。

如果一个颜色片段为 'A' 且 相邻两个颜色 都是颜色 'A' ，那么 Alice 可以删除该颜色片段。Alice 不可以 删除任何颜色 'B' 片段。
如果一个颜色片段为 'B' 且 相邻两个颜色 都是颜色 'B' ，那么 Bob 可以删除该颜色片段。Bob 不可以 删除任何颜色 'A' 片段。
Alice 和 Bob 不能 从字符串两端删除颜色片段。
如果其中一人无法继续操作，则该玩家 输 掉游戏且另一玩家 获胜 。
假设 Alice 和 Bob 都采用最优策略，如果 Alice 获胜，请返回 true，否则 Bob 获胜，返回 false。

 

示例 1：

输入：colors = "AAABABB"
输出：true
解释：
AAABABB -> AABABB
Alice 先操作。
她删除从左数第二个 'A' ，这也是唯一一个相邻颜色片段都是 'A' 的 'A' 。

现在轮到 Bob 操作。
Bob 无法执行任何操作，因为没有相邻位置都是 'B' 的颜色片段 'B' 。
因此，Alice 获胜，返回 true 。
示例 2：

输入：colors = "AA"
输出：false
解释：
Alice 先操作。
只有 2 个 'A' 且它们都在字符串的两端，所以她无法执行任何操作。
因此，Bob 获胜，返回 false 。
示例 3：

输入：colors = "ABBBBBBBAAA"
输出：false
解释：
ABBBBBBBAAA -> ABBBBBBBAA
Alice 先操作。
她唯一的选择是删除从右数起第二个 'A' 。

ABBBBBBBAA -> ABBBBBBAA
接下来轮到 Bob 操作。
他有许多选择，他可以选择任何一个 'B' 删除。

然后轮到 Alice 操作，她无法删除任何片段。
所以 Bob 获胜，返回 false 。
 

提示：

1 <= colors.length <= 105
colors 只包含字母 'A' 和 'B'
 */
pub fn winner_of_game(colors: String) -> bool {
    let colors = colors.chars().collect::<Vec<char>>();
    // 统计Alice删除的次数
    let mut a = 0;
    // 统计Bob删除的次数
    let mut b = 0;
    // 定义指针
    let mut i = 0;
    while i < colors.len() {
        // 记录开始位置
        let start = i;
        i += 1;
        while i < colors.len() && colors[i - 1] == colors[i] {
            i += 1;
        }
        // 记录重复字符串长度
        let len = i - start;
        if len > 2 {
            if colors[i - 1] == 'A' {
                a += len - 2;
            } else {
                b += len - 2;
            }
        }
    }
    a > b
}

/**
 * 1759. 统计同质子字符串的数目

中等
相关标签
相关企业
提示
给你一个字符串 s ，返回 s 中 同质子字符串 的数目。由于答案可能很大，只需返回对 109 + 7 取余 后的结果。

同质字符串 的定义为：如果一个字符串中的所有字符都相同，那么该字符串就是同质字符串。

子字符串 是字符串中的一个连续字符序列。

 

示例 1：

输入：s = "abbcccaa"
输出：13
解释：同质子字符串如下所列：
"a"   出现 3 次。
"aa"  出现 1 次。
"b"   出现 2 次。
"bb"  出现 1 次。
"c"   出现 3 次。
"cc"  出现 2 次。
"ccc" 出现 1 次。
3 + 1 + 2 + 1 + 3 + 2 + 1 = 13
示例 2：

输入：s = "xy"
输出：2
解释：同质子字符串是 "x" 和 "y" 。
示例 3：

输入：s = "zzzzz"
输出：15
 

提示：

1 <= s.length <= 105
s 由小写字符串组成。
 */
pub fn count_homogenous(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    // 统计结果
    let mut ans = 0;
    // 定义mod
    let m = 1000000007;
    // 记录当前数,默认值为1的原因是至少有一个字符自己满足同质子字符串的条件
    let mut count = 1;
    for i in 1..s.len() {
        if s[i - 1] == s[i] {
            // 如果相等则累加
            count += 1;
        } else {
            // 如果不相等则计算当前数的和
            ans = (ans + get_sum(count)) % m;
            // 重置当前数
            count = 1;
        }
    }
    ans = (ans + get_sum(count)) % m;
    ans
}

fn get_sum(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum = (sum + i) % 1000000007;
    }
    sum
}

/**
 * 228. 汇总区间

简单
相关标签
相关企业
给定一个  无重复元素 的 有序 整数数组 nums 。

返回 恰好覆盖数组中所有数字 的 最小有序 区间范围列表 。也就是说，
nums 的每个元素都恰好被某个区间范围所覆盖，并且不存在属于某个范围但不属于 nums 的数字 x 。

列表中的每个区间范围 [a,b] 应该按如下格式输出：

"a->b" ，如果 a != b
"a" ，如果 a == b
 

示例 1：

输入：nums = [0,1,2,4,5,7]
输出：["0->2","4->5","7"]
解释：区间范围是：
[0,2] --> "0->2"
[4,5] --> "4->5"
[7,7] --> "7"
示例 2：

输入：nums = [0,2,3,4,6,8,9]
输出：["0","2->4","6","8->9"]
解释：区间范围是：
[0,0] --> "0"
[2,4] --> "2->4"
[6,6] --> "6"
[8,9] --> "8->9"
 

提示：

0 <= nums.length <= 20
-231 <= nums[i] <= 231 - 1
nums 中的所有值都 互不相同
nums 按升序排列
 */
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res = vec![];
    let mut i = 0;
    while i < nums.len() {
        let start = i;
        i += 1;
        // 如果是连续递增的则继续往后遍历
        while i < nums.len() && nums[i - 1] + 1 == nums[i] {
            i += 1;
        }
        // 如果start和i-1相等则说明只有一个数
        if start == i - 1 {
            res.push(nums[start].to_string());
        } else {
            // 否则说明有多个数
            res.push(format!("{}->{}", nums[start], nums[i - 1]));
        }
    }
    res
}

/**
 * 2765. 最长交替子数组
已解答
简单
相关标签
相关企业
提示
给你一个下标从 0 开始的整数数组 nums 。如果 nums 中长度为 m 的子数组 s 满足以下条件，我们称它是一个 交替子数组 ：

m 大于 1 。
s1 = s0 + 1 。
下标从 0 开始的子数组 s 与数组 [s0, s1, s0, s1,...,s(m-1) % 2] 一样。也就是说，s1 - s0 = 1 ，s2 - s1 = -1 ，s3 - s2 = 1 ，s4 - s3 = -1 ，以此类推，直到 s[m - 1] - s[m - 2] = (-1)m 。
请你返回 nums 中所有 交替 子数组中，最长的长度，如果不存在交替子数组，请你返回 -1 。

子数组是一个数组中一段连续 非空 的元素序列。

 

示例 1：

输入：nums = [2,3,4,3,4]
输出：4
解释：交替子数组有 [3,4] ，[3,4,3] 和 [3,4,3,4] 。最长的子数组为 [3,4,3,4] ，长度为4 。
示例 2：

输入：nums = [4,5,6]
输出：2
解释：[4,5] 和 [5,6] 是仅有的两个交替子数组。它们长度都为 2 。
 

提示：

2 <= nums.length <= 100
1 <= nums[i] <= 104

对于本题来说，在内层循环时，假设这一组的第一个数是 3，那么这一组的数字必须形如 3,4,3,4,⋯，也就是
nums[i]=nums[i−2]
 */
pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = -1;
    let mut i = 0;
    while i < n - 1 {
        // 1.判断是否递增
        if nums[i + 1] - nums[i] != 1 {
            // 直接跳过
            i += 1;
            continue;
        }
        // 记录这一组的开始位置
        let start = i;
        // i 和 i+1 已经满足要求，从 i+2 开始判断
        i += 2;
        // 2.判断间隔的数是否相等
        while i < n && nums[i] == nums[i - 2] {
            i += 1;
        }
        // 从 start 到 i-1 是满足题目要求的（并且无法再延长的）子数组
        ans = ans.max((i - start) as i32);
        // 另外，对于 [3,4,3,4,5,4,5] 这样的数组，第一组交替子数组为 [3,4,3,4]，第二组交替子数组为 [4,5,4,5]，
        // 这两组有一个数是重叠的，所以下面代码在外层循环末尾要把 i 减一。
        i -= 1;
    }
    ans
}

/**
 * 2110. 股票平滑下跌阶段的数目

中等
相关标签
相关企业
提示
给你一个整数数组 prices ，表示一支股票的历史每日股价，其中 prices[i] 是这支股票第 i 天的价格。

一个 平滑下降的阶段 定义为：对于 连续一天或者多天 ，每日股价都比 前一日股价恰好少 1 ，这个阶段第一天的股价没有限制。

请你返回 平滑下降阶段 的数目。

 

示例 1：

输入：prices = [3,2,1,4]
输出：7
解释：总共有 7 个平滑下降阶段：
[3], [2], [1], [4], [3,2], [2,1] 和 [3,2,1]
注意，仅一天按照定义也是平滑下降阶段。
示例 2：

输入：prices = [8,6,7,7]
输出：4
解释：总共有 4 个连续平滑下降阶段：[8], [6], [7] 和 [7]
由于 8 - 6 ≠ 1 ，所以 [8,6] 不是平滑下降阶段。
示例 3：

输入：prices = [1]
输出：1
解释：总共有 1 个平滑下降阶段：[1]
 

提示：

1 <= prices.length <= 105
1 <= prices[i] <= 105

将 prices 按照平滑下降的定义分成若干组。例如 [3,2,1,4] 分为 [3,2,1] 和 [4] 两组。
对于每一组的所有非空子数组，都是平滑下降的。设该组长度为 m，则该组的非空子数组个数为 m*(m+1)/2。
累加每组的非空子区间个数即为答案。
 */
pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let n = prices.len();
    let mut ans: i64 = 0;
    let mut i = 0;
    while i < n {
        // 记录这一组的开始位置
        let start = i;
        // i 已经满足要求，从 i+1 开始判断
        i += 1;
        // 2.判断间隔的数是否相差1
        while i < n && prices[i - 1] - prices[i] == 1 {
            i += 1;
        }
        // 从 start 到 i-1 是满足题目要求的（并且无法再延长的）子数组
        let t = (i - start) as i64;
        // 累加
        ans += (t + 1) * t / 2;
    }
    ans
}

/**
 * 1578. 使绳子变成彩色的最短时间

中等
相关标签
相关企业
提示
Alice 把 n 个气球排列在一根绳子上。给你一个下标从 0 开始的字符串 colors ，其中 colors[i] 是第 i 个气球的颜色。

Alice 想要把绳子装扮成 五颜六色的 ，且她不希望两个连续的气球涂着相同的颜色，所以她喊来 Bob 帮忙。Bob 可以从绳子上移除一些气球使绳子变成 彩色 。给你一个 下标从 0 开始 的整数数组 neededTime ，其中 neededTime[i] 是 Bob 从绳子上移除第 i 个气球需要的时间（以秒为单位）。

返回 Bob 使绳子变成 彩色 需要的 最少时间 。

 

示例 1：
<img src="https://assets.leetcode.com/uploads/2021/12/13/ballon1.jpg" />

输入：colors = "abaac", neededTime = [1,2,3,4,5]
输出：3
解释：在上图中，'a' 是蓝色，'b' 是红色且 'c' 是绿色。
Bob 可以移除下标 2 的蓝色气球。这将花费 3 秒。
移除后，不存在两个连续的气球涂着相同的颜色。总时间 = 3 。
示例 2：
<img src="https://assets.leetcode.com/uploads/2021/12/13/balloon2.jpg" />

输入：colors = "abc", neededTime = [1,2,3]
输出：0
解释：绳子已经是彩色的，Bob 不需要从绳子上移除任何气球。
示例 3：
<img src="https://assets.leetcode.com/uploads/2021/12/13/balloon3.jpg" />

输入：colors = "aabaa", neededTime = [1,2,3,4,1]
输出：2
解释：Bob 会移除下标 0 和下标 4 处的气球。这两个气球各需要 1 秒来移除。
移除后，不存在两个连续的气球涂着相同的颜色。总时间 = 1 + 1 = 2 。
 

提示：

n == colors.length == neededTime.length
1 <= n <= 105
1 <= neededTime[i] <= 104
colors 仅由小写英文字母组成
 */
pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let colors = colors.chars().collect::<Vec<char>>();
    // 定义指针
    let mut i = 0;
    // 定义结果
    let mut ans = 0;
    while i < colors.len() {
        // 位置i对应的颜色
        let c = colors[i];
        // 定义重复颜色中删除成本最高是多少
        let mut max = 0;
        // 重复颜色删除的总成本
        let mut sum = 0;
        // 从i开始遍历相同颜色
        while i < colors.len() && colors[i] == c {
            // 相同颜色中删除最高成本的
            max = max.max(needed_time[i]);
            // 累计相同颜色删除的总成本
            sum += needed_time[i];
            i += 1;
        }
        // 保留删除成本最高的剩下的就是需要删除的最小成本，累加到结果中
        ans += sum - max;
    }
    ans
}

/**
 * 1839. 所有元音按顺序排布的最长子字符串

中等
相关标签
相关企业
提示
当一个字符串满足如下条件时，我们称它是 美丽的 ：

所有 5 个英文元音字母（'a' ，'e' ，'i' ，'o' ，'u'）都必须 至少 出现一次。
这些元音字母的顺序都必须按照 字典序 升序排布（也就是说所有的 'a' 都在 'e' 前面，所有的 'e' 都在 'i' 前面，以此类推）
比方说，字符串 "aeiou" 和 "aaaaaaeiiiioou" 都是 美丽的 ，但是 "uaeio" ，"aeoiu" 和 "aaaeeeooo" 不是美丽的 。

给你一个只包含英文元音字母的字符串 word ，请你返回 word 中 最长美丽子字符串的长度 。如果不存在这样的子字符串，请返回 0 。

子字符串 是字符串中一个连续的字符序列。

 

示例 1：

输入：word = "aeiaaioaaaaeiiiiouuuooaauuaeiu"
输出：13
解释：最长子字符串是 "aaaaeiiiiouuu" ，长度为 13 。
示例 2：

输入：word = "aeeeiiiioooauuuaeiou"
输出：5
解释：最长子字符串是 "aeiou" ，长度为 5 。
示例 3：

输入：word = "a"
输出：0
解释：没有美丽子字符串，所以返回 0 。
 

提示：

1 <= word.length <= 5 * 105
word 只包含字符 'a'，'e'，'i'，'o' 和 'u' 。
 */
pub fn longest_beautiful_substring(word: String) -> i32 {
    let word = word.chars().collect::<Vec<char>>();
    // 定义指针
    let mut i = 0;
    // 定义结果
    let mut ans = 0;
    while i < word.len() {
        // 记录开始位置
        let start = i;
        i += 1;
        // 统计
        let mut cnt = 0;
        // 从i开始遍历相同颜色,并统计满足条件的次数
        while i < word.len() && check(word[i - 1], word[i]) {
            if word[i] != word[i - 1] {
                // i-1和i不相等时才加1，相等表示是重复字符不需要加1 或 是1️以 u元音结尾时也不加1
                cnt += 1;
            }
            i += 1;
        }
        if cnt >= 4 {
            // 当元音字母都包含时，计算子串的长度，和比较之前结果保留最大值
            ans = ans.max(i - start);
        }
    }
    ans as i32
}

fn check(a: char, b: char) -> bool {
    match a {
        'a' => b == 'e' || b == 'a',
        'e' => b == 'i' || b == 'e',
        'i' => b == 'o' || b == 'i',
        'o' => b == 'u' || b == 'o',
        'u' => b == 'u',
        _ => false,
    }
}

/**
 * 2760. 最长奇偶子数组

简单
相关标签
提示
给你一个下标从 0 开始的整数数组 nums 和一个整数 threshold 。

请你从 nums 的子数组中找出以下标 l 开头、下标 r 结尾 (0 <= l <= r < nums.length) 且满足以下条件的 最长子数组 ：

nums[l] % 2 == 0
对于范围 [l, r - 1] 内的所有下标 i ，nums[i] % 2 != nums[i + 1] % 2
对于范围 [l, r] 内的所有下标 i ，nums[i] <= threshold
以整数形式返回满足题目要求的最长子数组的长度。

注意：子数组 是数组中的一个连续非空元素序列。

 

示例 1：

输入：nums = [3,2,5,4], threshold = 5
输出：3
解释：在这个示例中，我们选择从 l = 1 开始、到 r = 3 结束的子数组 => [2,5,4] ，满足上述条件。
因此，答案就是这个子数组的长度 3 。可以证明 3 是满足题目要求的最大长度。
示例 2：

输入：nums = [1,2], threshold = 2
输出：1
解释：
在这个示例中，我们选择从 l = 1 开始、到 r = 1 结束的子数组 => [2] 。
该子数组满足上述全部条件。可以证明 1 是满足题目要求的最大长度。
示例 3：

输入：nums = [2,3,4,5], threshold = 4
输出：3
解释：
在这个示例中，我们选择从 l = 0 开始、到 r = 2 结束的子数组 => [2,3,4] 。 
该子数组满足上述全部条件。
因此，答案就是这个子数组的长度 3 。可以证明 3 是满足题目要求的最大长度。
 

提示：

1 <= nums.length <= 100 
1 <= nums[i] <= 100 
1 <= threshold <= 100
 */
///
/// 题意解读
///
/// 选一个最长连续子数组，满足子数组元素依次是偶数，奇数，偶数，奇数，……，且元素值均不超过 threshold。
///
/// 例如 nums=[2,1,1,4,3,4,2,8],threshold=5，数组可以分成 [2,1],1,[4,3,4],[2],8，
/// 其中 [⋯ ] 是子数组，其余数字不满足要求。所以最长连续子数组的长度是 3。
///
/// 分组循环
///
/// 适用场景：按照题目要求，数组会被分割成若干组，且每一组的判断/处理逻辑是一样的。
///
/// 核心思想：
///
/// 外层循环负责遍历组之前的准备工作（记录开始位置），和遍历组之后的统计工作（更新答案最大值）。
///
/// 内层循环负责遍历组，找出这一组最远在哪结束。
///
/// 这个写法的好处是，各个逻辑块分工明确，也不需要特判最后一组（易错点）。以我的经验，这个写法是所有写法中最不容易出 bug 的，推荐大家记住。
///
/// 时间复杂度乍一看是 O(n^2)，但注意变量 iii 只会增加，不会重置也不会减少。所以二重循环总共循环 O(n) 次，所以时间复杂度是 O(n)。
///
/// 练习
/// 一般来说，分组循环的模板如下（可根据题目调整）：
///
/// ```
/// n = len(nums)
/// i = 0
/// while i < n:
///     start = i
///     while i < n and ...:
///         i += 1
///     # 从 start 到 i-1 是一组
///     # 下一组从 i 开始，无需 i += 1
/// ```
/// https://leetcode.cn/problems/longest-even-odd-subarray-with-threshold/solutions/2528771/jiao-ni-yi-ci-xing-ba-dai-ma-xie-dui-on-zuspx/
pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        // 这里的目的是让i指针停留在一个小于threshold偶数的位置
        if nums[i] > threshold || nums[i] % 2 != 0 {
            i += 1; // 直接跳过
            continue;
        }
        // 记录这一组的开始位置
        let start = i;
        // 开始位置已经满足要求，从下一个位置开始判断
        i += 1;
        while i < n && nums[i] <= threshold && nums[i] % 2 != nums[i - 1] % 2 {
            i += 1;
        }
        // 从 start 到 i-1 是满足题目要求的（并且无法再延长的）子数组
        ans = ans.max(i - start);
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_power() {
        let s = "leetcode".to_string();
        let max_power = max_power(s);
        println!("{:?}", max_power)
    }

    #[test]
    fn test_check_zero_ones() {
        let s = "1101".to_string();
        let check_zero_ones = check_zero_ones(s);
        println!("{:?}", check_zero_ones)
    }

    #[test]
    fn test_make_fancy_string() {
        let s = "leeetcode".to_string();
        let make_fancy_string = make_fancy_string(s);
        println!("{:?}", make_fancy_string)
    }

    #[test]
    fn test_winner_of_game() {
        let colors = "AAABBB".to_string();
        let winner_of_game = winner_of_game(colors);
        println!("{:?}", winner_of_game)
    }

    #[test]
    fn test_count_homogenous() {
        let s = "abbcccaa".to_string();
        let count_homogenous = count_homogenous(s);
        println!("{:?}", count_homogenous)
    }

    #[test]
    fn test_summary_ranges() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let summary_ranges = summary_ranges(nums);
        println!("{:?}", summary_ranges)
    }

    #[test]
    fn test_get_descent_periods() {
        let prices = vec![3, 2, 1, 4];
        let get_descent_periods = get_descent_periods(prices);
        println!("{:?}", get_descent_periods)
    }

    #[test]
    fn test_min_cost() {
        let colors = "aabbcc".to_string();
        let needed_time = vec![1, 2, 1, 2, 1, 2];
        let min_cost = min_cost(colors, needed_time);
        println!("{:?}", min_cost)
    }

    #[test]
    fn test_longest_beautiful_substring() {
        let word = "aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string();
        let longest = longest_beautiful_substring(word);
        println!("{:?}", longest)
    }
}