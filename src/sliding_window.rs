use std::{
    collections::{HashMap, VecDeque},
    vec,
};

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
    // 用来标记窗口内是否存在重复元素，也可以用 HashSet，这里为了效率用的 Vec，数组长度定义128是因为u8的范围是0-127
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
            // 调整窗口，滑出窗口的需要减掉
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
    // 循环不变量
    // 在使用滑动窗口遍历字符串的过程中，我们的循环不变量是：
    // 在每一次迭代或滑动窗口移动中，我们维护一个计数，表示当前窗口（长度恒为 k）中元音字母的数量。
    // 这个计数随着窗口的移动而更新，确保了我们总是有当前窗口内的元音字母数量。
    //
    // 算法步骤与循环不变量的应用
    // 初始化：首先，计算出字符串 s 的前 k 个字符中元音字母的总数作为初始窗口的元音字母数。
    // 这保证了在第一次迭代之前，我们有一个大小为 k 的窗口，其元音字母数量已经被正确统计。
    //
    // 保持：随着窗口向右滑动（即，每次向右移动窗口一位），我们从元音字母计数中减去移出窗口的字符（如果它是元音字母），
    // 并加上新进入窗口的字符（如果它是元音字母）。这一步保证了对于每个新的窗口位置，
    // 我们都重新计算并可能更新窗口内的元音字母计数，以保持计数为当前窗口位置下可能的最大元音字母数量。
    //
    // 终止：当我们的窗口滑动到字符串的末尾时，我们已经考察了所有可能的长度为 k 的子字符串。
    // 此时，我们找到了含有最多元音字母的子字符串，从而也就能知道字符串 s 中长度为 k 的单个子字符串中可能包含的最大元音字母数。
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
    let mut sum = nums[..(2 * k + 1) as usize]
        .iter()
        .map(|x| *x as i64)
        .sum::<i64>();
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

/// 强化练习 6：滑动窗口最大值
///
/// 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
///
/// 返回 滑动窗口中的最大值 。
///
/// 示例 1：
///
/// 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
///
/// 输出：[3,3,5,5,6,7]
///
/// 解释：
///
/// ```
/// 滑动窗口的位置                最大值
/// ---------------               -----
/// [1  3  -1] -3  5  3  6  7       3
///  1 [3  -1  -3] 5  3  6  7       3
///  1  3 [-1  -3  5] 3  6  7       5
///  1  3  -1 [-3  5  3] 6  7       5
///  1  3  -1  -3 [5  3  6] 7       6
///  1  3  -1  -3  5 [3  6  7]      7
/// ```
/// 示例 2：
///
/// 输入：nums = [1], k = 1
///
/// 输出：[1]
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// -104 <= nums[i] <= 104
///
/// 1 <= k <= nums.length
///
/// 单调队列套路
///
/// 入（元素进入队尾，同时维护队列单调性）
///
/// 出（元素离开队首）
///
/// 记录/维护答案（根据队首）
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut ans = Vec::new();
    // 双端队列
    let mut q = VecDeque::new();
    for (i, &x) in nums.iter().enumerate() {
        // 1. 入
        while !q.is_empty() && nums[*q.back().unwrap()] <= x {
            // 维护 q 的单调性
            q.pop_back();
        }
        // 入队
        q.push_back(i);
        // 2. 出
        if i - q[0] >= k {
            // 队首已经离开窗口了
            q.pop_front();
        }
        // 3. 记录答案
        if i >= k - 1 {
            // 由于队首到队尾单调递减，所以窗口最大值就是队首
            ans.push(nums[q[0]]);
        }
    }
    ans
}

