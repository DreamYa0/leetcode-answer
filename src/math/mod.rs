/**
 * 强化练习 4： Pow(x, n) 实现 pow(x, n) ，即计算 x 的整数 n 次幂函数（即，xn ）。
 */
pub fn my_pow(x: f64, n: i32) -> f64 {
    x.powi(n)
}

/**
 * 强化练习 6 ： Sqrt(x)
给你一个非负整数 x ，计算并返回 x 的 算术平方根 。

由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。

注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。



示例 1：

输入：x = 4
输出：2
示例 2：

输入：x = 8
输出：2
解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。


提示：

0 <= x <= 231 - 1
 */
pub fn my_sqrt(x: i32) -> i32 {
    (x as f64).sqrt() as i32
}

/**
 * 强化练习 1：2 的幂
给你一个整数 n，请你判断该整数是否是 2 的幂次方。如果是，返回 true ；否则，返回 false 。

如果存在一个整数 x 使得 n == 2x ，则认为 n 是 2 的幂次方。



示例 1：

输入：n = 1
输出：true
解释：20 = 1
示例 2：

输入：n = 16
输出：true
解释：24 = 16
示例 3：

输入：n = 3
输出：false
示例 4：

输入：n = 4
输出：true
示例 5：

输入：n = 5
输出：false


提示：

-231 <= n <= 231 - 1


进阶：你能够不使用循环/递归解决此问题吗？
 */
pub fn is_power_of_two(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    if n == 1 {
        return true;
    }
    let mut num = 1;
    for _ in 0..31 {
        num = num << 1;
        if num == n {
            return true;
        }
    }
    return false;
}

/**
 * 强化练习 2 ：3 的幂
给定一个整数，写一个函数来判断它是否是 3 的幂次方。如果是，返回 true ；否则，返回 false 。

整数 n 是 3 的幂次方需满足：存在整数 x 使得 n == 3x



示例 1：

输入：n = 27
输出：true
示例 2：

输入：n = 0
输出：false
示例 3：

输入：n = 9
输出：true
示例 4：

输入：n = 45
输出：false


提示：

-231 <= n <= 231 - 1


进阶：你能不使用循环或者递归来完成本题吗？
 */
pub fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    if n == 1 {
        return true;
    }
    let mut num = 1;
    for _ in 0..20 {
        num = num * 3;
        if num == n {
            return true;
        }
    }
    return false;
}

/**
 * 强化练习 3 ： 4 的幂
给定一个整数，写一个函数来判断它是否是 4 的幂次方。如果是，返回 true ；否则，返回 false 。

整数 n 是 4 的幂次方需满足：存在整数 x 使得 n == 4x



示例 1：

输入：n = 16
输出：true
示例 2：

输入：n = 5
输出：false
示例 3：

输入：n = 1
输出：true


提示：

-231 <= n <= 231 - 1


进阶：你能不使用循环或者递归来完成本题吗？
 */
pub fn is_power_of_four(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    if n == 1 {
        return true;
    }
    let mut num = 1;
    for _ in 0..16 {
        num = num << 2;
        if num == n {
            return true;
        }
    }
    return false;
}

/**
 * 强化练习 4 ：n 的第 k 个因子
给你两个正整数 n 和 k 。

如果正整数 i 满足 n % i == 0 ，那么我们就说正整数 i 是整数 n 的因子。

考虑整数 n 的所有因子，将它们 升序排列 。请你返回第 k 个因子。如果 n 的因子数少于 k ，请你返回 -1 。



示例 1：

输入：n = 12, k = 3
输出：3
解释：因子列表包括 [1, 2, 3, 4, 6, 12]，第 3 个因子是 3 。
示例 2：

输入：n = 7, k = 2
输出：7
解释：因子列表包括 [1, 7] ，第 2 个因子是 7 。
示例 3：

输入：n = 4, k = 4
输出：-1
解释：因子列表包括 [1, 2, 4] ，只有 3 个因子，所以我们应该返回 -1 。


提示：

1 <= k <= n <= 1000


进阶：

你可以设计时间复杂度小于 O(n) 的算法来解决此问题吗？
 */
pub fn kth_factor(n: i32, k: i32) -> i32 {
    let mut count = 0;
    for i in 1..=n {
        if n % i == 0 {
            count += 1;
            if count == k {
                return i;
            }
        }
    }
    -1
}

/**
 * 强化练习 5 ：有效的完全平方数
给你一个正整数 num 。如果 num 是一个完全平方数，则返回 true ，否则返回 false 。

完全平方数 是一个可以写成某个整数的平方的整数。换句话说，它可以写成某个整数和自身的乘积。

不能使用任何内置的库函数，如  sqrt 。



示例 1：

输入：num = 16
输出：true
解释：返回 true ，因为 4 * 4 = 16 且 4 是一个整数。
示例 2：

输入：num = 14
输出：false
解释：返回 false ，因为 3.742 * 3.742 = 14 但 3.742 不是一个整数。


提示：

1 <= num <= 231 - 1
 */
pub fn is_perfect_square(num: i32) -> bool {
    let mut i = 1;
    while i * i < num {
        i += 1;
    }
    i * i == num
}

/**
 * 829. 连续整数求和
困难
相关标签
相关企业
给定一个正整数 n，返回 连续正整数满足所有数字之和为 n 的组数 。



示例 1:

输入: n = 5
输出: 2
解释: 5 = 2 + 3，共有两组连续整数([5],[2,3])求和后为 5。
示例 2:

输入: n = 9
输出: 3
解释: 9 = 4 + 5 = 2 + 3 + 4
示例 3:

输入: n = 15
输出: 4
解释: 15 = 8 + 7 = 4 + 5 + 6 = 1 + 2 + 3 + 4 + 5


提示:

1 <= n <= 109​​​​​​​

解题思路：


 */
