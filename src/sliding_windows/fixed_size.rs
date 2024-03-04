use std::collections::HashMap;

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

/// 1456. 定长子串中元音的最大数目
pub fn max_vowels_ii(s: String, k: i32) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    // 定义元音字符数组
    let v = ['a', 'e', 'i', 'o', 'u'];
    // 定义k区间最大元音数
    let mut max = 0;
    // 计算第一个区间元音数
    for i in 0..k {
        if v.contains(&s[i as usize]) {
            max += 1;
        }
    }
    // 定义当前长度
    let mut cur = max;
    // 滑动定长窗口进行后续统计
    for i in k as usize..s.len() {
        if v.contains(&s[i - k as usize]) {
            // 左边窗口滑出的元素刚好为元音字符
            cur -= 1;
        }
        if v.contains(&s[i]) {
            // 右边滑入的为元音字符
            cur += 1;
        }
        // 滑动一次就对最大值进行统计
        max = max.max(cur);
    }
    max
}

/// 2269. 找到一个数字的 K 美丽值
pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    // 把数字转换为字符数组
    let nums = num.to_string().chars().collect::<Vec<char>>();
    // 统计美丽值的数量
    let mut cnt = 0;
    // 定义一个队列来存储区间的数字
    let mut queue = Vec::new();
    // 处理第一个窗口
    for i in 0..k {
        // 先把第一个窗口的数字放入队列
        queue.push(nums[i as usize]);
    }
    if is_divisible(queue.clone(), num) {
        cnt += 1;
    }
    // 开始滑动窗口，处理后续的区间
    for i in k as usize..nums.len() {
        // 窗口右边的字符入队
        queue.push(nums[i]);
        // 窗口左边的字符出队
        queue.remove(0);
        // 判断是否是美丽值
        if is_divisible(queue.clone(), num) {
            cnt += 1;
        }
    }
    cnt
}

fn is_divisible(mut queue: Vec<char>, num: i32) -> bool {
    // 如果队首是0就把0弹出来
    while let Some(v) = queue.first() {
        if *v == '0' {
            queue.remove(0);
        } else {
            break;
        }
    }
    // 如果队列中全为零的时候会被全部弹出，这个时候返回false
    if queue.is_empty() {
        return false;
    }
    // 把队列中的字符转为数字
    let parse = queue.iter().collect::<String>().parse::<i32>().unwrap();
    // 是否整除
    num % parse == 0
}

/// 1984. 学生分数的最小差值
pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut ans = i32::MAX;
    for i in 0..nums.len() - k as usize + 1 {
        // i + k - 1是滑动窗口的右边界，i 是滑动窗口的左边界
        ans = ans.min(nums[i + k as usize - 1] - nums[i]);
    }
    ans
}

/// 1343. 大小为 K 且平均值大于等于阈值的子数组数目
pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    // 定义结果
    let mut ans = 0;
    // 定义和，并处理第一个窗口
    let mut sum = arr[..k as usize].iter().sum::<i32>();
    // 计算第一个窗口的平均值
    if sum / k >= threshold {
        ans += 1;
    }
    // 滑动窗口处理后续值
    for i in k as usize..arr.len() {
        sum += arr[i] - arr[i - k as usize];
        if sum / k >= threshold {
            ans += 1;
        }
    }
    ans
}

/// 2379. 得到 K 个黑块的最少涂色次数
pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let b = blocks.chars().collect::<Vec<char>>();
    // 连续 k 个黑色块的 最少 操作次数,处理第一个窗口，统计白块的个数
    let mut min = b[..k as usize].iter().filter(|c| **c == 'W').count();
    // 定义滑动窗口中当前白块数量
    let mut cur = min;
    // 滑动窗口
    for i in k as usize..b.len() {
        if b[i - k as usize] == 'W' {
            cur -= 1;
        }
        if b[i] == 'W' {
            cur += 1;
        }
        min = min.min(cur);
    }
    min as i32
}

