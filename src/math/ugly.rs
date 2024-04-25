/**
 * 263. 丑数
简单
相关标签
相关企业
丑数 就是只包含质因数 2、3 和 5 的正整数。

给你一个整数 n ，请你判断 n 是否为 丑数 。如果是，返回 true ；否则，返回 false 。



示例 1：

输入：n = 6
输出：true
解释：6 = 2 × 3
示例 2：

输入：n = 1
输出：true
解释：1 没有质因数，因此它的全部质因数是 {2, 3, 5} 的空集。习惯上将其视作第一个丑数。
示例 3：

输入：n = 14
输出：false
解释：14 不是丑数，因为它包含了另外一个质因数 7 。


提示：

-231 <= n <= 231 - 1
 */
pub fn is_ugly(n: i32) -> bool {
    let mut n = n;
    if n <= 0 {
        return false;
    }
    while n % 2 == 0 {
        n /= 2;
    }
    while n % 3 == 0 {
        n /= 3;
    }
    while n % 5 == 0 {
        n /= 5;
    }
    n == 1
}

/**
 * 264. 丑数 II
已解答
中等
相关标签
相关企业
提示
给你一个整数 n ，请你找出并返回第 n 个 丑数 。

丑数 就是质因子只包含 2、3 和 5 的正整数。



示例 1：

输入：n = 10
输出：12
解释：[1, 2, 3, 4, 5, 6, 8, 9, 10, 12] 是由前 10 个丑数组成的序列。
示例 2：

输入：n = 1
输出：1
解释：1 通常被视为丑数。


提示：

1 <= n <= 1690
 */
pub fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![1];
    let mut i = 1;
    let mut i2 = 0;
    let mut i3 = 0;
    let mut i5 = 0;
    while i < n {
        dp.push((dp[i2] * 2).min(dp[i3] * 3).min(dp[i5] * 5));
        while dp[i2] * 2 <= dp[i] {
            i2 += 1;
        }
        while dp[i3] * 3 <= dp[i] {
            i3 += 1;
        }
        while dp[i5] * 5 <= dp[i] {
            i5 += 1;
        }
        i += 1;
    }
    dp[n - 1]
}
