use std::{cell::RefCell, rc::Rc};

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
}