/// 2841. 几乎唯一子数组的最大和
pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
    // 定义一个hash表来存储窗口中的元素，哈希表的 K表示数组中的数，V表示出现的次数
    let mut hash = HashMap::<i64, i64>::new();
    // 定义结果
    let mut ans: i64 = 0;
    // 定义窗口元素的和
    let mut cur_sum: i64 = 0;
    // 处理第一个窗口
    for i in 0..k as usize {
        // 把元素加入到哈希表中，相同的k v加一
        hash.insert(
            nums[i] as i64,
            hash.get(&(nums[i] as i64)).or(Some(&0)).unwrap() + 1,
        );
        // 累加和
        cur_sum += nums[i] as i64;
    }
    if hash.keys().len() >= m as usize {
        // 如果哈希表中的key数量大于等于m
        ans = ans.max(cur_sum);
    }
    // 滑动窗口处理后续元素
    for i in k as usize..nums.len() {
        // 窗口右边滑入的元素，在哈希表中的计数应该增加一
        hash.insert(
            nums[i] as i64,
            hash.get(&(nums[i] as i64)).or(Some(&0)).unwrap() + 1,
        );
        // 滑出窗口的元素
        if hash.get(&(nums[i - k as usize] as i64)).unwrap() - 1 == 0 {
            // 如果哈希表中的元素只有一个当从左边被划出窗口后就应该从哈希表中移除
            hash.remove(&(nums[i - k as usize] as i64));
        } else {
            // 如果哈希表中的元素大于1个当从左边被划出窗口后就应该从哈希表中的计数就应该减一
            hash.insert(
                nums[i - k as usize] as i64,
                hash.get(&(nums[i - k as usize] as i64)).unwrap() - 1,
            );
        }
        // 滑出窗口的数应该从和中减掉
        cur_sum -= nums[i - k as usize] as i64;
        // 滑入窗口的数应该加入和中
        cur_sum += nums[i] as i64;
        if hash.keys().len() >= m as usize {
            // 如果哈希表中的key数量大于等于m
            ans = ans.max(cur_sum);
        }
    }
    ans
}

/// 2461. 长度为 K 子数组中的最大和
pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    // 定义一个hash表来存储窗口中的元素，哈希表的 K表示数组中的数，V表示出现的次数
    let mut hash = HashMap::<i64, i64>::new();
    // 定义结果
    let mut ans: i64 = 0;
    // 定义窗口元素的和
    let mut cur_sum: i64 = 0;
    // 处理第一个窗口
    for i in 0..k as usize {
        // 把元素加入到哈希表中，相同的k v加一
        hash.insert(
            nums[i] as i64,
            hash.get(&(nums[i] as i64)).or(Some(&0)).unwrap() + 1,
        );
        // 累加和
        cur_sum += nums[i] as i64;
    }
    if hash.keys().len() == k as usize {
        // 如果哈希表中的key数量等于窗口大小
        ans = ans.max(cur_sum);
    }
    // 滑动窗口处理后续元素
    for i in k as usize..nums.len() {
        // 窗口右边滑入的元素，在哈希表中的计数应该增加一
        hash.insert(
            nums[i] as i64,
            hash.get(&(nums[i] as i64)).or(Some(&0)).unwrap() + 1,
        );
        // 滑出窗口的元素
        if hash.get(&(nums[i - k as usize] as i64)).unwrap() - 1 == 0 {
            // 如果哈希表中的元素只有一个当从左边被划出窗口后就应该从哈希表中移除
            hash.remove(&(nums[i - k as usize] as i64));
        } else {
            // 如果哈希表中的元素大于1个当从左边被划出窗口后就应该从哈希表中的计数就应该减一
            hash.insert(
                nums[i - k as usize] as i64,
                hash.get(&(nums[i - k as usize] as i64)).unwrap() - 1,
            );
        }
        // 滑出窗口的数应该从和中减掉
        cur_sum -= nums[i - k as usize] as i64;
        // 滑入窗口的数应该加入和中
        cur_sum += nums[i] as i64;
        if hash.keys().len() == k as usize {
            // 如果哈希表中的key数量大于等于m
            ans = ans.max(cur_sum);
        }
    }
    ans
}

/// 2134. 最少交换次数来组合所有的 1 II
///
/// 根据题意： 最终所有的 1 都会聚集在一起 ， 那么我们以 1 的个数作为滑动窗口的大小
///
/// 具体操作如下：
///
/// 因为是任意两个位置交换，那么最优解一定是跟 若干个 1 中间的 0 交换
///
/// 结论：如果在定长的窗口内0的个数最小 则一定最优
///
/// 举个栗子：
///
/// 0101100 0101100 窗口长度：3 最优解： 1
///
/// 11001 11001 窗口长度：3 最优解： 0
pub fn min_swaps(nums: Vec<i32>) -> i32 {
    // 统计数组nums中1的个数，用1的个数作为滑动窗口的大小
    let cnt = nums.iter().filter(|n| **n == 1).count();
    // 定义结果
    let mut ans = i32::MAX;
    // 处理第一个窗口,统计出第一个窗口0的个数
    let mut cur = nums[..cnt].iter().filter(|n| **n == 0).count();
    ans = ans.min(cur as i32);
    // 把数组中的数据复制一份
    let nums = nums.repeat(2);
    // 滑动窗口处理后续元素
    for i in cnt..nums.len() {
        if nums[i - cnt] == 0 {
            cur -= 1;
        }
        if nums[i] == 0 {
            cur += 1;
        }
        ans = ans.min(cur as i32);
    }
    ans
}

