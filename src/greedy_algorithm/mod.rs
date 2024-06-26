use std::collections::HashMap;

/// 【变量说明】
/// 数组长度为 n ，最小操作次数为 k ，操作前数组的最小值为 min ，操作前数组的最大值为 max ，操作前数组的第二大值为 max2 ，操作前数组所有数和为 sum 。
///
/// 【思路分析】
/// 假设你可以无误执行完整个流程，这样可以搭建流程执行前后的桥梁。
/// 将操作流程前后的量进行分析，找到一个不变量，构成等式，进行求解。
/// 根据题意，每次操作将会使 n - 1 个元素增加 1 。由于你执行了 k 次，所以一共会使 sum 增加 k * (n - 1) 。即操作结束后数组的和为 sum + k * (n - 1) 。
/// 贪心部分：在整个流程的每次操作中，最小的那个值都会增加 1 。（贪心的证明在下个板块，建议最后看）
/// 由贪心可知，经过k 步后， min 变为了 min + k ，也意味着此时数组的每一项都变为了 min + k ，所以操作结束后数组的和为 n * (min + k) 。
/// 根据等量代换：
/// sum + k * (n - 1) = n * (min + k)
/// sum + k * n - k = n * min + n * k
/// sum - k = n * min
/// k = sum - n * min
/// 所以，最短操作次数为 k = sum - n * min 。
/// 先求和再减去长度n个最小值，不是相当于每一项减去最小值再求和吗？
/// 数学证明：（证明公式为latex代码，课根据需要转为公式查看）
/// \sum_{i=0}^{i=n-1}{nums\left[ i \right]}-n\times \min =\sum_{i=0}^{i=n-1}{nums\left[ i \right]}-\sum_{i=0}^{i=n-1}{\min}=\sum_{i=0}^{i=n-1}{nums\left[ i \right] -\min}
/// 最终结论：k = SUM(nums[i] - min)
///
/// 【贪心证明】
/// 结论：在整个流程的每次操作中，最小的那个值都会增加 1 。
///
/// 证明过程：
/// 这个贪心可能不好理解，这里我们可以分成两个部分来理解：
/// 第一部分就是 max2 在追赶 max 的过程，设这个过程执行了 k' 次，在这 k' 次中，每次必然会使 max 的以外的元素增加 1 ，直到产生一个新的元素大于 max 为止（这个元素就是 max2 位置的元素，记为 max'），这个阶段 min 每次都增加了 1 ，并且这一步的前一步，max' 是等于max 的，之后的每一步都是 max' 和 max 争夺最大值的阶段， max' 和 max 轮流增加 1 ，其他元素每次的增加 1。由于其他元素的增速是大于 max' 和 max 的，所以很快会有个数大于 max' 和 max （也就是原数组的第三大的数，记为 max''），之后就是 max 和 max' 和 max'' 轮流作为最大值的过程。……
/// 第二部分就是第一个部分的临界阶段，也就是由于增速差除了原来最小的数 min 其他数轮流作为最大值时，这个时候 min 所在位置的数会以一种很快的速度增长直到他比最大值的数小 1为止（也就是支出现两种数为止），然后除了最大值的数，其他都增加 1 ，直到弥补和最小值之间的差为止。这个部分 min 也一直在增加。
///
/// 例题：这里以[1,2,3,4] 为例
///
/// 第一个部分：
/// [2,3,4,4]：其余数都在追赶你索引为3的数，最小数从1增加为了2。
/// [3,4,5,4]：索引为2的数超过了索引为3的数，最小数从2增加为了3。
/// [4,5,5,5]：其余数都在追赶你索引为2的数，最小数从3增加为了4。（此时只有两种类型的数 4 和 5 ，且差值为 1 ，进入第二部分）
///
/// 第二部分
/// [5,6,5,6]：索引为1的数超过了索引为2的数，最小数从4增加为了5。
/// [6,6,6,7]：其余数都在追赶你索引为1的数，索引为3的数超过了索引为1的数，最小数从5增加为了6。
/// [7,7,7,7]：其余数都在追赶你索引为3的数，最小数从6增加为了7。
pub fn min_moves(nums: Vec<i32>) -> i32 {
    let mut min = nums[0];
    let mut sum = 0;
    for i in nums.iter() {
        min = min.min(*i);
        sum += *i;
    }
    sum - nums.len() as i32 * min
}

/**
 * 455. 分发饼干
已解答
简单
相关标签
相关企业
假设你是一位很棒的家长，想要给你的孩子们一些小饼干。但是，每个孩子最多只能给一块饼干。

对每个孩子 i，都有一个胃口值 g[i]，这是能让孩子们满足胃口的饼干的最小尺寸；并且每块饼干 j，都有一个尺寸 s[j] 。如果 s[j] >= g[i]，我们可以将这个饼干 j 分配给孩子 i ，这个孩子会得到满足。你的目标是尽可能满足越多数量的孩子，并输出这个最大数值。

```
示例 1:

输入: g = [1,2,3], s = [1,1]
输出: 1
解释:
你有三个孩子和两块小饼干，3个孩子的胃口值分别是：1,2,3。
虽然你有两块小饼干，由于他们的尺寸都是1，你只能让胃口值是1的孩子满足。
所以你应该输出1。
示例 2:

输入: g = [1,2], s = [1,2,3]
输出: 2
解释:
你有两个孩子和三块小饼干，2个孩子的胃口值分别是1,2。
你拥有的饼干数量和尺寸都足以让所有孩子满足。
所以你应该输出2.


提示：

1 <= g.length <= 3 * 104
0 <= s.length <= 3 * 104
1 <= g[i], s[j] <= 231 - 1
```
 */
pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    if g.is_empty() || s.is_empty() {
        return 0;
    }
    // 胃口值
    let mut g = g;
    // 饼干的尺寸
    let mut s = s;
    // 排序
    g.sort();
    s.sort();
    // 定义结果
    let mut res = 0;
    let mut right: i32 = s.len() as i32 - 1;
    for v in g.iter().rev() {
        while right >= 0 && s[right as usize] >= *v {
            res += 1;
            right -= 1;
            break;
        }
    }
    res
}