/// 1438. 绝对差不超过限制的最长连续子数组
///
/// 给你一个整数数组 nums ，和一个表示限制的整数 limit，请你返回最长连续子数组的长度，该子数组中的任意两个元素之间的绝对差必须小于或者等于 limit 。
///
/// 如果不存在满足条件的子数组，则返回 0 。
///
/// 示例 1：
///
/// 输入：nums = [8,2,4,7], limit = 4
///
/// 输出：2
///
/// 解释：所有子数组如下：
///
/// ```
/// [8] 最大绝对差 |8-8| = 0 <= 4.
/// [8,2] 最大绝对差 |8-2| = 6 > 4.
/// [8,2,4] 最大绝对差 |8-2| = 6 > 4.
/// [8,2,4,7] 最大绝对差 |8-2| = 6 > 4.
/// [2] 最大绝对差 |2-2| = 0 <= 4.
/// [2,4] 最大绝对差 |2-4| = 2 <= 4.
/// [2,4,7] 最大绝对差 |2-7| = 5 > 4.
/// [4] 最大绝对差 |4-4| = 0 <= 4.
/// [4,7] 最大绝对差 |4-7| = 3 <= 4.
/// [7] 最大绝对差 |7-7| = 0 <= 4.
/// ```
///
/// 因此，满足题意的最长子数组的长度为 2 。
///
/// 示例 2：
///
/// 输入：nums = [10,1,2,4,7,2], limit = 5
///
/// 输出：4
///
/// 解释：满足题意的最长子数组是 [2,4,7,2]，其最大绝对差 |2-7| = 5 <= 5 。
///
/// 示例 3：
///
/// 输入：nums = [4,2,2,2,4,4,2,2], limit = 0
///
/// 输出：3
///
/// 提示：
///
/// 1 <= nums.length <= 10^5
///
/// 1 <= nums[i] <= 10^9
///
/// 0 <= limit <= 10^9
pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut max_q = VecDeque::new();
    let mut min_q = VecDeque::new();
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;
    while right < nums.len() {
        while !max_q.is_empty() && nums[right] > nums[*max_q.back().unwrap()] {
            max_q.pop_back();
        }
        while !min_q.is_empty() && nums[right] < nums[*min_q.back().unwrap()] {
            min_q.pop_back();
        }
        max_q.push_back(right);
        min_q.push_back(right);
        while nums[*max_q.front().unwrap()] - nums[*min_q.front().unwrap()] > limit {
            if *max_q.front().unwrap() < *min_q.front().unwrap() {
                left = max_q.pop_front().unwrap() + 1;
            } else {
                left = min_q.pop_front().unwrap() + 1;
            }
        }
        res = res.max(right - left + 1);
        right += 1;
    }
    res as i32
}

/// 强化练习 4：最小覆盖子串
///
/// 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。
///
/// 注意：
///
/// 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
///
/// 如果 s 中存在这样的子串，我们保证它是唯一的答案。
///
/// 示例 1：
///
/// 输入：s = "ADOBECODEBANC", t = "ABC"
///
/// 输出："BANC"
///
/// 解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
///
/// 示例 2：
///
/// 输入：s = "a", t = "a"
///
/// 输出："a"
///
/// 解释：整个字符串 s 是最小覆盖子串。
///
/// 示例 3:
///
/// 输入: s = "a", t = "aa"
///
/// 输出: ""
///
/// 解释: t 中两个字符 'a' 均应包含在 s 的子串中，
///
/// 因此没有符合条件的子字符串，返回空字符串。
///
/// 提示：
///
/// m == s.length
///
/// n == t.length
///
/// 1 <= m, n <= 105
///
/// s 和 t 由英文字母组成
///
/// 进阶：你能设计一个在 o(m+n) 时间内解决此问题的算法吗？
#[allow(unused)]
pub fn min_window(s: String, t: String) -> String {
    "".to_string()
}

/// 718. 最长重复子数组
///
/// 给两个整数数组 nums1 和 nums2 ，返回 两个数组中 公共的 、长度最长的子数组的长度 。
///
/// 示例 1：
///
/// 输入：nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
///
/// 输出：3
///
/// 解释：长度最长的公共子数组是 [3,2,1] 。
///
/// 示例 2：
///
/// 输入：nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
/// 输出：5
///
/// 提示：
///
/// ```
/// 1 <= nums1.length, nums2.length <= 1000
/// 0 <= nums1[i], nums2[i] <= 100
/// ```
#[allow(unused)]
pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    i32::MIN
}

/// 子数组最大平均数 I
///
/// 给你一个由 n 个元素组成的整数数组 nums 和一个整数 k 。
///
/// 请你找出平均数最大且 长度为 k 的连续子数组，并输出该最大平均数。
///
/// 任何误差小于 10-5 的答案都将被视为正确答案。
///
/// 示例 1：
///
/// 输入：nums = [1,12,-5,-6,50,3], k = 4
///
/// 输出：12.75
///
/// 解释：最大平均数 (12-5-6+50)/4 = 51/4 = 12.75
///
/// 示例 2：
///
/// 输入：nums = [5], k = 1
///
/// 输出：5.00000
///
/// 提示：
///
/// ```
/// n == nums.length
/// 1 <= k <= n <= 105
/// -104 <= nums[i] <= 104
/// ```
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    // 循环不变量定义
    // 在每次迭代中，窗口总和是窗口内当前 k 个元素的总和。
    // 在每次迭代后，我们都会有当前窗口的最大平均值。
    // 初始化：在循环开始之前，计算数组中前 k 个元素的总和。此时，窗口总和等于这些元素的总和，因此循环不变量在开始时成立。
    // 保持：在每次迭代中，我们从总和中减去窗口最左边的元素，并加上窗口最右边的新元素。
    // 这确保了窗口总和始终是当前窗口内 k 个元素的总和，从而维护了循环不变量。
    // 终止：当窗口到达数组的末尾时，我们已经考虑了所有可能的 k 个连续元素的子数组。循环不变量确保我们已经找到了具有最大平均值的子数组。
    // 计算前 k 个元素的总和
    let mut sum = 0;
    for i in 0..k as usize {
        // 第一个窗口的和
        sum += nums[i];
    }
    // 初始化最大平均值
    let mut max_avg = sum as f64 / k as f64;
    for i in k..nums.len() as i32 {
        // 滑动窗口
        sum += nums[i as usize] - nums[(i - k) as usize];
        // 更新最大平均值
        max_avg = max_avg.max(sum as f64 / k as f64);
    }
    max_avg
}

