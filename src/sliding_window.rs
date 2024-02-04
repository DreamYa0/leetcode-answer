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
    // 用来标记窗口内是否存在重复元素，也可以用 HashSet，这里为了效率用的 Vec
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

/// 209. 长度最小的子数组
/// 给定一个含有 n 个正整数的数组和一个正整数 target 。
///
/// 找出该数组中满足其总和大于等于 target 的长度最小的 连续子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。
///
/// 示例 1：
///
/// 输入：target = 7, nums = [2,3,1,2,4,3]
/// 输出：2
/// 解释：子数组 [4,3] 是该条件下的长度最小的子数组。
/// 示例 2：
///
/// 输入：target = 4, nums = [1,4,4]
/// 输出：1
/// 示例 3：
///
/// 输入：target = 11, nums = [1,1,1,1,1,1,1,1]
/// 输出：0
///
///
/// 提示：
///
/// 1 <= target <= 109
/// 1 <= nums.length <= 105
/// 1 <= nums[i] <= 105
///
///
/// 进阶：
///
/// 如果你已经实现 O(n) 时间复杂度的解法, 请尝试设计一个 O(n log(n)) 时间复杂度的解法。
///
/// 滑动窗口
///
/// 接下来就开始介绍数组操作中另一个重要的方法：滑动窗口。
///
/// 所谓滑动窗口，就是不断的调节子序列的起始位置和终止位置，从而得出我们要想的结果。
///
/// 在暴力解法中，是一个for循环滑动窗口的起始位置，一个for循环为滑动窗口的终止位置，用两个for循环 完成了一个不断搜索区间的过程。
///
/// 那么滑动窗口如何用一个for循环来完成这个操作呢。
///
/// 首先要思考 如果用一个for循环，那么应该表示 滑动窗口的起始位置，还是终止位置。
///
/// 如果只用一个for循环来表示 滑动窗口的起始位置，那么如何遍历剩下的终止位置？
///
/// 此时难免再次陷入 暴力解法的怪圈。
///
/// 所以 只用一个for循环，那么这个循环的索引，一定是表示 滑动窗口的终止位置。
///
/// 那么问题来了， 滑动窗口的起始位置如何移动呢？
///
/// 这里还是以题目中的示例来举例，s=7， 数组是 2，3，1，2，4，3，来看一下查找的过程：
/// <img class="marble" src="https://code-thinking.cdn.bcebos.com/gifs/209.%E9%95%BF%E5%BA%A6%E6%9C%80%E5%B0%8F%E7%9A%84%E5%AD%90%E6%95%B0%E7%BB%84.gif" alt="">
///
/// 最后找到 4，3 是最短距离。
///
/// 其实从动画中可以发现滑动窗口也可以理解为双指针法的一种！只不过这种解法更像是一个窗口的移动，所以叫做滑动窗口更适合一些。
///
/// 在本题中实现滑动窗口，主要确定如下三点：
///
/// 窗口内是什么？
/// 如何移动窗口的起始位置？
/// 如何移动窗口的结束位置？
/// 窗口就是 满足其和 ≥ s 的长度最小的 连续 子数组。
///
/// 窗口的起始位置如何移动：如果当前窗口的值大于s了，窗口就要向前移动了（也就是该缩小了）。
///
/// 窗口的结束位置如何移动：窗口的结束位置就是遍历数组的指针，也就是for循环里的索引。
///
/// 解题的关键在于 窗口的起始位置如何移动，如图所示：
/// <img class="marble" src="https://code-thinking-1253855093.file.myqcloud.com/pics/20210312160441942.png" alt="">
///
/// 可以发现滑动窗口的精妙之处在于根据当前子序列和大小的情况，不断调节子序列的起始位置。从而将O(n^2)暴力解法降为O(n)。
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // 定义起始指针
    let mut left = 0;
    // 定义和
    let mut sum = 0;
    // 定义结果
    let mut res = std::i32::MAX;
    for right in 0..nums.len() {
        // 累加
        sum += nums[right];
        // 循环的条件，当和大于等于目标值时，就要开始缩小窗口了
        while sum >= target {
            // (right - left + 1) 子序列的长度
            res = res.min((right - left + 1) as i32);
            // 调整窗口
            sum -= nums[left];
            // 左指针右移
            left += 1;
        }
    }
    // 如果res没有被赋值，就返回0
    if res == std::i32::MAX {
        0
    } else {
        res
    }
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
    // 最大子串 fast - slow + 1 = nums[slow] + nums[slow + 1]..nums[fast]
    for fast in 0..nums.len() {
        // 统计 nums[slow] + nums[slow + 1]..nums[fast] 的和
        let mut sum = 0;
        for i in slow..=fast {
            sum += nums[i];
        }

        while (slow < fast) && (fast - slow + 1) as i32 != sum {
            slow += 1;
        }
        max_len = max_len.max(sum);
    }
    max_len
}

