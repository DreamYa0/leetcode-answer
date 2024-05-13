use std::cmp::max;
/// 55.最大子数组和
/// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
///
/// 子数组 是数组中的一个连续部分。
///
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
/// https://leetcode.cn/problems/maximum-subarray/solutions/2361770/53-zui-da-zi-shu-zu-he-dong-tai-gui-hua-bvkq9/
/// 解题思路：
/// 动态规划是本题的最优解法，以下按照标准流程解题。
///
/// 常见解法	时间复杂度	空间复杂度
/// 暴力搜索	O(N^2)     O(1)
/// 分治思想	O(Nlog⁡N)	O(log⁡N)
/// 动态规划	O(N)	   O(1)
/// 动态规划解析：
///
/// 状态定义： 设动态规划列表 dp ，dp[i] 代表以元素 nums[i] 为结尾的连续子数组最大和。
///
/// 为何定义最大和 dp[i] 中必须包含元素 nums[i] ：保证 dp[i] 递推到 dp[i+1] 的正确性；如果不包含 nums[i]，递推时则不满足题目的 连续子数组 要求。
/// 转移方程： 若 dp[i−1]≤0 ，说明 dp[i−1] 对 dp[i]产生负贡献，即 dp[i−1]+nums[i] 还不如 nums[i] 本身大。
///
/// 初始状态： dp[0]=nums[0]，即以 nums[0] 结尾的连续子数组最大和为 nums[0] 。
///
/// 返回值： 返回 dp 列表中的最大值，代表全局最大值。
/// 状态压缩：
/// 由于 dp[i] 只与 dp[i−1] 和 nums[i] 有关系，因此可以将原数组 nums 用作 dp 列表，即直接在 nums 上修改即可。
/// 由于省去 dp 列表使用的额外空间，因此空间复杂度从 O(N) 降至 O(1) 。
/// <img class="marble" src="https://pic.leetcode-cn.com/77d1aa6a444743d3c8606ac951cd7fc38faf68a62064fd2639df517cd666a4d0-Picture1.png" alt="">
pub fn max_sub_array_dym(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut res = nums[0];
    for i in 1..nums.len() {
        nums[i] += max(nums[i - 1], 0);
        res = max(res, nums[i]);
    }
    res
}

/**
 * 509. 斐波那契数
简单
相关标签
相关企业

斐波那契数 （通常用 F(n) 表示）形成的序列称为 斐波那契数列 。该数列由 0 和 1 开始，后面的每一项数字都是前面两项数字的和。也就是：

F(0) = 0，F(1) = 1
F(n) = F(n - 1) + F(n - 2)，其中 n > 1
给定 n ，请计算 F(n) 。



示例 1：

输入：n = 2
输出：1
解释：F(2) = F(1) + F(0) = 1 + 0 = 1
示例 2：

输入：n = 3
输出：2
解释：F(3) = F(2) + F(1) = 1 + 1 = 2
示例 3：

输入：n = 4
输出：3
解释：F(4) = F(3) + F(2) = 2 + 1 = 3


提示：

0 <= n <= 30
 */