/// 爱生气的书店老板
///
/// 有一个书店老板，他的书店开了 n 分钟。每分钟都有一些顾客进入这家商店。
/// 给定一个长度为 n 的整数数组 customers ，其中 customers[i] 是在第 i 分钟开始时进入商店的顾客数量，
/// 所有这些顾客在第 i 分钟结束后离开。
///
/// 在某些时候，书店老板会生气。 如果书店老板在第 i 分钟生气，那么 grumpy[i] = 1，否则 grumpy[i] = 0。
///
/// 当书店老板生气时，那一分钟的顾客就会不满意，若老板不生气则顾客是满意的。
///
/// 书店老板知道一个秘密技巧，能抑制自己的情绪，可以让自己连续 minutes 分钟不生气，但却只能使用一次。
///
/// 请你返回 这一天营业下来，最多有多少客户能够感到满意 。
///
/// 示例 1：
///
/// 输入：customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], minutes = 3
///
/// 输出：16
///
/// 解释：书店老板在最后 3 分钟保持冷静。
///
/// 感到满意的最大客户数量 = 1 + 1 + 1 + 1 + 7 + 5 = 16.
///
/// 示例 2：
///
/// 输入：customers = [1], grumpy = [0], minutes = 1
///
/// 输出：1
///
/// 提示：
///
/// ```
/// n == customers.length == grumpy.length
/// 1 <= minutes <= n <= 2 * 104
/// 0 <= customers[i] <= 1000
/// grumpy[i] == 0 or 1
/// ```
pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    // 算法步骤
    // 1.首先，计算不使用秘密技巧时的满意顾客数。这包括所有老板不生气时的顾客数。
    // 2.然后，遍历每个可能的 X 分钟时间段，计算如果在这段时间使用秘密技巧，可以额外使多少原本不满意的顾客变得满意。这是通过计算在生气的那些分钟内的顾客数来实现的。
    // 3.对于每个窗口，计算总的满意顾客数（基本满意顾客数加上额外满意的顾客数），并记录下最大值。
    // 4.当遍历完所有可能的窗口后，返回记录的最大满意顾客数。
    // 循环不变量
    // 在这个算法中，我们可以定义如下循环不变量来帮助我们理解和验证算法的正确性：
    // 在每次迭代中，我们维护一个固定长度为 X 的窗口，该窗口代表了如果在这段时间内使用秘密技巧，可以额外满足的顾客数量。
    // 窗口移动过程中，我们记录下能使最多顾客满意的窗口位置。
    // 初始化
    // 在开始遍历之前，我们已经计算了不使用秘密技巧时的基本满意顾客数。此时，窗口还未开始移动，因此循环不变量成立。
    // 保持
    // 每次窗口移动，我们从额外满意顾客数中减去窗口最左边的值（如果老板在那一分钟生气的话），并加上窗口最右边的新值（如果老板在那一分钟生气的话）。这确保了在每次迭代中，我们都正确地计算了在当前窗口使用秘密技巧能够额外满足的顾客数。
    // 终止
    // 当窗口遍历完所有可能的位置后，我们已经考虑了所有可能的使用秘密技巧的时间段。循环不变量确保我们找到了能使最多顾客满意的那个时间段。
    let len = grumpy.len();
    // 定义前缀和数组
    let mut pre_sum = vec![0; len + 1];
    // 统计 1. 所有本来就不生气的顾客数量；2. 前缀和数组
    let mut origin_count = 0;
    for i in 0..len {
        if grumpy[i] == 0 {
            origin_count += customers[i];
            pre_sum[i + 1] = pre_sum[i];
        } else {
            pre_sum[i + 1] = pre_sum[i] + customers[i];
        }
    }
    // 计算不使用秘密技巧时的满意顾客数
    let mut max_angry_count = 0;
    for i in 0..len {
        if (i + minutes as usize) as usize <= len {
            max_angry_count = max_angry_count.max(pre_sum[i + minutes as usize] - pre_sum[i]);
        }
    }
    origin_count + max_angry_count
}