/// 438.找到字符串中所有字母的异位词
/// 给定两个字符串 s 和 p，找到 s 中所有 p 的 异位词 的子串，返回这些子串的起始索引。不考虑答案输出的顺序。
///
/// 异位词 指由相同字母重排列形成的字符串（包括相同的字符串）。
///
///
/// 示例 1:
///
/// 输入: s = "cbaebabacd", p = "abc"
/// 输出: [0,6]
/// 解释:
/// 起始索引等于 0 的子串是 "cba", 它是 "abc" 的异位词。
/// 起始索引等于 6 的子串是 "bac", 它是 "abc" 的异位词。
///  示例 2:
///
/// 输入: s = "abab", p = "ab"
/// 输出: [0,1,2]
/// 解释:
/// 起始索引等于 0 的子串是 "ab", 它是 "ab" 的异位词。
/// 起始索引等于 1 的子串是 "ba", 它是 "ab" 的异位词。
/// 起始索引等于 2 的子串是 "ab", 它是 "ab" 的异位词。
///
/// 提示:
///
/// 1 <= s.length, p.length <= 3 * 104
/// s 和 p 仅包含小写字母
///
/// 解题思路
/// 滑动窗口
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut res = Vec::with_capacity(s.len());
    let (s_len, p_len) = (s.len(), p.len());
    if s_len < p_len {
        return res;
    }

    let mut tab = [0; 26];
    // 定义滑动窗口
    let mut windows = [0; 26];

    // 初始化 tab数组,统计p字符串中每个字符出现的次数
    for i in p.as_bytes() {
        // [0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        tab[(i - b'a') as usize] += 1;
    }

    // 初始化windows数组，从s字符串中取p_len长度的字符放到windows中
    for i in 0..p_len {
        // 遍历到m前一个元素截止，m是开区间
        // [0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        windows[(s.as_bytes()[i] - b'a') as usize] += 1;
        if tab == windows {
            // 如果出现相等，则把起始索引放入结果集中
            res.push(0);
        }
    }

    // 开始滑动窗口，滑动范围是从m到n
    for i in p_len..s_len {
        // [0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        windows[(s.as_bytes()[i] - b'a') as usize] += 1;
        // 从 cur数组中剪掉滑动窗口起始点之前的数据
        // 第一次循环是 i = m - 1 第一次循环窗口往前滑动一位，所以就需要把windows[0]位置的数据减掉
        windows[(s.as_bytes()[i - p_len] - b'a') as usize] -= 1;
        if windows == tab {
            // 如果出现相等，则把起始索引放入结果集中
            res.push((i - p_len + 1) as i32);
        }
    }
    res
}