/**
 * 376. 摆动序列

如果连续数字之间的差严格地在正数和负数之间交替，则数字序列称为 摆动序列 。第一个差（如果存在的话）可能是正数或负数。
仅有一个元素或者含两个不等元素的序列也视作摆动序列。

例如， [1, 7, 4, 9, 2, 5] 是一个 摆动序列 ，因为差值 (6, -3, 5, -7, 3) 是正负交替出现的。

相反，[1, 4, 7, 2, 5] 和 [1, 7, 4, 5, 5] 不是摆动序列，第一个序列是因为它的前两个差值都是正数，第二个序列是因为它的最后一个差值为零。
子序列 可以通过从原始序列中删除一些（也可以不删除）元素来获得，剩下的元素保持其原始顺序。

给你一个整数数组 nums ，返回 nums 中作为 摆动序列 的 最长子序列的长度 。


```
示例 1：

输入：nums = [1,7,4,9,2,5]
输出：6
解释：整个序列均为摆动序列，各元素之间的差值为 (6, -3, 5, -7, 3) 。
示例 2：

输入：nums = [1,17,5,10,13,15,10,5,16,8]
输出：7
解释：这个序列包含几个长度为 7 摆动序列。
其中一个是 [1, 17, 10, 13, 10, 16, 8] ，各元素之间的差值为 (16, -7, 3, -3, 6, -8) 。
示例 3：

输入：nums = [1,2,3,4,5,6,7,8,9]
输出：2


提示：

1 <= nums.length <= 1000
0 <= nums[i] <= 1000


进阶：你能否用 O(n) 时间复杂度完成此题?
```
 */
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 1;
    }
    // 定义结果，从1开始是因为首位元素也算摆动
    let mut res = 1;
    let mut pre_diff = 0;
    for i in 1..nums.len() {
        let cur_diff = nums[i] - nums[i - 1];
        if (pre_diff >= 0 && cur_diff < 0) || (pre_diff <= 0 && cur_diff > 0) {
            res += 1;
            pre_diff = cur_diff;
        }
    }
    res
}

/**
 * 122. 买卖股票的最佳时机 II
中等
相关标签
相关企业
给你一个整数数组 prices ，其中 prices[i] 表示某支股票第 i 天的价格。

在每一天，你可以决定是否购买和/或出售股票。你在任何时候 最多 只能持有 一股 股票。你也可以先购买，然后在 同一天 出售。

返回 你能获得的 最大 利润 。


```
示例 1：

输入：prices = [7,1,5,3,6,4]
输出：7
解释：在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5 - 1 = 4 。
     随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6 - 3 = 3 。
     总利润为 4 + 3 = 7 。
示例 2：

输入：prices = [1,2,3,4,5]
输出：4
解释：在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5 - 1 = 4 。
     总利润为 4 。
示例 3：

输入：prices = [7,6,4,3,1]
输出：0
解释：在这种情况下, 交易无法获得正利润，所以不参与交易可以获得最大利润，最大利润为 0 。


提示：

1 <= prices.length <= 3 * 104
0 <= prices[i] <= 104
```
 */
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 1..prices.len() {
        // 只累加正数的结果
        res += (prices[i] - prices[i - 1]).max(0);
    }
    res
}

/**
 * 55. 跳跃游戏
中等
相关标签
相关企业
给你一个非负整数数组 nums ，你最初位于数组的 第一个下标 。数组中的每个元素代表你在该位置可以跳跃的最大长度。

判断你是否能够到达最后一个下标，如果可以，返回 true ；否则，返回 false 。


```
示例 1：

输入：nums = [2,3,1,1,4]
输出：true
解释：可以先跳 1 步，从下标 0 到达下标 1, 然后再从下标 1 跳 3 步到达最后一个下标。
示例 2：

输入：nums = [3,2,1,0,4]
输出：false
解释：无论怎样，总会到达下标为 3 的位置。但该下标的最大跳跃长度是 0 ， 所以永远不可能到达最后一个下标。


提示：

1 <= nums.length <= 104
0 <= nums[i] <= 105
```

解题思路：

其实就是求最大范围，比较最大范围是否可以覆盖整个数组
 */
pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.is_empty() || nums.len() == 1 {
        return true;
    }
    let mut cover = 0_i32;
    // 定义左指针
    let mut left = 0_i32;
    while left <= cover {
        // 跳跃范围是当前数据的为止加上可跳跃的距离
        cover = cover.max(left + nums[left as usize]);
        if cover >= nums.len() as i32 - 1 {
            // 如果覆盖范围可以覆盖完整个数组
            return true;
        }
        left += 1;
    }
    false
}

/**
 * 45. 跳跃游戏 II
中等
相关标签
相关企业
给定一个长度为 n 的 0 索引整数数组 nums。初始位置为 nums[0]。

每个元素 nums[i] 表示从索引 i 向前跳转的最大长度。换句话说，如果你在 nums[i] 处，你可以跳转到任意 nums[i + j] 处:

0 <= j <= nums[i]
i + j < n
返回到达 nums[n - 1] 的最小跳跃次数。生成的测试用例可以到达 nums[n - 1]。


```
示例 1:

输入: nums = [2,3,1,1,4]
输出: 2
解释: 跳到最后一个位置的最小跳跃数是 2。
     从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
示例 2:

输入: nums = [2,3,0,1,4]
输出: 2


提示:

1 <= nums.length <= 104
0 <= nums[i] <= 1000
题目保证可以到达 nums[n-1]
```
 */
pub fn jump(nums: Vec<i32>) -> i32 {
    if nums.is_empty() || nums.len() == 1 {
        return 0;
    }
    // 定义结果
    let mut res = 0;
    // 定义当前跳跃范围
    let mut cur = 0;
    // 定义下一层跳跃范围
    let mut next = 0;
    for i in 0..nums.len() {
        next = next.max(i as i32 + nums[i]);
        if i as i32 == cur {
            if i != nums.len() - 1 {
                // 说明还不够跳到末尾
                cur = next;
                res += 1;
                if cur as usize == nums.len() - 1 {
                    break;
                }
            } else {
                break;
            }
        }
    }
    res
}

/**
 * 1005. K 次取反后最大化的数组和
简单
相关标签
相关企业
给你一个整数数组 nums 和一个整数 k ，按以下方法修改该数组：

选择某个下标 i 并将 nums[i] 替换为 -nums[i] 。
重复这个过程恰好 k 次。可以多次选择同一个下标 i 。

以这种方式修改数组后，返回数组 可能的最大和 。


```
示例 1：

输入：nums = [4,2,3], k = 1
输出：5
解释：选择下标 1 ，nums 变为 [4,-2,3] 。
示例 2：

输入：nums = [3,-1,0,2], k = 3
输出：6
解释：选择下标 (1, 2, 2) ，nums 变为 [3,1,0,2] 。
示例 3：

输入：nums = [2,-3,-1,5,-4], k = 2
输出：13
解释：选择下标 (1, 4) ，nums 变为 [2,3,-1,5,4] 。


提示：

1 <= nums.length <= 104
-100 <= nums[i] <= 100
1 <= k <= 104
```
 */
pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut k = k;
    let mut nums = nums;
    let len = nums.len();
    // 按照绝对值从大到小排列
    nums.sort_by(|a, b| b.abs().cmp(&a.abs()));
    for i in 0..len {
        // 如果数组中有负数，对负数取反
        if nums[i] < 0 && k > 0 {
            nums[i] = nums[i] * -1;
            k -= 1;
        }
    }
    // 如果数组中没有负数，对数组中最小的数连续取反
    if k % 2 == 1 {
        nums[len - 1] *= -1;
    }
    nums.iter().sum()
}

