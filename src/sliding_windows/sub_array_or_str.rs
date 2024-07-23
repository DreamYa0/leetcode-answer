/**
 * 467. 环绕字符串中唯一的子字符串
中等
相关标签
相关企业
提示
定义字符串 base 为一个 "abcdefghijklmnopqrstuvwxyz" 无限环绕的字符串，所以 base 看起来是这样的：

"...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".
给你一个字符串 s ，请你统计并返回 s 中有多少 不同非空子串 也在 base 中出现。



示例 1：

输入：s = "a"
输出：1
解释：字符串 s 的子字符串 "a" 在 base 中出现。
示例 2：

输入：s = "cac"
输出：2
解释：字符串 s 有两个子字符串 ("a", "c") 在 base 中出现。
示例 3：

输入：s = "zab"
输出：6
解释：字符串 s 有六个子字符串 ("z", "a", "b", "za", "ab", and "zab") 在 base 中出现。


提示：

1 <= s.length <= 105
s 由小写英文字母组成
 */
pub fn find_substring_in_wrapround_string(s: String) -> i32 {
    let (mut dp, p_arr, mut cnt) = (vec![0; 32], s.as_bytes(), 0);
    for i in 0..s.len() {
        // p_arr[i] - p_arr[i - 1] == 1 说明字符串是连续的
        // p_arr[i - 1] - p_arr[i] == 25 说明从末尾指向开头仍然属于连续的
        if i > 0 && (p_arr[i] - p_arr[i - 1] == 1 || p_arr[i - 1] - p_arr[i] == 25) {
            cnt += 1;
        } else {
            cnt = 1
        }
        dp[(p_arr[i] - b'a') as usize] = cnt.max(dp[(p_arr[i] - b'a') as usize])
    }
    dp.iter().sum::<i32>()
}

/**
 * 795. 区间子数组个数
中等
相关标签
相关企业
给你一个整数数组 nums 和两个整数：left 及 right 。找出 nums 中连续、非空且其中最大元素在范围 [left, right] 内的子数组，并返回满足条件的子数组的个数。

生成的测试用例保证结果符合 32-bit 整数范围。



示例 1：

输入：nums = [2,1,4,3], left = 2, right = 3
输出：3
解释：满足条件的三个子数组：[2], [2, 1], [3]
示例 2：

输入：nums = [2,9,2,5,6], left = 2, right = 8
输出：7


提示：

1 <= nums.length <= 105
0 <= nums[i] <= 109
0 <= left <= right <= 109
 */
pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    not_greater(&nums, right) - not_greater(&nums, left - 1)
}

fn not_greater(nums: &Vec<i32>, k: i32) -> i32 {
    let mut cnt = 0;
    let mut res = 0;
    for i in 0..nums.len() {
        if nums[i] <= k {
            cnt += 1;
        } else {
            cnt = 0;
        }
        res += cnt;
    }
    res
}

/**
 * 904. 水果成篮
中等
相关标签
相关企业
你正在探访一家农场，农场从左到右种植了一排果树。这些树用一个整数数组 fruits 表示，其中 fruits[i] 是第 i 棵树上的水果 种类 。

你想要尽可能多地收集水果。然而，农场的主人设定了一些严格的规矩，你必须按照要求采摘水果：

你只有 两个 篮子，并且每个篮子只能装 单一类型 的水果。每个篮子能够装的水果总量没有限制。
你可以选择任意一棵树开始采摘，你必须从 每棵 树（包括开始采摘的树）上 恰好摘一个水果 。采摘的水果应当符合篮子中的水果类型。每采摘一次，你将会向右移动到下一棵树，并继续采摘。
一旦你走到某棵树前，但水果不符合篮子的水果类型，那么就必须停止采摘。
给你一个整数数组 fruits ，返回你可以收集的水果的 最大 数目。



示例 1：

输入：fruits = [1,2,1]
输出：3
解释：可以采摘全部 3 棵树。
示例 2：

输入：fruits = [0,1,2,2]
输出：3
解释：可以采摘 [1,2,2] 这三棵树。
如果从第一棵树开始采摘，则只能采摘 [0,1] 这两棵树。
示例 3：

输入：fruits = [1,2,3,2,2]
输出：4
解释：可以采摘 [2,3,2,2] 这四棵树。
如果从第一棵树开始采摘，则只能采摘 [1,2] 这两棵树。
示例 4：

输入：fruits = [3,3,3,1,2,1,1,2,3,3,4]
输出：5
解释：可以采摘 [1,2,1,1,2] 这五棵树。


提示：

1 <= fruits.length <= 105
0 <= fruits[i] < fruits.length
 */
pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut slow = 0;
    let mut res = 0;
    let mut k = 2;
    // 统计窗口中的数据个数，如果数据范围很大可以换成HashMap
    let mut cnt = vec![0; fruits.len() + 1];
    for fast in 0..fruits.len() {
        if cnt[fruits[fast] as usize] == 0 {
            // 当窗口中没有arr[fast]时，k-1,k表示窗口中不同元素的个数
            k -= 1;
        }
        // 窗口中arr[fast]的个数加1
        cnt[fruits[fast] as usize] += 1;
        while k < 0 {
            // 当窗口中不同元素的个数大于k时，slow右移，直到窗口中不同元素的个数小于等于k
            cnt[fruits[slow] as usize] -= 1;
            if cnt[fruits[slow] as usize] == 0 {
                // 当窗口中arr[slow]的个数为0时，k+1
                k += 1;
            }
            // slow右移
            slow += 1;
        }
        // 当窗口中不同元素的个数小于等于k时，计算子数组的个数
        res = res.max((fast - slow + 1) as i32);
    }
    res
}

/**
 * 992. K 个不同整数的子数组
困难
相关标签
相关企业
给定一个正整数数组 nums和一个整数 k，返回 nums 中 「好子数组」 的数目。

如果 nums 的某个子数组中不同整数的个数恰好为 k，则称 nums 的这个连续、不一定不同的子数组为 「好子数组 」。

例如，[1,2,3,1,2] 中有 3 个不同的整数：1，2，以及 3。
子数组 是数组的 连续 部分。



示例 1：

输入：nums = [1,2,1,2,3], k = 2
输出：7
解释：恰好由 2 个不同整数组成的子数组：[1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2].
示例 2：

输入：nums = [1,2,1,3,4], k = 3
输出：3
解释：恰好由 3 个不同整数组成的子数组：[1,2,1,3], [2,1,3], [1,3,4].


提示：

1 <= nums.length <= 2 * 104
1 <= nums[i], k <= nums.length
 */
pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    at_most_k(&nums, k) - at_most_k(&nums, k - 1)
}

fn at_most_k(arr: &Vec<i32>, mut k: i32) -> i32 {
    let mut slow = 0;
    let mut res = 0;
    // 统计窗口中的数据个数，如果数据范围很大可以换成HashMap
    let mut cnt = vec![0; arr.len() + 1];
    for fast in 0..arr.len() {
        if cnt[arr[fast] as usize] == 0 {
            // 当窗口中没有arr[fast]时，k-1,k表示窗口中不同元素的个数
            k -= 1;
        }
        // 窗口中arr[fast]的个数加1
        cnt[arr[fast] as usize] += 1;
        while k < 0 {
            // 当窗口中不同元素的个数大于k时，slow右移，直到窗口中不同元素的个数小于等于k
            cnt[arr[slow] as usize] -= 1;
            if cnt[arr[slow] as usize] == 0 {
                // 当窗口中arr[slow]的个数为0时，k+1
                k += 1;
            }
            // slow右移
            slow += 1;
        }
        // 当窗口中不同元素的个数小于等于k时，计算子数组的个数
        res += (fast - slow + 1) as i32;
    }
    res
}

/**
 * 718. 最长重复子数组
中等
相关标签
相关企业
提示
给两个整数数组 nums1 和 nums2 ，返回 两个数组中 公共的 、长度最长的子数组的长度 。



示例 1：

输入：nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
输出：3
解释：长度最长的公共子数组是 [3,2,1] 。
示例 2：

输入：nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
输出：5


提示：

1 <= nums1.length, nums2.length <= 1000
0 <= nums1[i], nums2[i] <= 100
 */
pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    if nums1.len() > nums2.len() {
        find_length(nums2, nums1)
    } else {
        find_max(nums1, nums2)
    }
}

fn find_max(nums1: Vec<i32>, nums2: Vec<i32>)-> i32  {
    let m = nums1.len();
    let n = nums2.len();
    let mut max = 0;

    /*
        A:           |*|*|*|*|
        B: |*|*|*|*|*|*|
                 ↓
        A:       |*|*|*|*|
        B: |*|*|*|*|*|*|
    */
    for i in 1..m {
        max = max.max(find(&nums1, &nums2, 0, n - i, i));
    }

    /*
        A:     |*|*|*|*|
        B: |*|*|*|*|*|*|
                 ↓
        A: |*|*|*|*|
        B: |*|*|*|*|*|*|
    */
    for i in (0..=(n - m)).rev() {
        max = max.max(find(&nums1, &nums2, 0, i, m));
    }

    /*
        A: |*|*|*|*|
        B:   |*|*|*|*|*|*|
                 ↓
        A: |*|*|*|*|
        B:       |*|*|*|*|*|*|
    */
    for i in (1..=(m - 1)).rev() {
        max = max.max(find(&nums1, &nums2, m - i, 0, i));
    }
    max
}

fn find(nums1: &Vec<i32>, nums2: &Vec<i32>, i: usize, j: usize, len: usize) -> i32 {
    let mut max = 0;
    let mut count = 0;
    for k in 0..len {
        if nums1[i + k] == nums2[j + k] {
            count += 1;
        } else {
            max = max.max(count);
            count = 0;
        }
    }
    max.max(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_subarray_bounded_max() {
        let nums = vec![2, 1, 4, 3];
        let left = 2;
        let right = 3;
        assert_eq!(num_subarray_bounded_max(nums, left, right), 3);
    }

    #[test]
    fn test_find_length() {
        let nums1 = vec![1,2,3,2,1];
        let nums2 = vec![3,2,1,4];
        assert_eq!(find_length(nums1, nums2), 3);
    }
}