/// 可获得的最大点数
///
/// 几张卡牌 排成一行，每张卡牌都有一个对应的点数。点数由整数数组 cardPoints 给出。
///
/// 每次行动，你可以从行的开头或者末尾拿一张卡牌，最终你必须正好拿 k 张卡牌。
///
/// 你的点数就是你拿到手中的所有卡牌的点数之和。
///
/// 给你一个整数数组 cardPoints 和整数 k，请你返回可以获得的最大点数。
///
/// 示例 1：
///
/// 输入：cardPoints = [1,2,3,4,5,6,1], k = 3
///
/// 输出：12
///
/// 解释：第一次行动，不管拿哪张牌，你的点数总是 1 。但是，先拿最右边的卡牌将会最大化你的可获得点数。最优策略是拿右边的三张牌，最终点数为 1 + 6 + 5 = 12 。
///
/// 示例 2：
///
/// 输入：cardPoints = [2,2,2], k = 2
///
/// 输出：4
///
/// 解释：无论你拿起哪两张卡牌，可获得的点数总是 4 。
///
/// 示例 3：
///
/// 输入：cardPoints = [9,7,7,9,7,7,9], k = 7
///
/// 输出：55
///
/// 解释：你必须拿起所有卡牌，可以获得的点数为所有卡牌的点数之和。
///
/// 示例 4：
///
/// 输入：cardPoints = [1,1000,1], k = 1
///
/// 输出：1
///
/// 解释：你无法拿到中间那张卡牌，所以可以获得的最大点数为 1 。
///
/// 示例 5：
///
/// 输入：cardPoints = [1,79,80,1,1,1,200,1], k = 3
///
/// 输出：202
///
/// 提示：
///
/// 1 <= cardPoints.length <= 10^5
///
/// 1 <= cardPoints[i] <= 10^4
///
/// 1 <= k <= cardPoints.length
///
/// 解题思路
///
/// 这个问题可以通过一个滑动窗口的方法解决，其中循环不变量是关键概念，帮助我们确保算法的正确性。
/// 我们首先将问题定义转换为：在数组 cardPoints 中找到一个长度为 len(cardPoints) - k 的最小子数组和，
/// 其余部分即为我们可以获得的最大点数。这里的思路是，由于我们只能从头部或尾部拿卡牌，
/// 因此留下的部分（我们不拿的卡牌）形成的连续子数组一定位于数组的中间某处。
/// 我们的目标是最小化这个中间部分的和，从而最大化我们所拿卡牌的总点数。
pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    // 循环不变量定义
    // 在使用滑动窗口遍历数组以找到长度为 len(cardPoints) - k 的最小子数组和的过程中，我们的循环不变量是：
    // 在每一步滑动窗口迭代中，我们维护的窗口都包含了一段长度为 len(cardPoints) - k 的连续子数组。
    // 我们保证这个窗口内的元素总和是当前窗口位置下可能的最小和。
    // 窗口的总和在每次迭代后更新，要么保持不变（如果新加入窗口的元素和移出窗口的元素总和相等），要么变小（如果新加入的元素小于移出的元素）。
    //
    // 算法步骤与循环不变量
    // 初始化：计算出数组 cardPoints 的前 len(cardPoints) - k 个元素的总和作为初始窗口的总和。
    // 这保证了在第一次迭代之前，我们有一个大小正确的窗口，其总和是基于当前窗口位置的最小可能和。
    //
    // 保持：随着窗口的滑动（即，每次向右移动一个位置），我们从总和中减去移出窗口的元素，并加上新进入窗口的元素。
    // 这一步保证了，对于每个新的窗口位置，我们都重新计算并可能更新窗口的总和，以保持窗口总和为当前位置下可能的最小和。
    //
    // 终止：当我们的窗口滑动到数组的末尾时，我们已经考察了所有可能的长度为 len(cardPoints) - k 的连续子数组。
    // 此时，我们找到了最小的子数组和，从而也就能通过总点数减去这个最小和来计算出可以获得的最大点数。
    let len = card_points.len();
    // 定义当前窗口的的总和
    let mut cur_sum = 0;
    // 计算前 len(cardPoints) - k 个元素的总和,第一个窗口的和
    card_points
        .iter()
        .take(len - k as usize)
        .for_each(|&x| cur_sum += x);
    // 初始化k长度最小和
    let mut min_sum = cur_sum;
    // 计算k到len-1的总和
    for i in len - k as usize..len {
        // 左边滑出窗口的元素，需要减掉
        cur_sum -= card_points[i - (len - k as usize)];
        // 右边滑入窗口的元素，需要加上
        cur_sum += card_points[i];
        // 更新最大和
        min_sum = min_sum.min(cur_sum);
    }
    // 计算总和
    let total_sum = card_points.iter().sum::<i32>();
    total_sum - min_sum
}

