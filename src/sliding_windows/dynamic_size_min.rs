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

/// 1234. 替换子串得到平衡字符串
/// https://leetcode.cn/problems/replace-the-substring-for-balanced-string/
pub fn balanced_string(s: String) -> i32 {
    let bytes = s.into_bytes();
    // 定义哈希表来统计字符串中字符出现的次数
    let mut cnt = vec![0; 128];
    // 遍历字节数组
    for b in bytes.iter() {
        cnt[*b as usize] += 1;
    }
    let m = bytes.len() / 4;
    if cnt[b'Q' as usize] == m
        && cnt[b'W' as usize] == m
        && cnt[b'E' as usize] == m
        && cnt[b'R' as usize] == m
    {
        // 已经符合要求
        return 0;
    }
    let mut ans = bytes.len();
    // 定义慢指针
    let mut slow = 0;
    for fast in 0..bytes.len() {
        // cnt 为整个字符串字符出现次数的统计，fast右移动说明窗口在扩大，进入窗口的字符就应该从cnt中减掉
        cnt[bytes[fast] as usize] -= 1;
        while cnt[b'Q' as usize] <= m
            && cnt[b'W' as usize] <= m
            && cnt[b'E' as usize] <= m
            && cnt[b'R' as usize] <= m
        {
            ans = ans.min(fast - slow + 1);
            // cnt 为整个字符串字符出现次数的统计，slow右移动说明窗口在缩小，出窗口的字符就应该加回cnt中
            cnt[bytes[slow] as usize] += 1;
            slow += 1;
        }
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        let length = min_sub_array_len(target, nums);
        println!("{:?}", length)
    }

    #[test]
    fn test_min_operations() {
        let nums = vec![5, 6, 7, 8, 9];
        let x = 4;
        let min_operations = min_operations(nums, x);
        println!("{:?}", min_operations)
    }

    #[test]
    fn test_balanced_string() {
        let s = "QQWE".to_string();
        let balanced_string = balanced_string(s);
        assert_eq!(balanced_string, 1);
    }
}