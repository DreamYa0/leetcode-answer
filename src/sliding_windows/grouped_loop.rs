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

/// 2760. 最长奇偶子数组
///
/// 题意解读
///
/// 选一个最长连续子数组，满足子数组元素依次是偶数，奇数，偶数，奇数，……，且元素值均不超过 threshold\textit{threshold}threshold。
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