/// 将 x 减到 0 的最小操作数
///
/// 给你一个整数数组 nums 和一个整数 x 。
/// 每一次操作时，你应当移除数组 nums 最左边或最右边的元素，然后从 x 中减去该元素的值。
/// 请注意，需要 修改 数组以供接下来的操作使用。
///
/// 如果可以将 x 恰好 减到 0 ，返回 最小操作数 ；否则，返回 -1 。
///
/// 示例 1：
///
/// 输入：nums = [1,1,4,2,3], x = 5
///
/// 输出：2
///
/// 解释：最佳解决方案是移除后两个元素，将 x 减到 0 。
///
/// 示例 2：
///
/// 输入：nums = [5,6,7,8,9], x = 4
///
/// 输出：-1
///
/// 示例 3：
///
/// 输入：nums = [3,2,20,1,1,3], x = 10
///
/// 输出：5
///
/// 解释：最佳解决方案是移除后三个元素和前两个元素（总共 5 次操作），将 x 减到 0 。
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// 1 <= nums[i] <= 104
///
/// 1 <= x <= 109
///
/// 解题思路
///
/// 这个问题可以转换为寻找数组中的最长连续子数组，其元素之和等于 sum(nums) - x。
/// 如果我们找到了这样一个子数组，那么数组的其余部分（位于子数组两侧的元素）就是我们需要通过操作移除的部分，
/// 以达到将 x 减到 0 的目标。因此，最小操作数等于 nums 的长度减去这个最长子数组的长度。
pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    // 循环不变量定义
    // 在使用双指针或滑动窗口遍历数组来寻找这个最长子数组的过程中，我们可以定义以下循环不变量来帮助我们理解和保证算法的正确性：
    // 1.在每次迭代或窗口扩展/收缩中，我们维护一个当前子数组的和 currentSum，这个和表示从当前左指针到右指针的元素之和。
    // 2.我们保证在每次迭代结束时，currentSum 要么等于我们寻找的目标和 target = sum(nums) - x，
    // 要么小于 target（如果存在这样的子数组），要么我们调整指针以继续寻找。
    //
    // 算法步骤与循环不变量的应用
    //
    // 初始化：计算整个数组的和 totalSum，设定目标和 target = totalSum - x。
    // 如果 target 小于 0，则直接返回 -1，因为我们不能通过移除元素来得到一个负数。
    // 初始化左右指针 left = 0 和 right = 0，以及当前和 currentSum = 0。
    //
    // 保持：移动右指针扩展窗口，每次增加 right 指针指向的元素到 currentSum，
    // 直到 currentSum 大于等于 target 或 right 达到数组末尾。
    // 如果 currentSum 等于 target，更新最长子数组的长度。然后，移动左指针以收缩窗口，
    // 每次减去 left 指针指向的元素从 currentSum，直到 currentSum 小于 target，重复此过程。
    //
    // 终止：当右指针达到数组末尾，并且左指针遍历完可能的所有位置时，算法结束。
    // 我们已经考察了所有可能的连续子数组，找到了最长的符合条件的子数组（如果存在）。
    //
    // 结果计算：如果找到了符合条件的子数组，返回 nums 的长度减去最长子数组的长度作为最小操作数；否则返回 -1。

    // 计算总和
    let total_sum: i32 = nums.iter().sum();
    // 定义慢指针
    let mut slow = 0;
    // 当前窗口的和
    let mut cur_sum = 0;
    let mut ans = nums.len() + 1;
    let len = nums.len();
    for fast in 0..len {
        cur_sum += nums[fast];
        while slow <= fast && total_sum - cur_sum < x {
            cur_sum -= nums[slow];
            slow += 1;
        }
        if total_sum - cur_sum == x {
            ans = ans.min(len - (fast - slow + 1));
        }
    }
    if ans == nums.len() + 1 {
        -1
    } else {
        ans as i32
    }
}