/**
 * 134. 加油站
中等
相关标签
相关企业
在一条环路上有 n 个加油站，其中第 i 个加油站有汽油 gas[i] 升。

你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。你从其中的一个加油站出发，开始时油箱为空。

给定两个整数数组 gas 和 cost ，如果你可以按顺序绕环路行驶一周，则返回出发时加油站的编号，否则返回 -1 。如果存在解，则 保证 它是 唯一 的。


```
示例 1:

输入: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
输出: 3
解释:
从 3 号加油站(索引为 3 处)出发，可获得 4 升汽油。此时油箱有 = 0 + 4 = 4 升汽油
开往 4 号加油站，此时油箱有 4 - 1 + 5 = 8 升汽油
开往 0 号加油站，此时油箱有 8 - 2 + 1 = 7 升汽油
开往 1 号加油站，此时油箱有 7 - 3 + 2 = 6 升汽油
开往 2 号加油站，此时油箱有 6 - 4 + 3 = 5 升汽油
开往 3 号加油站，你需要消耗 5 升汽油，正好足够你返回到 3 号加油站。
因此，3 可为起始索引。
示例 2:

输入: gas = [2,3,4], cost = [3,4,3]
输出: -1
解释:
你不能从 0 号或 1 号加油站出发，因为没有足够的汽油可以让你行驶到下一个加油站。
我们从 2 号加油站出发，可以获得 4 升汽油。 此时油箱有 = 0 + 4 = 4 升汽油
开往 0 号加油站，此时油箱有 4 - 3 + 2 = 3 升汽油
开往 1 号加油站，此时油箱有 3 - 3 + 3 = 3 升汽油
你无法返回 2 号加油站，因为返程需要消耗 4 升汽油，但是你的油箱只有 3 升汽油。
因此，无论怎样，你都不可能绕环路行驶一周。


提示:

gas.length == n
cost.length == n
1 <= n <= 105
0 <= gas[i], cost[i] <= 104
```
 */
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut cur = 0;
    let mut start = 0;
    for i in 0..gas.len() {
        // 计算加油量和耗油量的差总和，如果大于0，说明可以走完一圈，否则无法走完
        total += gas[i] - cost[i];
        // 加油量和耗油量的差为正数，说明可以走到下一个加油站，起点只会诞生在这些加油站中
        cur += gas[i] - cost[i];
        if cur < 0 {
            start = i + 1;
            cur = 0;
        }
    }
    if total < 0 {
        -1
    } else {
        start as i32
    }
}

/**
 * 135. 分发糖果
困难
相关标签
相关企业
n 个孩子站成一排。给你一个整数数组 ratings 表示每个孩子的评分。

你需要按照以下要求，给这些孩子分发糖果：

每个孩子至少分配到 1 个糖果。
相邻两个孩子评分更高的孩子会获得更多的糖果。
请你给每个孩子分发糖果，计算并返回需要准备的 最少糖果数目 。


```
示例 1：

输入：ratings = [1,0,2]
输出：5
解释：你可以分别给第一个、第二个、第三个孩子分发 2、1、2 颗糖果。
示例 2：

输入：ratings = [1,2,2]
输出：4
解释：你可以分别给第一个、第二个、第三个孩子分发 1、2、1 颗糖果。
     第三个孩子只得到 1 颗糖果，这满足题面中的两个条件。


提示：

n == ratings.length
1 <= n <= 2 * 104

```
 */
pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];
    // 从左往右遍历，如果右边的评分比左边的高，那么右边的糖果数就是左边的糖果数加1
    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    // 从右往左遍历，如果左边的评分比右边的高，那么左边的糖果数就是右边的糖果数加1
    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
    }
    candies.iter().sum()
}

/**
 * 860. 柠檬水找零
简单
相关标签
相关企业
在柠檬水摊上，每一杯柠檬水的售价为 5 美元。顾客排队购买你的产品，（按账单 bills 支付的顺序）一次购买一杯。

每位顾客只买一杯柠檬水，然后向你付 5 美元、10 美元或 20 美元。你必须给每个顾客正确找零，也就是说净交易是每位顾客向你支付 5 美元。

注意，一开始你手头没有任何零钱。

给你一个整数数组 bills ，其中 bills[i] 是第 i 位顾客付的账。如果你能给每位顾客正确找零，返回 true ，否则返回 false 。


```
示例 1：

输入：bills = [5,5,5,10,20]
输出：true
解释：
前 3 位顾客那里，我们按顺序收取 3 张 5 美元的钞票。
第 4 位顾客那里，我们收取一张 10 美元的钞票，并返还 5 美元。
第 5 位顾客那里，我们找还一张 10 美元的钞票和一张 5 美元的钞票。
由于所有客户都得到了正确的找零，所以我们输出 true。
示例 2：

输入：bills = [5,5,10,10,20]
输出：false
解释：
前 2 位顾客那里，我们按顺序收取 2 张 5 美元的钞票。
对于接下来的 2 位顾客，我们收取一张 10 美元的钞票，然后返还 5 美元。
对于最后一位顾客，我们无法退回 15 美元，因为我们现在只有两张 10 美元的钞票。
由于不是每位顾客都得到了正确的找零，所以答案是 false。


提示：

1 <= bills.length <= 105
bills[i] 不是 5 就是 10 或是 20
```
 */