/// 强化练习 1：定长子串中元音的最大数目
///
/// 给你字符串 s 和整数 k 。
///
/// 请返回字符串 s 中长度为 k 的单个子字符串中可能包含的最大元音字母数。
///
/// 英文中的 元音字母 为（a, e, i, o, u）。
///
/// 示例 1：
///
/// 输入：s = "abciiidef", k = 3
///
/// 输出：3
///
/// 解释：子字符串 "iii" 包含 3 个元音字母。
///
/// 示例 2：
///
/// 输入：s = "aeiou", k = 2
///
/// 输出：2
///
/// 解释：任意长度为 2 的子字符串都包含 2 个元音字母。
///
/// 示例 3：
///
/// 输入：s = "leetcode", k = 3
///
/// 输出：2
///
/// 解释："lee"、"eet" 和 "ode" 都包含 2 个元音字母。
///
/// 示例 4：
///
/// 输入：s = "rhythms", k = 4
///
/// 输出：0
///
/// 解释：字符串 s 中不含任何元音字母。
///
/// 示例 5：
///
/// 输入：s = "tryhard", k = 4
///
/// 输出：1
///
/// 提示：
///
/// 1 <= s.length <= 10^5
///
/// s 由小写英文字母组成
///
/// 1 <= k <= s.length
///
/// 解题思路
///
/// 固定滑动窗口
pub fn max_vowels(s: String, k: i32) -> i32 {
    let k = k as usize;
    // 把字符串转换成字符数组
    let s: Vec<char> = s.chars().collect();
    let mut r = k;
    // 定义一个闭包，用来判断字符是否是元音
    let is_vowel = |x| match x {
        'a' | 'e' | 'i' | 'o' | 'u' => 1,
        _ => 0,
    };
    // 计算第一个窗口内元音的个数 0 - k
    let mut cur_vowels = (&s[..k]).iter().map(|&x| is_vowel(x)).sum::<i32>();
    // 定义最大元音数
    let mut max_vowels = cur_vowels;
    while r < s.len() {
        // 如果是元音则当前元音数加1
        cur_vowels += is_vowel(s[r]);
        // 如果不是元音则当前元音数减1
        cur_vowels -= is_vowel(s[r - k]);
        // 更新最大元音数
        max_vowels = max_vowels.max(cur_vowels);
        // 窗口右移
        r += 1;
    }
    max_vowels
}

/// 强化练习 3：字符串的排列
///
/// 给你两个字符串 s1 和 s2 ，写一个函数来判断 s2 是否包含 s1 的排列。如果是，返回 true ；否则，返回 false 。
///
/// 换句话说，s1 的排列之一是 s2 的 子串 。
///
/// 示例 1：
///
/// 输入：s1 = "ab" s2 = "eidbaooo"
///
/// 输出：true
///
/// 解释：s2 包含 s1 的排列之一 ("ba").
///
/// 示例 2：
///
/// 输入：s1= "ab" s2 = "eidboaoo"
///
/// 输出：false
///
/// 提示：
///
/// 1 <= s1.length, s2.length <= 104
///
/// s1 和 s2 仅包含小写字母
pub fn check_inclusion(s1: String, s2: String) -> bool {
    let (s, p) = (s2, s1);
    let mut rst = vec![];
    if s.len() < p.len() {
        // 如果s的长度小于p的长度，直接返回false
        return false;
    }
    let mut count_p = [0; 128];
    let mut count_s = [0; 128];
    let s = s.as_bytes();
    let p = p.as_bytes();
    for i in 0..p.len() {
        // 分别统计 s1 和 s2 中的字符出现的次数
        count_p[p[i] as usize] += 1;
        count_s[s[i] as usize] += 1;
    }
    if count_p == count_s {
        // 如果s1和s2的前p.len()个字符的出现次数相同，则把0放入结果集中
        rst.push(0);
    }
    for r in p.len()..s.len() {
        // 滑动窗口左边滑出的元素索引
        let l = r - p.len();
        // 对新滑入窗口的字符进行统计
        count_s[s[r] as usize] += 1;
        // 对滑出窗口的字符进行统计
        count_s[s[l] as usize] -= 1;
        if count_p == count_s {
            // 如果s1和s2的前p.len()个字符的出现次数相同，则把l+1放入结果集中
            rst.push(l as i32 + 1);
        }
    }
    !rst.is_empty()
}