/// 替换后的最长重复字符
/// 给你一个字符串 s 和一个整数 k 。你可以选择字符串中的任一字符，并将其更改为任何其他大写英文字符。该操作最多可执行 k 次。
///
/// 在执行上述操作后，返回 包含相同字母的最长子字符串的长度。
///
/// 示例 1：
///
/// 输入：s = "ABAB", k = 2
///
/// 输出：4
///
/// 解释：用两个'A'替换为两个'B',反之亦然。
///
/// 示例 2：
///
/// 输入：s = "AABABBA", k = 1
///
/// 输出：4
///
/// 解释：
///
/// 将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
///
/// 子串 "BBBB" 有最长重复字母, 答案为 4。
///
/// 可能存在其他的方法来得到同样的结果。
///
/// 提示：
///
/// 1 <= s.length <= 105
/// s 仅由大写英文字母组成
/// 0 <= k <= s.length
///
/// 解题思路
///
/// 这个问题可以通过滑动窗口的方法来解决，目的是找到最长的子字符串，
/// 其中包含的相同字母数量加上最多 k 次字符更改操作，能够使得整个子字符串全部由相同的字母构成。
#[allow(unused)]
pub fn character_replacement(s: String, k: i32) -> i32 {
    // 循环不变量定义
    //
    // 在使用滑动窗口遍历字符串的过程中，我们定义以下循环不变量来帮助我们理解和保证算法的正确性：
    // 窗口内最频繁出现的字符的出现次数加上 k 应该大于等于窗口的大小。
    // 这意味着，我们可以通过最多 k 次操作，将窗口内的所有其他字符更改为窗口内出现次数最多的字符。
    //
    // 算法步骤与循环不变量的应用
    //
    // 1.初始化：设置两个指针 left = 0 和 right = 0 来表示滑动窗口的左右边界。
    // 使用一个数据结构（如哈希表）来记录窗口内每个字符的出现次数，以及一个变量来维护窗口内出现次数最多的字符的数量 maxCount。
    //
    // 2.保持：向右移动右指针 right 来扩大窗口，并更新窗口内字符的出现次数以及 maxCount。
    // 每次窗口更新后，检查是否可以通过最多 k 次操作将窗口内的所有字符更改为相同字符。
    // 如果可以，更新最长子字符串的长度。如果不可以，则向右移动左指针 left 来缩小窗口，直到窗口满足条件为止。
    //
    // 终止：当右指针遍历完字符串 s 时，算法结束。此时，我们已经考察了所有可能的子字符串，并找到了满足条件的最长子字符串。
    let mut left = 0;
    let mut right = 0;
    // 记录窗口内最大字符的数量
    let mut max_count = 0;
    // 定义字符频次哈希表
    let mut char_freq = vec![0; 26];
    let s = s.chars().collect::<Vec<char>>();
    while right < s.len() {
        // 计数
        char_freq[(s[right] as u8 - 'A' as u8) as usize] += 1;
        // 更新最大字符数量
        max_count = max_count.max(char_freq[(s[right] as u8 - 'A' as u8) as usize]);
        // 如果窗口内的字符数量加上 k 小于窗口的大小，说明窗口内的字符数量加上 k 无法满足窗口的大小
        if right - left + 1 - max_count > k as usize {
            // 减掉即将移出的字符计数
            char_freq[(s[left] as u8 - 'A' as u8) as usize] -= 1;
            // 移动左指针
            left += 1;
        }
        // 移动右指针
        right += 1;
    }
    (right - left) as i32
}