pub fn lemonade_change(bills: Vec<i32>) -> bool {
    // 定义5元和10元的数量
    let mut five = 0;
    let mut ten = 0;
    for b in bills {
        match b {
            5 => {
                five += 1;
            }
            10 => {
                if five == 0 {
                    return false;
                } else {
                    five -= 1;
                    ten += 1;
                }
            }
            20 => {
                if ten > 0 && five > 0 {
                    ten -= 1;
                    five -= 1;
                } else if five >= 3 {
                    five -= 3;
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

/**
 * 406. 根据身高重建队列
中等
相关标签
相关企业
提示

假设有打乱顺序的一群人站成一个队列，数组 people 表示队列中一些人的属性（不一定按顺序）。

每个 people[i] = [hi, ki] 表示第 i 个人的身高为 hi ，前面 正好 有 ki 个身高大于或等于 hi 的人。

请你重新构造并返回输入数组 people 所表示的队列。返回的队列应该格式化为数组 queue ，
其中 queue[j] = [hj, kj] 是队列中第 j 个人的属性（queue[0] 是排在队列前面的人）。


```
示例 1：

输入：people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
输出：[[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
解释：
编号为 0 的人身高为 5 ，没有身高更高或者相同的人排在他前面。
编号为 1 的人身高为 7 ，没有身高更高或者相同的人排在他前面。
编号为 2 的人身高为 5 ，有 2 个身高更高或者相同的人排在他前面，即编号为 0 和 1 的人。
编号为 3 的人身高为 6 ，有 1 个身高更高或者相同的人排在他前面，即编号为 1 的人。
编号为 4 的人身高为 4 ，有 4 个身高更高或者相同的人排在他前面，即编号为 0、1、2、3 的人。
编号为 5 的人身高为 7 ，有 1 个身高更高或者相同的人排在他前面，即编号为 1 的人。
因此 [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] 是重新构造后的队列。
示例 2：

输入：people = [[6,0],[5,0],[4,0],[3,2],[2,2],[1,4]]
输出：[[4,0],[5,0],[2,2],[3,2],[1,4],[6,0]]


提示：

1 <= people.length <= 2000
0 <= hi <= 106
0 <= ki < people.length
题目数据确保队列可以被重建
```
 */
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    let mut queue = vec![];
    people.sort_by(|a, b| {
        if a[0] == b[0] {
            return a[1].cmp(&b[1]);
        }
        b[0].cmp(&a[0])
    });
    queue.push(people[0].clone());
    for v in people.iter().skip(1) {
        queue.insert(v[1] as usize, v.clone());
    }
    queue
}

/**
 * 452. 用最少数量的箭引爆气球
中等
相关标签
相关企业

有一些球形气球贴在一堵用 XY 平面表示的墙面上。墙面上的气球记录在整数数组 points ，
其中points[i] = [xstart, xend] 表示水平直径在 xstart 和 xend之间的气球。你不知道气球的确切 y 坐标。

一支弓箭可以沿着 x 轴从不同点 完全垂直 地射出。在坐标 x 处射出一支箭，
若有一个气球的直径的开始和结束坐标为 xstart，xend，
且满足  xstart ≤ x ≤ xend，则该气球会被 引爆 。可以射出的弓箭的数量 没有限制 。

弓箭一旦被射出之后，可以无限地前进。

给你一个数组 points ，返回引爆所有气球所必须射出的 最小 弓箭数 。

```
示例 1：

输入：points = [[10,16],[2,8],[1,6],[7,12]]
输出：2
解释：气球可以用2支箭来爆破:
-在x = 6处射出箭，击破气球[2,8]和[1,6]。
-在x = 11处发射箭，击破气球[10,16]和[7,12]。
示例 2：

输入：points = [[1,2],[3,4],[5,6],[7,8]]
输出：4
解释：每个气球需要射出一支箭，总共需要4支箭。
示例 3：

输入：points = [[1,2],[2,3],[3,4],[4,5]]
输出：2
解释：气球可以用2支箭来爆破:
- 在x = 2处发射箭，击破气球[1,2]和[2,3]。
- 在x = 4处射出箭，击破气球[3,4]和[4,5]。


提示:

1 <= points.length <= 105
points[i].length == 2
-231 <= xstart < xend <= 231 - 1
```
 */
pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    // 按照二维数组的左边界进行从小到大排序
    points.sort_by(|a, b| a[0].cmp(&b[0]));
    // 定义结果，初始化为一，默认需要一把剑
    let mut res = 1;
    for i in 1..points.len() {
        if points[i][0] > points[i - 1][1] {
            // 说明没有重合，需要多一把剑
            res += 1;
        } else {
            // 有重合，不需要多一把剑,更新右边界,取两个重合区间的最小右边界
            points[i][1] = points[i][1].min(points[i - 1][1]);
        }
    }
    res
}

/**
 * 435. 无重叠区间
中等
相关标签
相关企业

给定一个区间的集合 intervals ，其中 intervals[i] = [starti, endi] 。返回 需要移除区间的最小数量，使剩余区间互不重叠 。


```
示例 1:

输入: intervals = [[1,2],[2,3],[3,4],[1,3]]
输出: 1
解释: 移除 [1,3] 后，剩下的区间没有重叠。
示例 2:

输入: intervals = [ [1,2], [1,2], [1,2] ]
输出: 2
解释: 你需要移除两个 [1,2] 来使剩下的区间没有重叠。
示例 3:

输入: intervals = [ [1,2], [2,3] ]
输出: 0
解释: 你不需要移除任何区间，因为它们已经是无重叠的了。


提示:

1 <= intervals.length <= 105
intervals[i].length == 2
-5 * 104 <= starti < endi <= 5 * 104
```
 */
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    // 按照左边界的从小到大进行排序
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    // 定义结果
    let mut res = 0;
    for i in 1..intervals.len() {
        if intervals[i][0] < intervals[i - 1][1] {
            // 说明存在区间重复了
            res += 1;
            // 更新右边界,取两个重合区间的最小右边界
            intervals[i][1] = intervals[i][1].min(intervals[i - 1][1]);
        }
    }
    res
}

/**
 * 763. 划分字母区间
中等
相关标签
相关企业
提示

给你一个字符串 s 。我们要把这个字符串划分为尽可能多的片段，同一字母最多出现在一个片段中。

注意，划分结果需要满足：将所有划分结果按顺序连接，得到的字符串仍然是 s 。

返回一个表示每个字符串片段的长度的列表。


```
示例 1：
输入：s = "ababcbacadefegdehijhklij"
输出：[9,7,8]
解释：
划分结果为 "ababcbaca"、"defegde"、"hijhklij" 。
每个字母最多出现在一个片段中。
像 "ababcbacadefegde", "hijhklij" 这样的划分是错误的，因为划分的片段数较少。
示例 2：

输入：s = "eccbbbbdec"
输出：[10]


提示：

1 <= s.length <= 500
s 仅由小写英文字母组成
```
 */
pub fn partition_labels(s: String) -> Vec<i32> {
    let s = s.into_bytes();
    // 定义哈希表来存储字符以及最远出现的为止
    let mut record = vec![0; 130];
    // 遍历一遍字符串
    for (i, v) in s.iter().enumerate() {
        // 遍历完成后就记录了，字符和对应出现的最远位置
        record[*v as usize] = i;
    }
    // 定义结果
    let mut res = Vec::new();
    // 最远位置下标
    let mut max = 0;
    // 前一个分界线索引位置
    let mut pre = 0;
    for (i, v) in s.iter().enumerate() {
        max = max.max(record[*v as usize]);
        // 如果当前索引位置和最大下标位置相同，说明 i 左边的元素一定不会再 i 的右边出现
        if i == max {
            res.push((i - pre) as i32 + 1);
            pre = i + 1;
        }
    }
    res
}

/// 合并区间
/// 以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。
///
///
///
/// 示例 1：
///
/// 输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
/// 输出：[[1,6],[8,10],[15,18]]
/// 解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
/// 示例 2：
///
/// 输入：intervals = [[1,4],[4,5]]
/// 输出：[[1,5]]
/// 解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
///
///
/// 提示：
///
/// 1 <= intervals.length <= 104
/// intervals[i].length == 2
/// 0 <= starti <= endi <= 104
/// 这个问题可以通过以下步骤解决：
///
/// 首先，按照区间的开始位置对所有区间进行排序。
/// 然后，创建一个空的结果数组，并将第一个区间添加到结果数组中。
/// 遍历剩余的每个区间，如果当前区间的开始位置小于或等于结果数组中最后一个区间的结束位置，
/// 那么将结果数组中最后一个区间的结束位置更新为当前区间的结束位置和最后一个区间的结束位置中的较大值。
/// 否则，将当前区间添加到结果数组中。
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    // 使用数组中的左区间从小到大排序
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    // 定义结果
    let mut res = vec![intervals[0].clone()];
    for i in 1..intervals.len() {
        if res.last().unwrap()[1] < intervals[i][0] {
            // 如果 i 位置区间的左区间比 结果集中最后一个区间的右区间大说明没有重复，没有重复直接放入结果集中
            res.push(intervals[i].clone());
        } else {
            // res数组中最后一个元素
            let last = res.last_mut().unwrap();
            // 当前区间和结果集中最后一个区间的右区间取最大值
            last[1] = last[1].max(intervals[i][1]);
        }
    }
    res
}

/**
 * 738. 单调递增的数字
中等
相关标签
相关企业
提示

当且仅当每个相邻位数上的数字 x 和 y 满足 x <= y 时，我们称这个整数是单调递增的。

给定一个整数 n ，返回 小于或等于 n 的最大数字，且数字呈 单调递增 。


```
示例 1:

输入: n = 10
输出: 9
示例 2:

输入: n = 1234
输出: 1234
示例 3:

输入: n = 332
输出: 299


提示:

0 <= n <= 109
```
 */
pub fn monotone_increasing_digits(n: i32) -> i32 {
    let mut n_bytes = n.to_string().into_bytes();
    // 标记从哪位开始右边的都要处理为9
    let mut flag = n_bytes.len();
    for i in (1..n_bytes.len()).rev() {
        if n_bytes[i - 1] > n_bytes[i] {
            // 标记要处理为9的位置
            flag = i;
            // 左边的数减1
            n_bytes[i - 1] -= 1;
        }
    }
    for v in n_bytes.iter_mut().skip(flag) {
        // 右边的数都要处理为9 57是字符9的ASCII码
        *v = 57;
    }
    // 转换为数字
    n_bytes
        .into_iter()
        .fold(0, |acc, x| acc * 10 + x as i32 - 48)
}

/**
 * 2578. 最小和分割
简单
相关标签
相关企业
提示
给你一个正整数 num ，请你将它分割成两个非负整数 num1 和 num2 ，满足：

num1 和 num2 直接连起来，得到 num 各数位的一个排列。
换句话说，num1 和 num2 中所有数字出现的次数之和等于 num 中所有数字出现的次数。
num1 和 num2 可以包含前导 0 。
请你返回 num1 和 num2 可以得到的和的 最小 值。

注意：

num 保证没有前导 0 。
num1 和 num2 中数位顺序可以与 num 中数位顺序不同。


示例 1：

输入：num = 4325
输出：59
解释：我们可以将 4325 分割成 num1 = 24 和 num2 = 35 ，和为 59 ，59 是最小和。
示例 2：

输入：num = 687
输出：75
解释：我们可以将 687 分割成 num1 = 68 和 num2 = 7 ，和为最优值 75 。


提示：

10 <= num <= 109
 */
pub fn split_num(num: i32) -> i32 {
    // 排序加奇偶分组，把数字小的放在高位
    let mut s: Vec<u8> = num.to_string().bytes().collect();
    s.sort_unstable();
    let mut a = [0, 0];
    for (i, &c) in s.iter().enumerate() {
        a[i % 2] = a[i % 2] * 10 + c as i32 - '0' as i32;
    }
    a[0] + a[1]
}

/**
 * 409. 最长回文串

给定一个包含大写字母和小写字母的字符串 s ，返回 通过这些字母构造成的 最长的回文串 。

在构造过程中，请注意 区分大小写 。比如 "Aa" 不能当做一个回文字符串。



示例 1:

输入:s = "abccccdd"
输出:7
解释:
我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。
示例 2:

输入:s = "a"
输出:1
示例 3：

输入:s = "aaaaaccc"
输出:7


提示:

1 <= s.length <= 2000
s 只由小写 和/或 大写英文字母组成
 */
pub fn longest_palindrome(s: String) -> i32 {
    let mut res = 0;
    let mut add = 0;
    let bytes = s.into_bytes();
    // 定义哈希表来统计字符出现的个数
    let mut cnt = vec![0; 130];
    for b in bytes {
        cnt[b as usize] += 1;
    }
    for c in cnt {
        let rem = c % 2;
        res += c - rem;
        if rem == 1 {
            add = 1;
        }
    }
    res + add
}

/**
 * 976. 三角形的最大周长
简单
相关标签
相关企业
给定由一些正数（代表长度）组成的数组 nums ，返回 由其中三个长度组成的、面积不为零的三角形的最大周长 。
如果不能形成任何面积不为零的三角形，返回 0。



示例 1：

输入：nums = [2,1,2]
输出：5
解释：你可以用三个边长组成一个三角形:1 2 2。
示例 2：

输入：nums = [1,2,1,10]
输出：0
解释：
你不能用边长 1,1,2 来组成三角形。
不能用边长 1,1,10 来构成三角形。
不能用边长 1、2 和 10 来构成三角形。
因为我们不能用任何三条边长来构成一个非零面积的三角形，所以我们返回 0。


提示：

3 <= nums.length <= 104
1 <= nums[i] <= 106
 */
pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    for i in (0..nums.len() - 2).rev() {
        if nums[i] + nums[i + 1] > nums[i + 2] {
            return nums[i] + nums[i + 1] + nums[i + 2];
        }
    }
    0
}

