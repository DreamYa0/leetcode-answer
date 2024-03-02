pub mod replace;
pub mod reverse;

/// 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
///
/// 字符          数值
/// I             1
/// V             5
/// X             10
///  L             50
///  C             100
///  D             500
///  M             1000
///
///  例如， 罗马数字 2 写做 II ，即为两个并列的 1 。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
///  通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
///  I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
///  X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
///  C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
///  给定一个罗马数字，将其转换成整数。
///
///  示例 1:
///  输入: s = "III"
///  输出: 3
///
///  示例 2:
///  输入: s = "IV"
///  输出: 4
///
///  示例 3:
///  输入: s = "IX"
///  输出: 9
///
///  示例 4:
///  输入: s = "LVIII"
///  输出: 58
///  解释: L = 50, V= 5, III = 3.
///
///  示例 5:
///  输入: s = "MCMXCIV"
///  输出: 1994
///  解释: M = 1000, CM = 900, XC = 90, IV = 4.
///
///  提示：
///  1 <= s.length <= 15
///  s 仅含字符 ('I', 'V', 'X', 'L', 'C', 'D', 'M')
///  题目数据保证 s 是一个有效的罗马数字，且表示整数在范围 [1, 3999] 内
///  题目所给测试用例皆符合罗马数字书写规则，不会出现跨位等情况。
///  IL 和 IM 这样的例子并不符合题目要求，49 应该写作 XLIX，999 应该写作 CMXCIX 。
/// 关于罗马数字的详尽书写规则，可以参考 罗马数字 - Mathematics 。
pub fn roman_to_int(s: String) -> i32 {
    // 用byte达到最高效率，反向迭代，若上一数字大于当前数字则减去，反之则加
    let mut res = 0;
    let mut last = 0;
    for c in s.chars().rev() {
        let n = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!(),
        };
        res += if n < last { -n } else { n };
        last = n;
    }
    res
}

/// 1614. 括号的最大嵌套深度
pub fn max_depth(s: String) -> i32 {
    // 定义最大嵌套深度
    let mut max_len = 0;
    // 定义当前嵌套深度
    let mut cur_len = 0;
    // 遍历栈找寻合适的左右括号
    for (_, c) in s.chars().enumerate() {
        if c == '(' {
            cur_len += 1;
        }
        if c == ')' {
            cur_len -= 1;
        }
        max_len = max_len.max(cur_len);
    }
    max_len as i32
}

/// 1021. 删除最外层的括号
pub fn remove_outer_parentheses(s: String) -> String {
    // 存放结果
    let mut cns = Vec::with_capacity(s.len());
    // 统计
    let mut cnt = 0;
    // 遍历字符串
    for c in s.chars() {
        if c == ')' {
            cnt -= 1;
        }
        if cnt >= 1 {
            cns.push(c)
        }
        if c == '(' {
            cnt += 1;
        }
    }
    cns.iter().collect()
}

/// 1332. 删除回文子序列
pub fn remove_palindrome_sub(s: String) -> i32 {
    let len = s.len();
    let s = s.chars().collect::<Vec<char>>();
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = len - 1;
    while left < right {
        // 如果不相等说明字符串自身不是回文字符串，则需要删除两次，否则只需要删除一次
        if s[left] != s[right] {
            return 2;
        }
        left += 1;
        right -= 1;
    }
    1
}

/// 1446. 连续字符
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

/// 1869. 哪种连续子字符串更长
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

/// 1957. 删除字符使字符串变好
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

/// 2038. 如果相邻两个颜色均相同则删除当前颜色
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

/// 1759. 统计同质子字符串的数目
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

/// 228. 汇总区间
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

/// 2765. 最长交替子数组
/// 对于本题来说，在内层循环时，假设这一组的第一个数是 333，那么这一组的数字必须形如 3,4,3,4,⋯，也就是
///
/// ```
/// nums[i]=nums[i−2]
/// ```
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

/// 2110. 股票平滑下跌阶段的数目
///
/// 将 prices 按照平滑下降的定义分成若干组。例如 [3,2,1,4] 分为 [3,2,1] 和 [4] 两组。
///
/// 对于每一组的所有非空子数组，都是平滑下降的。设该组长度为 m，则该组的非空子数组个数为 m*(m+1)/2。
///
/// 累加每组的非空子区间个数即为答案。
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

/// 1578. 使绳子变成彩色的最短时间
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

/// 1839. 所有元音按顺序排布的最长子字符串
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let roman_to_int = roman_to_int("MCMXCIV".to_string());
        println!("{:?}", roman_to_int)
    }

    #[test]
    fn test_max_depth() {
        let s = "(1+(2*3)+((8)/4))+1".to_string();
        assert_eq!(max_depth(s), 3);
    }

    #[test]
    fn test_remove_outer_parentheses() {
        let s = "(()())(())".to_string();
        assert_eq!(remove_outer_parentheses(s), "()()()");
    }

    #[test]
    fn test_remove_palindrome_sub() {
        let s = "ababa".to_string();
        assert_eq!(remove_palindrome_sub(s), 1);
    }

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