/// 强化练习 5：半径为 k 的子数组平均值
///
/// 给你一个下标从 0 开始的数组 nums ，数组中有 n 个整数，另给你一个整数 k 。
///
/// 半径为 k 的子数组平均值 是指：nums 中一个以下标 i 为 中心 且 半径 为 k 的子数组中所有元素的平均值，即下标在 i - k 和 i + k 范围（含 i - k 和 i + k）内所有元素的平均值。如果在下标 i 前或后不足 k 个元素，那么 半径为 k 的子数组平均值 是 -1 。
///
/// 构建并返回一个长度为 n 的数组 avgs ，其中 avgs[i] 是以下标 i 为中心的子数组的 半径为 k 的子数组平均值 。
///
/// x 个元素的 平均值 是 x 个元素相加之和除以 x ，此时使用截断式 整数除法 ，即需要去掉结果的小数部分。
///
/// 例如，四个元素 2、3、1 和 5 的平均值是 (2 + 3 + 1 + 5) / 4 = 11 / 4 = 2.75，截断后得到 2 。
///
/// 示例 1：
///
/// 输入：nums = [7,4,3,9,1,8,5,2,6], k = 3
///
/// 输出：[-1,-1,-1,5,4,4,-1,-1,-1]
///
/// 解释：
///
/// - avg[0]、avg[1] 和 avg[2] 是 -1 ，因为在这几个下标前的元素数量都不足 k 个。
///
/// - 中心为下标 3 且半径为 3 的子数组的元素总和是：7 + 4 + 3 + 9 + 1 + 8 + 5 = 37 。
///
/// - 使用截断式 整数除法，avg[3] = 37 / 7 = 5 。
///
/// - 中心为下标 4 的子数组，avg[4] = (4 + 3 + 9 + 1 + 8 + 5 + 2) / 7 = 4 。
///
/// - 中心为下标 5 的子数组，avg[5] = (3 + 9 + 1 + 8 + 5 + 2 + 6) / 7 = 4 。
///
/// - avg[6]、avg[7] 和 avg[8] 是 -1 ，因为在这几个下标后的元素数量都不足 k 个。
///
/// 示例 2：
///
/// 输入：nums = [100000], k = 0
///
/// 输出：[100000]
///
/// 解释：
///
/// - 中心为下标 0 且半径 0 的子数组的元素总和是：100000 。
///
/// - avg[0] = 100000 / 1 = 100000 。
///
/// 示例 3：
///
/// 输入：nums = [8], k = 100000
///
/// 输出：[-1]
///
/// 解释：
///
/// - avg[0] 是 -1 ，因为在下标 0 前后的元素数量均不足 k 。
///
/// 提示：
///
/// n == nums.length
///
/// 1 <= n <= 105
///
/// 0 <= nums[i], k <= 105
pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // 定义结果集
    let mut res = Vec::<i32>::with_capacity(nums.len());
    // 滑动窗口的区间范围为 k - nums.len()-k
    if k == 0 {
        // 如果k=0，直接返回nums
        return nums;
    }

    if nums.len() < (2 * k + 1) as usize {
        for _ in 0..nums.len() {
            res.push(-1);
        }
        return res;
    }
    // 0 - k 都是-1
    for _ in 0..k as usize {
        res.push(-1);
    }

    // 统计第一个窗口的和
    let mut sum = nums[..(2 * k + 1) as usize].iter().map(|x| *x as i64).sum::<i64>();
    // 计算第一个窗口的平均值
    res.push((sum / (2 * k + 1) as i64) as i32);
    // 滑动窗口
    for i in (2 * k + 1)..nums.len() as i32 {
        // 滑动窗口左边滑出的元素索引
        let l = i - (2 * k + 1);
        // 对新滑入窗口的字符进行统计
        sum += nums[i as usize] as i64;
        // 对滑出窗口的字符进行统计
        sum -= nums[l as usize] as i64;
        // 计算滑动窗口的平均值
        res.push((sum / (2 * k + 1) as i64) as i32);
    }
    // nums.len()-k 到 nums.len() 都是-1
    for _ in 0..k as usize {
        res.push(-1);
    }
    res
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
    fn test_min_sub_array_len() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        let length = min_sub_array_len(target, nums);
        println!("{:?}", length)
    }

    #[test]
    fn test_find_max_consecutive_ones() {
        let nums = [1, 0, 1, 1, 0, 1].to_vec();
        let find_max_consecutive_ones = find_max_consecutive_ones(nums);
        println!("{:?}", find_max_consecutive_ones)
    }

    #[test]
    fn test_find_anagrams() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let find_anagrams = find_anagrams(s, p);
        println!("{:?}", find_anagrams)
    }

    #[test]
    fn test_max_vowels() {
        let s = "abciiidef".to_string();
        let k = 3;
        let max_vowels = max_vowels(s, k);
        println!("{:?}", max_vowels)
    }

    #[test]
    fn test_check_inclusion() {
        let s1 = "hello".to_string();
        let s2 = "ooolleoooleh".to_string();
        let check_inclusion = check_inclusion(s1, s2);
        println!("{:?}", check_inclusion)
    }

    #[test]
    fn test_get_averages() {
        let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
        let k = 3;
        let get_averages = get_averages(nums, k);
        println!("{:?}", get_averages)
    }
}