/**
 * 605. 种花问题
简单
相关标签
相关企业
假设有一个很长的花坛，一部分地块种植了花，另一部分却没有。可是，花不能种植在相邻的地块上，它们会争夺水源，两者都会死去。

给你一个整数数组 flowerbed 表示花坛，由若干 0 和 1 组成，其中 0 表示没种植花，1 表示种植了花。另有一个数 n ，
能否在不打破种植规则的情况下种入 n 朵花？能则返回 true ，不能则返回 false 。



示例 1：

输入：flowerbed = [1,0,0,0,1], n = 1
输出：true
示例 2：

输入：flowerbed = [1,0,0,0,1], n = 2
输出：false


提示：

1 <= flowerbed.length <= 2 * 104
flowerbed[i] 为 0 或 1
flowerbed 中不存在相邻的两朵花
0 <= n <= flowerbed.length
 */
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut n = n;
    let mut left = 0;
    while left < flowerbed.len() - 1 {
        if flowerbed[left] == 0 && flowerbed[left + 1] == 0 {
            n -= 1;
            left += 2;
        } else if flowerbed[left] == 1 {
            left += 2;
        } else {
            // 如果不满足上面两个条件，说明flowerbed[left] == 0 && flowerbed[left + 1] == 1
            left += 1;
        }
    }
    // 处理最后一个位置
    if left == flowerbed.len() - 1 && flowerbed[left] == 0 {
        n -= 1;
    }
    n <= 0
}

/**
 * 2591. 将钱分给最多的儿童
简单
相关标签
提示
给你一个整数 money ，表示你总共有的钱数（单位为美元）和另一个整数 children ，表示你要将钱分配给多少个儿童。

你需要按照如下规则分配：

所有的钱都必须被分配。
每个儿童至少获得 1 美元。
没有人获得 4 美元。
请你按照上述规则分配金钱，并返回 最多 有多少个儿童获得 恰好 8 美元。如果没有任何分配方案，返回 -1 。



示例 1：

输入：money = 20, children = 3
输出：1
解释：
最多获得 8 美元的儿童数为 1 。一种分配方案为：
- 给第一个儿童分配 8 美元。
- 给第二个儿童分配 9 美元。
- 给第三个儿童分配 3 美元。
没有分配方案能让获得 8 美元的儿童数超过 1 。
示例 2：

输入：money = 16, children = 2
输出：2
解释：每个儿童都可以获得 8 美元。


提示：

1 <= money <= 200
2 <= children <= 30
 */