pub fn consecutive_numbers_sum(n: i32) -> i32 {
    let mut ans = 0;
    let n = n * 2;
    let mut k = 1;
    while k * k < n {
        if k % n != 0 {
            continue;
        }
        if (n / k - (k - 1)) % 2 == 0 {
            ans += 1;
        }
        k += 1;
    }
    ans
}

/**
 * 强化练习 1：一周中的第几天
给你一个日期，请你设计一个算法来判断它是对应一周中的哪一天。

输入为三个整数：day、month 和 year，分别表示日、月、年。

您返回的结果必须是这几个值中的一个 {"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}。



示例 1：

输入：day = 31, month = 8, year = 2019
输出："Saturday"
示例 2：

输入：day = 18, month = 7, year = 1999
输出："Sunday"
示例 3：

输入：day = 15, month = 8, year = 1993
输出："Sunday"


提示：

给出的日期一定是在 1971 到 2100 年之间的有效日期。
 */
pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
    let days = vec![
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let t = vec![0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut year = year;
    if month < 3 {
        year -= 1;
    }
    let mut res = (year + year / 4 - year / 100 + year / 400 + t[(month - 1) as usize] + day) % 7;
    res = (res + 7) % 7;
    days[res as usize].to_string()
}

/**
 * 强化练习 2 ：一年中的第几天
给你一个字符串 date ，按 YYYY-MM-DD 格式表示一个 现行公元纪年法 日期。返回该日期是当年的第几天。



示例 1：

输入：date = "2019-01-09"
输出：9
解释：给定日期是2019年的第九天。
示例 2：

输入：date = "2019-02-10"
输出：41


提示：

date.length == 10
date[4] == date[7] == '-'，其他的 date[i] 都是数字
date 表示的范围从 1900 年 1 月 1 日至 2019 年 12 月 31 日
 */
pub fn day_of_year(date: String) -> i32 {
    let date = date.split("-").collect::<Vec<&str>>();
    let year = date[0].parse::<i32>().unwrap();
    let month = date[1].parse::<i32>().unwrap();
    let day = date[2].parse::<i32>().unwrap();
    let mut days = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        days[2] = 29;
    }
    let mut res = 0;
    for i in 0..month {
        res += days[i as usize];
    }
    res += day;
    res
}

/**
 * 强化练习 3 ：日期之间隔几天
请你编写一个程序来计算两个日期之间隔了多少天。

日期以字符串形式给出，格式为 YYYY-MM-DD，如示例所示。



示例 1：

输入：date1 = "2019-06-29", date2 = "2019-06-30"
输出：1
示例 2：

输入：date1 = "2020-01-15", date2 = "2019-12-31"
输出：15


提示：

给定的日期是 1971 年到 2100 年之间的有效日期
 */
pub fn days_between_dates(date1: String, date2: String) -> i32 {
    let is_leap = |year: i32| (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let str_to_year = |date: String| -> i32 {
        let month_to_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut it = date.split('-');
        let year = it.next().unwrap().parse::<i32>().unwrap();
        let month = it.next().unwrap().parse::<i32>().unwrap();
        let mut days = it.next().unwrap().parse::<i32>().unwrap();

        for i in 1970..year {
            days += if is_leap(i) { 366 } else { 365 };
        }

        for i in 1..month {
            if i == 2 && is_leap(year) {
                days += 1;
            }
            days += month_to_days[i as usize - 1];
        }

        days
    };
    (str_to_year(date1) - str_to_year(date2)).abs()
}

/**
 * 强化练习 1：回文素数
给你一个整数 n ，返回大于或等于 n 的最小回文质数。

一个整数如果恰好有两个除数：1 和它本身，那么它是 质数 。注意，1 不是质数。

例如，2、3、5、7、11 和 13 都是质数。
一个整数如果从左向右读和从右向左读是相同的，那么它是 回文数 。

例如，101 和 12321 都是回文数。
测试用例保证答案总是存在，并且在 [2, 2 * 108] 范围内。



示例 1：

输入：n = 6
输出：7
示例 2：

输入：n = 8
输出：11
示例 3：

输入：n = 13
输出：101


提示：

1 <= n <= 108
 */
pub fn prime_palindrome(n: i32) -> i32 {
    if n == 1 {
        // 2 是最小的素数
        return 2;
    }
    let mut n = n;
    loop {
        // 遍历所有数字，检查是不是回文串。如果是，检查是不是素数
        if is_palindrome(n) && is_prime(n) {
            return n;
        }
        n += 1;
        // 如果当前数字长度为 8，可以跳过检查，因为不存在 8 长度的素数
        if 10_000_000 < n && n < 100_000_000 {
            n = 100_000_000;
        }
    }
}

/// 判断是否是素数
fn is_prime(n: i32) -> bool {
    let sqrtn = (n as f64).sqrt();
    for i in 2..=sqrtn as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// 判断是否是回文数
fn is_palindrome(n: i32) -> bool {
    let mut m = 0;
    let mut t = n;
    while t > 0 {
        m = m * 10 + t % 10;
        t /= 10;
    }
    n == m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        let n = 16;
        assert_eq!(is_power_of_two(n), true);
    }

    #[test]
    fn test_consecutive_numbers_sum() {
        let n = 15;
        assert_eq!(consecutive_numbers_sum(n), 4);
    }

    #[test]
    fn test_prime_palindrome() {
        let n = 13;
        assert_eq!(prime_palindrome(n), 101);
    }

    #[test]
    fn test_is_palindrome() {
        let n = 121;
        assert_eq!(is_palindrome(n), true);
    }

    #[test]
    fn test_is_prime() {
        let n = 13;
        assert_eq!(is_prime(n), true);
    }
}