/// 2653. 滑动子数组的美丽值
pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    // 定义结果
    let mut ans = Vec::new();
    // 定义数组存放窗口中的元素
    let mut windows = Vec::new();
    // 定义哈希表，idx为nums[i] 值为数字出现的次数
    let mut hash = vec![0; 101];
    // 定义基数，由于nums[i] 可能为 最小-50 的数，所以计数定义为50 保证放入哈希表中的位置不为负数
    let bias = 50;
    // 处理第一个窗口
    for i in 0..k as usize {
        windows.push(nums[i]);
        hash[(nums[i] + bias) as usize] += 1;
    }
    ans.push(get_x_last(&hash, x));
    // 滑动窗口处理后续元素
    for i in k as usize..nums.len() {
        windows.push(nums[i]);
        windows.remove(0);
        hash[(nums[i - k as usize] + bias) as usize] -= 1;
        hash[(nums[i] + bias) as usize] += 1;
        ans.push(get_x_last(&hash, x))
    }
    ans
}

fn get_x_last(hash: &Vec<i32>, x: i32) -> i32 {
    // 倒数第x个数的位置，当x递减到0时就表示遍历到了倒数第x个数了
    let mut x = x;
    for (idx, v) in hash.iter().enumerate() {
        if *v > 0 {
            // 减v是因为当一个数出现多次时也需要减掉，所以可能会减为负数
            x -= *v;
        }
        // 倒数第x个数可能刚好出现在重复数字中，此时x就为负数
        if x <= 0 {
            // 此处需要减去 50 的基数
            if idx as i32 - 50 < 0 {
                return idx as i32 - 50;
            }
        }
    }
    0
}

/// 1176. 健身计划评估
///
/// https://leetcode.cn/problems/diet-plan-performance/
///
/// 思路
///
/// 固定k长度的滑动窗口
pub fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
    // 计算第一个窗口消耗的卡路里
    let mut sum = calories[..k as usize].iter().sum();
    // 定义得分
    let mut ans = calc_score(sum, lower, upper);
    // 滑动窗口处理后续数据
    for i in k as usize..calories.len() {
        // 滑出窗口的卡路里值需要减掉,滑入窗口的中的卡路里值需要加上
        sum += calories[i] - calories[i - k as usize];
        ans += calc_score(sum, lower, upper);
    }
    ans
}

fn calc_score(sum: i32, lower: i32, upper: i32) -> i32 {
    if sum < lower {
        -1
    } else if sum > upper {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_max_vowels_ii() {
        let s = "abciiidef".to_string();
        let k = 3;
        let max_vowels = max_vowels_ii(s, k);
        println!("{:?}", max_vowels)
    }

    #[test]
    fn test_divisor_substrings() {
        let num = 430043;
        let k = 2;
        let divisor_substrings = divisor_substrings(num, k);
        println!("{:?}", divisor_substrings)
    }

    #[test]
    fn test_minimum_difference() {
        let nums = vec![9, 4, 1, 7];
        let k = 2;
        let minimum_difference = minimum_difference(nums, k);
        println!("{}", minimum_difference)
    }

    #[test]
    fn test_minimum_recolors() {
        let blocks = "WBBWWBBWBW".to_string();
        let k = 7;
        let minimum_recolors = minimum_recolors(blocks, k);
        assert_eq!(minimum_recolors, 3);
    }

    #[test]
    fn test_max_sum() {
        let nums = vec![2, 6, 7, 3, 1, 7];
        let m = 3;
        let k = 4;
        let max_sum = max_sum(nums, m, k);
        assert_eq!(max_sum, 18);
    }

    #[test]
    fn test_maximum_subarray_sum() {
        let nums = vec![1, 5, 4, 2, 9, 9, 9];
        let k = 3;
        let maximum_subarray_sum = maximum_subarray_sum(nums, k);
        assert_eq!(maximum_subarray_sum, 15);
    }

    #[test]
    fn test_get_subarray_beauty() {
        let nums = vec![-14, 9, 13, -26, 47, -39, -49, -14, 29];
        let k = 9;
        let x = 4;
        let get_subarray_beauty = get_subarray_beauty(nums, k, x);
        println!("{:?}", get_subarray_beauty);
    }
}