pub fn dist_money(money: i32, children: i32) -> i32 {
    if money < children {
        return -1;
    }
    if money == children {
        // 如果钱数和儿童数相等，那么每个儿童都可以获得1美元
        return 0;
    }

    let mut money = money;
    // 先给每个儿童分配1美元
    money -= children;
    // 贪心策略是尽可能多的儿童分配7美元，如果cnt大于children，说明有小孩一定会分配到多余8元
    let mut cnt = (money / 7).min(children);
    // 从剩下的钱中分配
    money -= cnt * 7;
    let mut children = children;
    // 到目前为止已经有cnt个小孩已经分配完成了，剩下的小孩数
    children -= cnt;
    // 若剩余 0 个人，并且 money>0，那么将所有的美元分配给一个已经分到 8 美元的人，令 cnt 减去 1
    // 若剩余 1 个人，并且 money=3，为了避免分到 4 美元，并注意到题目输入中的 children>=2，因此将这 3 美元拆成两部分，
    // 将其中的一部分分配给已经分到 8 美元的人，令 cnt 减去 1。
    if (children == 0 && money > 0) || (children == 1 && money == 3) {
        cnt -= 1;
    }
    cnt
}

/**
 * 1221. 分割平衡字符串
简单
相关标签
相关企业
提示
平衡字符串 中，'L' 和 'R' 字符的数量是相同的。

给你一个平衡字符串 s，请你将它分割成尽可能多的子字符串，并满足：

每个子字符串都是平衡字符串。
返回可以通过分割得到的平衡字符串的 最大数量 。



示例 1：

输入：s = "RLRRLLRLRL"
输出：4
解释：s 可以分割为 "RL"、"RRLL"、"RL"、"RL" ，每个子字符串中都包含相同数量的 'L' 和 'R' 。
示例 2：

输入：s = "RLRRRLLRLL"
输出：2
解释：s 可以分割为 "RL"、"RRRLLRLL"，每个子字符串中都包含相同数量的 'L' 和 'R' 。
注意，s 无法分割为 "RL"、"RR"、"RL"、"LR"、"LL" 因为第 2 个和第 5 个子字符串不是平衡字符串。
示例 3：

输入：s = "LLLLRRRR"
输出：1
解释：s 只能保持原样 "LLLLRRRR" 。


提示：

2 <= s.length <= 1000
s[i] = 'L' 或 'R'
s 是一个 平衡 字符串

解题思路：

一个合法的 LR 子串满足 L 字符和 R 字符数量相等，常规检查一个字符串是否为合格的 LR 子串可以使用 O(n)O(n)O(n) 的遍历方式，
可以使用记录前缀信息的数据结构，而对于成对结构的元素统计，更好的方式是转换为数学判定，使用 1 来代指 L 得分，使用 -1 来代指 R 得分。

那么一个子串为合格 LR 子串的充要条件为 整个 LR 子串的总得分为 0。

这种方式最早应该在 (题解) 301. 删除无效的括号 详细讲过，可延伸到任意的成对结构元素统计题目里去。
 */
pub fn balanced_string_split(s: String) -> i32 {
    let mut res = 0;
    let s = s.chars().collect::<Vec<char>>();
    // R用1替代，L用-1替代
    let mut cnt = 0;
    // 定义左指针
    let mut left = 0;
    while left < s.len() {
        if s[left] == 'R' {
            cnt += 1;
        } else {
            cnt -= 1;
        }
        if cnt == 0 {
            res += 1;
        }
        left += 1;
    }
    res
}

/**
 * 2656. K 个元素的最大和
简单
相关标签
给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。你需要执行以下操作 恰好 k 次，最大化你的得分：

从 nums 中选择一个元素 m 。
将选中的元素 m 从数组中删除。
将新元素 m + 1 添加到数组中。
你的得分增加 m 。
请你返回执行以上操作恰好 k 次后的最大得分。



示例 1：

输入：nums = [1,2,3,4,5], k = 3
输出：18
解释：我们需要从 nums 中恰好选择 3 个元素并最大化得分。
第一次选择 5 。和为 5 ，nums = [1,2,3,4,6] 。
第二次选择 6 。和为 6 ，nums = [1,2,3,4,7] 。
第三次选择 7 。和为 5 + 6 + 7 = 18 ，nums = [1,2,3,4,8] 。
所以我们返回 18 。
18 是可以得到的最大答案。
示例 2：

输入：nums = [5,5,5], k = 2
输出：11
解释：我们需要从 nums 中恰好选择 2 个元素并最大化得分。
第一次选择 5 。和为 5 ，nums = [5,5,6] 。
第二次选择 6 。和为 6 ，nums = [5,5,7] 。
所以我们返回 11 。
11 是可以得到的最大答案。


提示：

1 <= nums.length <= 100
1 <= nums[i] <= 100
1 <= k <= 100
 */
pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut res = Vec::new();
    let pop = nums.pop().unwrap();
    for _ in 0..k as usize {
        if res.is_empty() {
            res.push(pop);
        } else {
            res.push(res.last().unwrap() + 1);
        }
    }
    res.iter().sum()
}

/**
 * LCP 40. 心算挑战
简单
相关标签
相关企业
「力扣挑战赛」心算项目的挑战比赛中，要求选手从 N 张卡牌中选出 cnt 张卡牌，若这 cnt 张卡牌数字总和为偶数，则选手成绩「有效」且得分为 cnt 张卡牌数字总和。 给定数组 cards 和 cnt，其中 cards[i] 表示第 i 张卡牌上的数字。 请帮参赛选手计算最大的有效得分。若不存在获取有效得分的卡牌方案，则返回 0。

示例 1：

输入：cards = [1,2,8,9], cnt = 3

输出：18

解释：选择数字为 1、8、9 的这三张卡牌，此时可获得最大的有效得分 1+8+9=18。

示例 2：

输入：cards = [3,3,1], cnt = 1

输出：0

解释：不存在获取有效得分的卡牌方案。

提示：

1 <= cnt <= cards.length <= 10^5
1 <= cards[i] <= 1000
 */
pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
    let mut cards = cards;
    cards.sort_by(|a, b| b.cmp(&a));
    let cnt = cnt as usize;
    // 奇数和
    let mut odd = vec![0];
    // 偶数和
    let mut even = vec![0];
    for card in cards {
        // 如果是奇数
        if card & 1 != 0 {
            let v = odd[odd.len() - 1];
            odd.push(v + card);
        } else {
            // 如果是偶数
            let v = even[even.len() - 1];
            even.push(v + card);
        }
    }
    let mut ans = 0;
    for i in (0..odd.len()).step_by(2) {
        if cnt >= i && cnt < even.len() + i {
            ans = ans.max(odd[i] + even[cnt - i]);
        }
    }
    ans
}

