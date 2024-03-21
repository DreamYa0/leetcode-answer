use std::collections::{HashMap, HashSet};

/// 2799. 统计完全子数组的数目
/// https://leetcode.cn/problems/count-complete-subarrays-in-an-array/
pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    // 先统计nums数组中不同元素的个数
    let mut set = HashSet::with_capacity(nums.len());
    for n in nums.iter() {
        set.insert(n);
    }
    // nums数组中不同元素的个数
    let len = set.len();
    // 定义结果
    let mut ans = 0;
    // 定义慢指针
    let mut slow = 0;
    // 定义哈希表来统计滑动窗口中不同元素的个数
    let mut hash = HashMap::new();
    for fast in 0..nums.len() {
        // nums[fast] 元素放入哈希表中
        hash.insert(nums[fast], hash.get(&nums[fast]).or(Some(&0)).unwrap() + 1);
        while hash.keys().len() == len {
            // nums[slow] 滑出窗口
            if *hash.get(&nums[slow]).unwrap() == 1 {
                hash.remove(&nums[slow]);
            } else {
                hash.insert(nums[slow], hash.get(&nums[slow]).unwrap() - 1);
            }
            // 右移slow指针
            slow += 1;
        }
        ans += slow;
    }
    ans as i32
}

/// 713. 乘积小于 K 的子数组
/// https://leetcode.cn/problems/subarray-product-less-than-k/
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    //同样排除k为1的情况比如  [1,1,1] k=1
    if k <= 1 {
        return 0;
    }
    // 定义结果
    let mut ans = 0;
    // 定义慢指针
    let mut slow = 0;
    // 定义窗口内元素的积
    let mut mult = 1;
    for fast in 0..nums.len() {
        mult *= nums[fast];
        while mult >= k {
            mult /= nums[slow];
            slow += 1;
        }
        // 每次fast指针位移到一个新位置，应该加上 x 种数组组合：
        // nums[fast]
        // nums[fast-1], nums[fast]
        // nums[fast-2], nums[fast-1], nums[fast]
        // nums[slow], ......, nums[fast-2], nums[fast-1], nums[fast]
        //共有 fast - slow + 1 种
        ans += fast - slow + 1;
    }
    ans as i32
}

/// 1358. 包含所有三种字符的子字符串数目
///
/// https://leetcode.cn/problems/number-of-substrings-containing-all-three-characters/
pub fn number_of_substrings(s: String) -> i32 {
    let s = s.into_bytes();
    // 定义结果
    let mut ans = 0;
    // 定义慢指针
    let mut slow = 0;
    // 定义哈希表来统计字符串出现的次数,存放 a - z 26个字母的acii码
    let mut cnt = vec![0; 3];
    for fast in 0..s.len() {
        cnt[(s[fast] - b'a') as usize] += 1;
        while cnt[(b'a' - b'a') as usize] >= 1
            && cnt[(b'b' - b'a') as usize] >= 1
            && cnt[(b'c' - b'a') as usize] >= 1
        {
            // 当满足条件时
            ans += s.len() - fast;
            // 从cnt同扣减掉 s[slow] 的统计
            cnt[(s[slow] - b'a') as usize] -= 1;
            slow += 1;
        }
    }
    ans as i32
}

/**
 * 1248. 统计「优美子数组」
中等
相关标签
相关企业
提示
给你一个整数数组 nums 和一个整数 k。如果某个连续子数组中恰好有 k 个奇数数字，我们就认为这个子数组是「优美子数组」。

请返回这个数组中 「优美子数组」 的数目。



示例 1：

输入：nums = [1,1,2,1,1], k = 3
输出：2
解释：包含 3 个奇数的子数组是 [1,1,2,1] 和 [1,2,1,1] 。
示例 2：

输入：nums = [2,4,6], k = 1
输出：0
解释：数列中不包含任何奇数，所以不存在优美子数组。
示例 3：

输入：nums = [2,2,2,1,2,2,1,2,2,2], k = 2
输出：16


提示：

1 <= nums.length <= 50000
1 <= nums[i] <= 10^5
1 <= k <= nums.length
 */
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut slow = 0;
    let mut fast = 0;
    // 多少个奇数
    let mut odd_cnt = 0;
    while fast < nums.len() {
        if nums[fast] % 2 == 1 {
            // 奇数个数加1
            odd_cnt += 1;
        }
        fast += 1;
        //  若当前滑动窗口 [left, right) 中有 k 个奇数了，进入此分支统计当前滑动窗口中的优美子数组个数。
        if odd_cnt == k {
            // 临时保持一下fast指针
            let tmp = fast;
            // 先将滑动窗口的右边界向右拓展，直到遇到下一个奇数（或出界）
            // rightEvenCnt 即为第 k 个奇数右边的偶数的个数
            while fast < nums.len() && nums[fast] % 2 == 0 {
                // 如果nums[fast] 为偶数，就继续向右推进fast指针
                fast += 1;
            }
            let right_cnt = fast - tmp;
            let mut left_cnt = 0;
            // 如果nums[slow]偶数就继续向右推荐slow指针
            while nums[slow] % 2 == 0 {
                slow += 1;
                left_cnt += 1;
            }
            res += (left_cnt + 1) * (right_cnt + 1);
            slow += 1;
            odd_cnt -= 1;
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_complete_subarrays() {
        let nums = vec![1, 3, 1, 2, 2];
        let count_complete_subarrays = count_complete_subarrays(nums);
        assert_eq!(count_complete_subarrays, 4);
    }

    #[test]
    fn test_number_of_substrings() {
        let s = "abcabc".to_string();
        let number_of_substrings = number_of_substrings(s);
        assert_eq!(number_of_substrings, 10);
    }

    #[test]
    fn test_number_of_subarrays() {
        let nums = vec![1, 1, 2, 1, 1];
        let k = 3;
        let number_of_subarrays = number_of_subarrays(nums, k);
        assert_eq!(number_of_subarrays, 2);
    }
}