/// 删除子数组的最大得分
///
/// 给你一个正整数数组 nums ，请你从中删除一个含有 若干不同元素 的子数组。删除子数组的 得分 就是子数组各元素之 和 。
///
/// 返回 只删除一个 子数组可获得的 最大得分 。
///
/// 如果数组 b 是数组 a 的一个连续子序列，即如果它等于 a[l],a[l+1],...,a[r] ，那么它就是 a 的一个子数组。
///
/// 示例 1：
///
/// 输入：nums = [4,2,4,5,6]
///
/// 输出：17
///
/// 解释：最优子数组是 [2,4,5,6]
///
/// 示例 2：
///
/// 输入：nums = [5,2,1,2,5,2,1,2,5]
///
/// 输出：8
///
/// 解释：最优子数组是 [5,2,1] 或 [1,2,5]
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// 1 <= nums[i] <= 104
///
/// 解题思路
///
/// 这个问题可以通过动态规划（DP）或双指针加上一些辅助数据结构（如哈希集合）来解决，
/// 旨在找到一个含有若干不同元素的子数组，其和最大，然后从原数组中删除这个子数组。
/// 但考虑到问题的特性和求解过程的需要，我们可以采用一种“前缀和”和“后缀和”结合的方法来简化问题，并利用双指针技术高效寻找解。
/// 这个方法的核心在于，我们先计算每个位置为结束点的子数组最大得分（后缀和）和每个位置为起始点的子数组最大得分（前缀和），
/// 然后通过比较这些得分来找到最优解。
pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    // 定义左指针
    let mut left = 0;
    // 统计数组内元素的频次
    let mut counter = HashMap::new();
    // 当前和
    let mut cur_sum = 0;
    // 定义最大得分
    let mut max_sum = 0;

    for right in 0..nums.len() {
        // 统计元素出现的频次
        *counter.entry(nums[right]).or_insert(0) += 1;
        // 计算和
        cur_sum += nums[right];
        // 当大于1说明窗口内出现重复元素，就需要移动left指针来缩小窗口
        while *counter.get(&nums[right]).unwrap() > 1 {
            // 移动left指针之前需要把即将移出窗口的元素出现的频次减掉
            *counter.get_mut(&nums[left]).unwrap() -= 1;
            // 移动left指针之前需要把即将移出窗口的元素和减掉
            cur_sum -= nums[left];
            // 右移动左指针
            left += 1;
        }
        // 更新最大得分
        max_sum = max_sum.max(cur_sum);
    }
    max_sum
}

/// 最大连续1的个数 II
///
/// 给定一个二进制数组 nums ，如果最多可以翻转一个 0 ，则返回数组中连续 1 的最大个数。
///
/// 示例 1：
///
/// 输入：nums = [1,0,1,1,0]
///
/// 输出：4
///
/// 解释：翻转第一个 0 可以得到最长的连续 1。
///
/// 当翻转以后，最大连续 1 的个数为 4。
///
/// 示例 2:
///
/// 输入：nums = [1,0,1,1,0,1]
///
/// 输出：4
///
///
/// 提示:
///
/// 1 <= nums.length <= 105
///
/// nums[i] 不是 0 就是 1.
///
/// 进阶：如果输入的数字是作为 无限流 逐个输入如何处理？换句话说，内存不能存储下所有从流中输入的数字。您可以有效地解决吗？
pub fn find_max_consecutive_ones_ii(nums: Vec<i32>) -> i32 {
    // 循环不变量定义
    // 在滑动窗口的遍历过程中，我们定义以下循环不变量来帮助我们理解和保证算法的正确性：
    // 窗口内最多包含一个 0。这个循环不变量保证了在任何时刻，我们的窗口（考虑的子数组）要么只包含全 1，
    // 要么包含一个 0 和一些 1，这正好符合题目的要求。
    //
    // 算法步骤与循环不变量的应用
    //
    // 初始化：设置两个指针 left = 0 和 right = 0 来表示滑动窗口的左右边界。同时，维护一个计数器 zeroCount 来记录当前窗口内 0 的数量
    //
    // 保持：向右移动右指针 right 扩大窗口，并在每次移动时更新 zeroCount。如果 zeroCount 超过 1，
    // 则向右移动左指针 left 来缩小窗口，直到 zeroCount 再次等于 1 或小于 1。在这个过程中，我们保持循环不变量不变，即窗口内最多只有一个 0。
    //
    // 终止：当右指针遍历完整个数组时，算法结束。此时，我们已经考察了所有可能的窗口，找到了满足条件的最长窗口。
    //
    // 结果计算：通过维护窗口的最大长度来记录并最终返回数组中连续 1 的最大个数（包括最多翻转一个 0 的情况）。
    // 定义滑动窗口左边界指针
    let mut left = 0;
    // 可反转次数
    let mut k = 1;
    // 定义最大长度
    let mut max_len = 0;
    for right in 0..nums.len() {
        // 如果是0，就减少可反转次数
        if nums[right] == 0 {
            k -= 1;
        }
        while k < 0 {
            if nums[left] == 0 {
                // 如果窗口中的0滑出了窗口，这可反转计数需要加1
                k += 1;
            }
            left += 1;
        }
        max_len = max_len.max(right - left + 1);
    }
    max_len as i32
}