pub fn fib(n: i32) -> i32 {
    if n < 1 {
        return n;
    }
    // 1.dp[i]的定义: 第i个斐波那契数是dp[i]
    // 2.确定递推公式: dp[i] = dp[i-1] + dp[i-2]
    // 3.dp数组如何初始化: dp[0] = 0, dp[1] = 1
    // 4.确定遍历顺序: 从前往后
    // 5.举例推导dp数组: dp[2] = dp[1] + dp[0] = 1
    let n = n as usize;
    // 定义dp数组
    let mut dp = vec![0; n + 1];
    // 初始化dp数组
    dp[1] = 1;
    for i in 2..=n {
        // 递推公式
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

/**
 * 70. 爬楼梯
简单
相关标签
相关企业
提示
假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？



示例 1：

输入：n = 2
输出：2
解释：有两种方法可以爬到楼顶。
1. 1 阶 + 1 阶
2. 2 阶
示例 2：

输入：n = 3
输出：3
解释：有三种方法可以爬到楼顶。
1. 1 阶 + 1 阶 + 1 阶
2. 1 阶 + 2 阶
3. 2 阶 + 1 阶


提示：

1 <= n <= 45
 */
pub fn climb_stairs(n: i32) -> i32 {
    // 1.dp[i]的定义：爬到第i阶楼梯需要dp[i]种方法
    // 2.递推公式：dp[i] = dp[i-1] + dp[i-2]
    // 3.dp数组初始化：dp[1] = 1, dp[2] = 2
    // 4.遍历顺序：从前往后
    // 5.举例推导dp数组：dp[3] = dp[2] + dp[1] = 3
    let n = n as usize;
    // 默认长度为n+2，因为dp[i] i从1开始
    let mut dp = vec![0; n + 2];
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

/**
 * 1137. 第 N 个泰波那契数
简单
相关标签
相关企业
提示
泰波那契序列 Tn 定义如下：

T0 = 0, T1 = 1, T2 = 1, 且在 n >= 0 的条件下 Tn+3 = Tn + Tn+1 + Tn+2

给你整数 n，请返回第 n 个泰波那契数 Tn 的值。



示例 1：

输入：n = 4
输出：4
解释：
T_3 = 0 + 1 + 1 = 2
T_4 = 1 + 1 + 2 = 4
示例 2：

输入：n = 25
输出：1389537


提示：

0 <= n <= 37
答案保证是一个 32 位整数，即 answer <= 2^31 - 1。
 */
pub fn tribonacci(n: i32) -> i32 {
    // 1.dp[i]的定义：第i个泰波那契数是dp[i]
    // 2.递推公式：dp[i] = dp[i-1] + dp[i-2] + dp[i-3]
    // 3.dp数组初始化：dp[0] = 0, dp[1] = 1, dp[2] = 1
    // 4.遍历顺序：从前往后
    // 5.举例推导dp数组：dp[3] = dp[2] + dp[1] + dp[0] = 2
    let n = n as usize;
    let mut dp = vec![0; n + 3];
    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 1;
    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
    }
    dp[n]
}

/**
 * 746. 使用最小花费爬楼梯
简单
相关标签
相关企业
提示
给你一个整数数组 cost ，其中 cost[i] 是从楼梯第 i 个台阶向上爬需要支付的费用。一旦你支付此费用，即可选择向上爬一个或者两个台阶。

你可以选择从下标为 0 或下标为 1 的台阶开始爬楼梯。

请你计算并返回达到楼梯顶部的最低花费。



示例 1：

输入：cost = [10,15,20]
输出：15
解释：你将从下标为 1 的台阶开始。
- 支付 15 ，向上爬两个台阶，到达楼梯顶部。
总花费为 15 。
示例 2：

输入：cost = [1,100,1,1,1,100,1,1,100,1]
输出：6
解释：你将从下标为 0 的台阶开始。
- 支付 1 ，向上爬两个台阶，到达下标为 2 的台阶。
- 支付 1 ，向上爬两个台阶，到达下标为 4 的台阶。
- 支付 1 ，向上爬两个台阶，到达下标为 6 的台阶。
- 支付 1 ，向上爬一个台阶，到达下标为 7 的台阶。
- 支付 1 ，向上爬两个台阶，到达下标为 9 的台阶。
- 支付 1 ，向上爬一个台阶，到达楼梯顶部。
总花费为 6 。


提示：

2 <= cost.length <= 1000
0 <= cost[i] <= 999
 */
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    // 1. dp[i] 到达i位置最小花费为dp[i]
    // 2. 递推公式 dp[i] = min(dp[i-1] + cost[i-1] , dp[i-2] + cost[i-2])
    // 3. 初始化dp数组，dp[0] = 0, dp[1] = 0
    // 4. 遍历顺序：从前往后
    // 5. 打印dp数组
    let mut dp = vec![0; cost.len() + 1];
    for i in 2..=cost.len() {
        dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
    }
    println!("{:?}", dp);
    dp[cost.len()]
}

/**
 * 62. 不同路径
中等
相关标签
相关企业
一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。

问总共有多少条不同的路径？



示例 1：

<img src="https://pic.leetcode.cn/1697422740-adxmsI-image.png" alt="">

输入：m = 3, n = 7
输出：28
示例 2：

输入：m = 3, n = 2
输出：3
解释：
从左上角开始，总共有 3 条路径可以到达右下角。
1. 向右 -> 向下 -> 向下
2. 向下 -> 向下 -> 向右
3. 向下 -> 向右 -> 向下
示例 3：

输入：m = 7, n = 3
输出：28
示例 4：

输入：m = 3, n = 3
输出：6


提示：

1 <= m, n <= 100
题目数据保证答案小于等于 2 * 109
 */
pub fn unique_paths(m: i32, n: i32) -> i32 {
    // 1. 定义dp数组的含义：dp[i][j]表示到达i,j位置的路径数
    // 2. 递推公式 dp[i][j] = dp[i-1][j] + dp[i][j-1]
    // 3. 初始化dp数组：dp[0][j] = 1 dp[i][0] = 1
    // 4. 遍历顺序：从上往下遍历，从左往右遍历
    // 5. 打印dp数组
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![vec![1; n]; m];
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array_dym() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array_dym(nums), 6);
    }

    #[test]
    fn test_climb_stairs() {
        let n = 1;
        assert_eq!(climb_stairs(n), 1);
    }

    #[test]
    fn test_min_cost_climbing_stairs() {
        let cost = vec![10, 15, 20];
        assert_eq!(min_cost_climbing_stairs(cost), 15);
    }

    #[test]
    fn test_unique_paths() {
        let m = 3;
        let n = 7;
        assert_eq!(unique_paths(m, n), 28);
    }
}
