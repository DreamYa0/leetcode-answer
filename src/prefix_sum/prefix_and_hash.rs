use std::{
    collections::{HashMap, HashSet},
    vec,
};

/**
 * 523. 连续的子数组和
给你一个整数数组 nums 和一个整数 k ，编写一个函数来判断该数组是否含有同时满足下述条件的连续子数组：

子数组大小 至少为 2 ，且
子数组元素总和为 k 的倍数。
如果存在，返回 true ；否则，返回 false 。

如果存在一个整数 n ，令整数 x 符合 x = n * k ，则称 x 是 k 的一个倍数。0 始终视为 k 的一个倍数。



示例 1：

输入：nums = [23,2,4,6,7], k = 6
输出：true
解释：[2,4] 是一个大小为 2 的子数组，并且和为 6 。
示例 2：

输入：nums = [23,2,6,4,7], k = 6
输出：true
解释：[23, 2, 6, 4, 7] 是大小为 5 的子数组，并且和为 42 。
42 是 6 的倍数，因为 42 = 7 * 6 且 7 是一个整数。
示例 3：

输入：nums = [23,2,6,4,7], k = 13
输出：false


提示：

1 <= nums.length <= 105
0 <= nums[i] <= 109
0 <= sum(nums[i]) <= 231 - 1
1 <= k <= 231 - 1
 */
pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }
    // 用hash_set保存过去出现过的值
    let mut hash_set = HashSet::new();
    // 子数组大小 至少为 2,所有从2开始遍历
    for i in 2..=nums.len() {
        hash_set.insert(prefix[i - 2] % k);
        if hash_set.contains(&(prefix[i] % k)) {
            return true;
        }
    }
    false
}

/**
 * 560. 和为 K 的子数组
中等
相关标签
相关企业
提示
给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。

子数组是数组中元素的连续非空序列。



示例 1：

输入：nums = [1,1,1], k = 2
输出：2
示例 2：

输入：nums = [1,2,3], k = 3
输出：2


提示：

1 <= nums.length <= 2 * 104
-1000 <= nums[i] <= 1000
-107 <= k <= 107
 */
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        // 计算前缀和
        prefix[i + 1] = prefix[i] + nums[i];
    }
    let mut res = 0;
    // 通过哈希表来统计 prefix[j] - prefix[i] = k 的个数
    let mut hash_map = HashMap::new();
    for i in 0..prefix.len() {
        if let Some(&v) = hash_map.get(&(prefix[i] - k)) {
            res += v;
        }
        *hash_map.entry(prefix[i]).or_insert(0) += 1;
    }
    res
}

/**
 * 525. 连续数组
中等
相关标签
相关企业
给定一个二进制数组 nums , 找到含有相同数量的 0 和 1 的最长连续子数组，并返回该子数组的长度。



示例 1:

输入: nums = [0,1]
输出: 2
说明: [0, 1] 是具有相同数量 0 和 1 的最长连续子数组。
示例 2:

输入: nums = [0,1,0]
输出: 2
说明: [0, 1] (或 [1, 0]) 是具有相同数量0和1的最长连续子数组。


提示：

1 <= nums.length <= 105
nums[i] 不是 0 就是 1
 */
pub fn find_max_length(nums: Vec<i32>) -> i32 {
    // 由于「0 和 1 的数量相同」等价于「1 的数量减去 0 的数量等于 0」，
    // 我们可以将数组中的 0 视作 −1，则原问题转换成「求最长的连续子数组，其元素和为 0」。
    let n = nums.len();
    let mut ans = 0;
    // 规定空的前缀的结束下标为 −1，由于空的前缀的元素和为 0，因此在遍历之前，首先在哈希表中存入键值对 (0,−1)。
    let mut mp = std::collections::HashMap::new();
    mp.insert(0, -1);
    let mut pre_sum = 0;
    for i in 0..n {
        if nums[i] == 1 {
            pre_sum += 1;
        } else {
            pre_sum -= 1;
        }
        if let Some(j) = mp.get(&pre_sum) {
            ans = ans.max(i as i32 - j);
        } else {
            mp.insert(pre_sum, i as i32);
        }
    }
    ans
}

/**
 * 974. 和可被 K 整除的子数组
中等
相关标签
相关企业
给定一个整数数组 nums 和一个整数 k ，返回其中元素之和可被 k 整除的（连续、非空） 子数组 的数目。

子数组 是数组的 连续 部分。



示例 1：

输入：nums = [4,5,0,-2,-3,1], k = 5
输出：7
解释：
有 7 个子数组满足其元素之和可被 k = 5 整除：
[4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
示例 2:

输入: nums = [5], k = 9
输出: 0


提示:

1 <= nums.length <= 3 * 104
-104 <= nums[i] <= 104
2 <= k <= 104
 */
pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }
    let mut res = 0;
    let mut hash = HashMap::new();
    // 根据 mod 运算的性质，我们知道 (sum[j]−sum[i]) % k = sum[j] % k − sum[i] % k
    // 故若想 (sum[j] − sum[i]) % k = 0 ，则必有 sum[j] % k = sum[i] % k
    for i in 0..prefix.len() {
        // 由于数组中有可能出现负数，我们需要将其加 K 从而使其 %K 之后的值为正数
        let key = (prefix[i] % k + k) % k;
        if let Some(v) = hash.get(&key) {
            res += v;
        }
        hash.insert(key, hash.get(&key).unwrap_or(&0) + 1);
    }
    res
}

/**
 * 930. 和相同的二元子数组
中等
相关标签
相关企业
给你一个二元数组 nums ，和一个整数 goal ，请你统计并返回有多少个和为 goal 的 非空 子数组。

子数组 是数组的一段连续部分。



示例 1：

输入：nums = [1,0,1,0,1], goal = 2
输出：4
解释：
有 4 个满足题目要求的子数组：[1,0,1]、[1,0,1,0]、[0,1,0,1]、[1,0,1]
示例 2：

输入：nums = [0,0,0,0,0], goal = 0
输出：15


提示：

1 <= nums.length <= 3 * 104
nums[i] 不是 0 就是 1
0 <= goal <= nums.length
 */
pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }
    let mut res = 0;
    let mut hash = HashMap::new();
    for i in 0..prefix.len() {
        if let Some(v) = hash.get(&(prefix[i] - goal)) {
            res += v;
        }
        hash.insert(prefix[i], hash.get(&prefix[i]).unwrap_or(&0) + 1);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_subarray_sum() {
        let nums = vec![23, 2, 4, 6, 7];
        let k = 6;
        assert_eq!(check_subarray_sum(nums, k), true);
    }

    #[test]
    fn test_subarray_sum() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(subarray_sum(nums, k), 2);
    }

    #[test]
    fn test_find_max_length() {
        let nums = vec![0, 1];
        assert_eq!(find_max_length(nums), 2);
    }

    #[test]
    fn test_subarrays_div_by_k() {
        let nums = vec![2, -2, 2, -4];
        let k = 6;
        assert_eq!(subarrays_div_by_k(nums, k), 2);
    }

    #[test]
    fn test_num_subarrays_with_sum() {
        let nums = vec![1, 0, 1, 0, 1];
        let goal = 2;
        assert_eq!(num_subarrays_with_sum(nums, goal), 4);
    }
}