/// 最大连续 1 的个数 III
///
/// 给定一个二进制数组 nums 和一个整数 k，如果可以翻转最多 k 个 0 ，则返回 数组中连续 1 的最大个数 。
///
/// 示例 1：
///
/// 输入：nums = [1,1,1,0,0,0,1,1,1,1,0], K = 2
///
/// 输出：6
///
/// 解释：[1,1,1,0,0,1,1,1,1,1,1]
///
/// 粗体数字从 0 翻转到 1，最长的子数组长度为 6。
///
/// 示例 2：
///
/// 输入：nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], K = 3
///
/// 输出：10
///
/// 解释：[0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
///
/// 粗体数字从 0 翻转到 1，最长的子数组长度为 10。
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// nums[i] 不是 0 就是 1
///
/// 0 <= k <= nums.length
pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    // 可反转次数
    let mut k = k;
    // 定义最大长度
    let mut max_len = 0;
    for right in 0..nums.len() {
        // 如果是0，就减少可反转次数
        if nums[right] == 0 {
            k -= 1;
        }
        while k < 0 {
            if nums[left] == 0 {
                // 如果窗口中的0滑出了窗口，这可反转计数需要加1
                k += 1;
            }
            left += 1;
        }
        max_len = max_len.max(right - left + 1);
    }
    max_len as i32
}

/// 删掉一个元素以后全为 1 的最长子数组
/// 
/// 给你一个二进制数组 nums ，你需要从中删掉一个元素。
/// 
/// 请你在删掉元素的结果数组中，返回最长的且只包含 1 的非空子数组的长度。
/// 
/// 如果不存在这样的子数组，请返回 0 。
/// 
/// 提示 1：
/// 
/// 输入：nums = [1,1,0,1]
/// 
/// 输出：3
/// 
/// 解释：删掉位置 2 的数后，[1,1,1] 包含 3 个 1 。
/// 
/// 示例 2：
/// 
/// 输入：nums = [0,1,1,1,0,1,1,0,1]
/// 
/// 输出：5
/// 
/// 解释：删掉位置 4 的数字后，[0,1,1,1,1,1,0,1] 的最长全 1 子数组为 [1,1,1,1,1] 。
/// 
/// 示例 3：
/// 
/// 输入：nums = [1,1,1]
/// 
/// 输出：2
/// 
/// 解释：你必须要删除一个元素。
///  
/// 提示：
/// 
/// 1 <= nums.length <= 105
/// 
/// nums[i] 要么是 0 要么是 1 。
pub fn longest_subarray_ii(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    // 窗口中能包含0的最大个数
    let mut k = 1;
    // 定义最大长度
    let mut max_len = 0;
    for right in 0..nums.len() {
        // 如果是0，就减少可反转次数
        if nums[right] == 0 {
            k -= 1;
        }
        while k < 0 {
            if nums[left] == 0 {
                // 如果窗口中的0滑出了窗口，这可反转计数需要加1
                k += 1;
            }
            left += 1;
        }
        // 因为0是需要被删除的的，所以长度为right - left 而不是 right - left + 1
        max_len = max_len.max(right - left);
    }
    max_len as i32
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

    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let max_sliding_window = max_sliding_window(nums, k);
        println!("{:?}", max_sliding_window)
    }

    #[test]
    fn test_longest_subarray() {
        let nums = vec![8, 2, 4, 7];
        let limit = 4;
        let longest_subarray = longest_subarray(nums, limit);
        println!("{:?}", longest_subarray)
    }

    #[test]
    fn test_find_max_average() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let find_max_average = find_max_average(nums, k);
        println!("{:?}", find_max_average)
    }

    #[test]
    fn test_max_satisfied() {
        let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
        let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
        let minutes = 3;
        let max_satisfied = max_satisfied(customers, grumpy, minutes);
        println!("{:?}", max_satisfied)
    }

    #[test]
    fn test_max_score() {
        let card_points = vec![2, 2, 2];
        let k = 2;
        let max_score = max_score(card_points, k);
        println!("{:?}", max_score)
    }

    #[test]
    fn test_min_operations() {
        let nums = vec![5, 6, 7, 8, 9];
        let x = 4;
        let min_operations = min_operations(nums, x);
        println!("{:?}", min_operations)
    }

    #[test]
    fn test_character_replacement() {
        let s = "AABABBA".to_string();
        let k = 1;
        let character_replacement = character_replacement(s, k);
        println!("{:?}", character_replacement)
    }

    #[test]
    fn test_find_max_consecutive_ones_ii() {
        let nums = vec![1, 0, 1, 1, 0];
        let find_max_consecutive_ones = find_max_consecutive_ones_ii(nums);
        println!("{:?}", find_max_consecutive_ones)
    }

    #[test]
    fn test_longest_ones() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let k = 2;
        let longest_ones = longest_ones(nums, k);
        println!("{:?}", longest_ones)
    }

    #[test]
    fn test_longest_subarray_ii() {
        let nums = vec![1, 1, 0, 1];
        let longest_subarray_ii = longest_subarray_ii(nums);
        println!("{:?}", longest_subarray_ii)
    }
}