/**
 * 强化练习 1 ：两个数对之间的最大乘积差
两个数对 (a, b) 和 (c, d) 之间的 乘积差 定义为 (a * b) - (c * d) 。

例如，(5, 6) 和 (2, 7) 之间的乘积差是 (5 * 6) - (2 * 7) = 16 。
给你一个整数数组 nums ，选出四个 不同的 下标 w、x、y 和 z ，使数对 (nums[w], nums[x]) 和 (nums[y], nums[z]) 之间的 乘积差 取到 最大值 。

返回以这种方式取得的乘积差中的 最大值 。



示例 1：

输入：nums = [5,6,2,7,4]
输出：34
解释：可以选出下标为 1 和 3 的元素构成第一个数对 (6, 7) 以及下标 2 和 4 构成第二个数对 (2, 4)
乘积差是 (6 * 7) - (2 * 4) = 34
示例 2：

输入：nums = [4,2,5,9,7,4,8]
输出：64
解释：可以选出下标为 3 和 6 的元素构成第一个数对 (9, 8) 以及下标 1 和 5 构成第二个数对 (2, 4)
乘积差是 (9 * 8) - (2 * 4) = 64


提示：

4 <= nums.length <= 104
1 <= nums[i] <= 104
 */
pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums[nums.len() - 1] * nums[nums.len() - 2] - nums[0] * nums[1]
}

/**
 * 强化练习 3：救生艇
给定数组 people 。people[i]表示第 i 个人的体重 ，船的数量不限，每艘船可以承载的最大重量为 limit。

每艘船最多可同时载两人，但条件是这些人的重量之和最多为 limit。

返回 承载所有人所需的最小船数 。



示例 1：

输入：people = [1,2], limit = 3
输出：1
解释：1 艘船载 (1, 2)
示例 2：

输入：people = [3,2,2,1], limit = 3
输出：3
解释：3 艘船分别载 (1, 2), (2) 和 (3)
示例 3：

输入：people = [3,5,3,4], limit = 5
输出：4
解释：4 艘船分别载 (3), (3), (4), (5)


提示：

1 <= people.length <= 5 * 104
1 <= people[i] <= limit <= 3 * 104
 */
pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort_unstable();
    let mut left = 0;
    let mut right = people.len() - 1;
    let mut res = 0;
    while left <= right {
        if left == right {
            // 说明只有一个人,分配一个船之后退出循环
            res += 1;
            break;
        } else if people[left] + people[right] > limit {
            // 如果最重的那个人和最轻的那个人加起来不能坐一条船，那么最重的那个人势必只能 "一意孤行" 了，因为其他人更加不可能和他同行。转变成
            right -= 1;
            res += 1;
        } else {
            // 如果最重的那个人可以和最轻的人一起坐一条船，那就顺带捎上，转变成
            res += 1;
            left += 1;
            right -= 1;
        }
    }
    res
}

/**
 * 324. 摆动排序 II
中等
相关标签
相关企业
给你一个整数数组 nums，将它重新排列成 nums[0] < nums[1] > nums[2] < nums[3]... 的顺序。

你可以假设所有输入数组都可以得到满足题目要求的结果。



示例 1：

输入：nums = [1,5,1,1,6,4]
输出：[1,6,1,5,1,4]
解释：[1,4,1,5,1,6] 同样是符合题目要求的结果，可以被判题程序接受。
示例 2：

输入：nums = [1,3,2,2,3,1]
输出：[2,3,1,3,1,2]


提示：

1 <= nums.length <= 5 * 104
0 <= nums[i] <= 5000
题目数据保证，对于给定的输入 nums ，总能产生满足题目要求的结果


进阶：你能用 O(n) 时间复杂度和 / 或原地 O(1) 额外空间来实现吗？
 */
pub fn wiggle_sort_ii(nums: &mut Vec<i32>) {
    let mut ret = nums.clone();
    ret.sort_unstable();
    let mut len = ret.len() - 1;
    // 先插奇数位
    for i in (1..ret.len()).step_by(2) {
        nums[i] = ret[len];
        len -= 1;
    }
    for i in (0..ret.len()).step_by(2) {
        nums[i] = ret[len];
        len -= 1;
    }
}

/**
 * 280. 摆动排序
中等
相关标签
相关企业
给你一个的整数数组 nums, 将该数组重新排序后使 nums[0] <= nums[1] >= nums[2] <= nums[3]...

输入数组总是有一个有效的答案。



示例 1:

输入：nums = [3,5,2,1,6,4]
输出：[3,5,1,6,2,4]
解释：[1,6,2,5,3,4]也是有效的答案
示例 2:

输入：nums = [6,6,5,6,3,8]
输出：[6,6,5,6,3,8]


提示：

1 <= nums.length <= 5 * 104
0 <= nums[i] <= 104
输入的 nums 保证至少有一个答案。



进阶：你能在 O(n) 时间复杂度下解决这个问题吗？
 */
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    let mut ret = nums.clone();
    ret.sort_unstable();
    let mut len = ret.len() - 1;
    // 先插奇数位
    for i in (1..ret.len()).step_by(2) {
        nums[i] = ret[len];
        len -= 1;
    }
    for i in (0..ret.len()).step_by(2) {
        nums[i] = ret[len];
        len -= 1;
    }
}

/**
 * 强化练习 6 ：最少操作使数组递增
给你一个整数数组 nums （下标从 0 开始）。每一次操作中，你可以选择数组中一个元素，并将它增加 1 。

比方说，如果 nums = [1,2,3] ，你可以选择增加 nums[1] 得到 nums = [1,3,3] 。
请你返回使 nums 严格递增 的 最少 操作次数。

我们称数组 nums 是 严格递增的 ，当它满足对于所有的 0 <= i < nums.length - 1 都有 nums[i] < nums[i+1] 。一个长度为 1 的数组是严格递增的一种特殊情况。



示例 1：

输入：nums = [1,1,1]
输出：3
解释：你可以进行如下操作：
1) 增加 nums[2] ，数组变为 [1,1,2] 。
2) 增加 nums[1] ，数组变为 [1,2,2] 。
3) 增加 nums[2] ，数组变为 [1,2,3] 。
示例 2：

输入：nums = [1,5,2,4,1]
输出：14
示例 3：

输入：nums = [8]
输出：0


提示：

1 <= nums.length <= 5000
1 <= nums[i] <= 104
 */
pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    // 初始化pre
    let mut pre = nums[0] + 1;
    for i in 1..nums.len() {
        if pre < nums[i] {
            // 如果当前数比pre大，就把pre替换为当前数+1
            pre = nums[i] + 1;
        } else {
            // 否则，需要把 当前的数 变成 前一个数加一
            res += pre - nums[i];
            pre += 1;
        }
    }
    res
}

/**
 * 强化练习 7 ：使数组唯一的最小增量
给你一个整数数组 nums 。每次 move 操作将会选择任意一个满足 0 <= i < nums.length 的下标 i，并将 nums[i] 递增 1。

返回使 nums 中的每个值都变成唯一的所需要的最少操作次数。



示例 1：

输入：nums = [1,2,2]
输出：1
解释：经过一次 move 操作，数组将变为 [1, 2, 3]。
示例 2：

输入：nums = [3,2,1,2,1,7]
输出：6
解释：经过 6 次 move 操作，数组将变为 [3, 4, 1, 2, 5, 7]。
可以看出 5 次或 5 次以下的 move 操作是不能让数组的每个值唯一的。


提示：
1 <= nums.length <= 105
0 <= nums[i] <= 105
 */
pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let (mut res, mut pre) = (0, nums[0] + 1);
    for i in 1..nums.len() {
        if pre < nums[i] {
            // 如果当前数比pre大，就把pre替换为当前数+1
            pre = nums[i] + 1;
        } else {
            // 否则，需要把 当前的数 变成 前一个数加一
            res += pre - nums[i];
            pre += 1;
        }
    }
    res
}

/**
 * 强化练习 8 ：有效三角形的个数
给定一个包含非负整数的数组 nums ，返回其中可以组成三角形三条边的三元组个数。



示例 1:

输入: nums = [2,2,3,4]
输出: 3
解释:有效的组合是:
2,3,4 (使用第一个 2)
2,3,4 (使用第二个 2)
2,2,3
示例 2:

输入: nums = [4,2,3,4]
输出: 4


提示:

1 <= nums.length <= 1000
0 <= nums[i] <= 1000
 */
pub fn triangle_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut nums = nums;
    // 首先，将所有数组元素按照递增排序
    nums.sort_unstable();
    // [2,nums.len() - 1] 左闭右闭区间
    for i in (2..=nums.len() - 1).rev() {
        // 固定 i i为最长的边，通过left和right指针来寻找另外两条边
        let (mut left, mut right) = (0, i - 1);
        while left < right {
            if nums[left] + nums[right] > nums[i] {
                // 如果left和right的和大于i，那么left和right之间的数都可以和right组成三角形
                res += right - left;
                right -= 1;
            } else {
                // 如果left和right的和小于等于i，那么left需要右移
                left += 1;
            }
        }
    }
    res as i32
}

/**
 * 1323. 6 和 9 组成的最大数字
简单
相关标签
相关企业
提示
给你一个仅由数字 6 和 9 组成的正整数 num。

你最多只能翻转一位数字，将 6 变成 9，或者把 9 变成 6 。

请返回你可以得到的最大数字。



示例 1：

输入：num = 9669
输出：9969
解释：
改变第一位数字可以得到 6669 。
改变第二位数字可以得到 9969 。
改变第三位数字可以得到 9699 。
改变第四位数字可以得到 9666 。
其中最大的数字是 9969 。
示例 2：

输入：num = 9996
输出：9999
解释：将最后一位从 6 变到 9，其结果 9999 是最大的数。
示例 3：

输入：num = 9999
输出：9999
解释：无需改变就已经是最大的数字了。


提示：

1 <= num <= 10^4
num 每一位上的数字都是 6 或者 9 。
 */
pub fn maximum69_number(num: i32) -> i32 {
    let mut nums = num.to_string().chars().collect::<Vec<char>>();
    for i in 0..nums.len() {
        if nums[i] == '6' {
            nums[i] = '9';
            break;
        }
    }
    nums.iter().collect::<String>().parse::<i32>().unwrap()
}

/**
 * 767. 重构字符串
中等
相关标签
相关企业
提示
给定一个字符串 s ，检查是否能重新排布其中的字母，使得两相邻的字符不同。

返回 s 的任意可能的重新排列。若不可行，返回空字符串 "" 。



示例 1:

输入: s = "aab"
输出: "aba"
示例 2:

输入: s = "aaab"
输出: ""


提示:

1 <= s.length <= 500
s 只包含小写字母
 */
pub fn reorganize_string(s: String) -> String {
    let n = s.len();
    let mut count = HashMap::new();
    for c in s.bytes() {
        *count.entry(c).or_insert(0) += 1;
    }
    // 转换为元祖后再按照值从大到小排序
    let mut a = count.into_iter().collect::<Vec<_>>();
    // 按出现次数从大到小排序
    a.sort_unstable_by(|p, q| q.1.cmp(&p.1));
    let m = a[0].1;
    if m > n - m + 1 {
        return "".to_string();
    }

    let mut ans = vec![b'\0'; n];
    let mut i = 0;
    for (ch, cnt) in a {
        for _ in 0..cnt {
            ans[i] = ch;
            i += 2;
            if i >= n {
                i = 1; // 从奇数下标开始填
            }
        }
    }
    unsafe { String::from_utf8_unchecked(ans) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves() {
        let nums = vec![1, 2, 3, 7, 4, 6, 9, 8];
        // sum + K * (n - 1) = n * (min + K)
        let min_moves = min_moves(nums);
        println!("{:?}", min_moves)
    }

    #[test]
    fn test_find_content_children() {
        let g = vec![1, 2, 3];
        let s = vec![3];
        let res = find_content_children(g, s);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_wiggle_max_length() {
        let nums = vec![1, 7, 4, 9, 2, 5];
        let res = wiggle_max_length(nums);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_can_jump() {
        let nums = vec![2, 0, 0];
        let res = can_jump(nums);
        assert_eq!(res, true);
    }

    #[test]
    fn test_jump() {
        let nums = vec![2, 3, 1, 1, 4];
        let res = jump(nums);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_largest_sum_after_k_negations() {
        let nums = vec![4, 2, 3];
        let k = 1;
        let res = largest_sum_after_k_negations(nums, k);
        assert_eq!(res, 13);
    }

    #[test]
    fn test_can_complete_circuit() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        let res = can_complete_circuit(gas, cost);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_candy() {
        let ratings = vec![1, 0, 2];
        let res = candy(ratings);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_lemonade_change() {
        let bills = vec![5, 5, 5, 10, 20];
        let res = lemonade_change(bills);
        assert_eq!(res, true);
    }

    #[test]
    fn test_find_min_arrow_shots() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let res = find_min_arrow_shots(points);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_erase_overlap_intervals() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        let res = erase_overlap_intervals(intervals);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_partition_labels() {
        let s = "ababcbacadefegdehijhklij".to_string();
        let res = partition_labels(s);
        assert_eq!(res, vec![9, 7, 8]);
    }

    #[test]
    fn test_merge() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(
            merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test_longest_palindrome() {
        let s = "abccccdd".to_string();
        let res = longest_palindrome(s);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_can_place_flowers() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let res = can_place_flowers(flowerbed, n);
        assert_eq!(res, true);
    }

    #[test]
    fn test_dist_money() {
        let money = 20;
        let children = 3;
        let res = dist_money(money, children);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_maximize_sum() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 3;
        let res = maximize_sum(nums, k);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_maxmium_score() {
        let cards = vec![10, 3];
        let cnt = 1;
        let res = maxmium_score(cards, cnt);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_min_operations() {
        let nums = vec![1, 5, 2, 4, 1];
        let res = min_operations(nums);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_min_increment_for_unique() {
        let nums = vec![3, 2, 1, 2, 1, 7];
        let res = min_increment_for_unique(nums);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_triangle_number() {
        let nums = vec![2, 2, 3, 4];
        let res = triangle_number(nums);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_maximum69_number() {
        let num = 9669;
        let res = maximum69_number(num);
        assert_eq!(res, 9969);
    }
}
