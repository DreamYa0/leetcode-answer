/**
 * 78. 子集
中等
相关标签
相关企业
给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的
子集
（幂集）。

解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。


```
示例 1：

输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
示例 2：

输入：nums = [0]
输出：[[],[0]]


提示：

1 <= nums.length <= 10
-10 <= nums[i] <= 10
nums 中的所有元素 互不相同
```
 */
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    do_subsets(&nums, 0, &mut path, &mut res);
    res
}

fn do_subsets(nums: &Vec<i32>, start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    // 收集下一层递归的结果
    res.push(path.to_vec());

    if start >= nums.len() {
        // 递归终止条件
        return;
    }
    for i in start..nums.len() {
        // 放入结果
        path.push(nums[i]);
        // 递归
        do_subsets(nums, i + 1, path, res);
        // 回溯
        path.pop();
    }
}

/**
 * 90. 子集 II
中等
相关标签
相关企业
给你一个整数数组 nums ，其中可能包含重复元素，请你返回该数组所有可能的<span data-keyword="subset">子集</span>（幂集）。

解集 不能 包含重复的子集。返回的解集中，子集可以按 任意顺序 排列。


```
示例 1：

输入：nums = [1,2,2]
输出：[[],[1],[1,2],[1,2,2],[2],[2,2]]
示例 2：

输入：nums = [0]
输出：[[],[0]]


提示：

1 <= nums.length <= 10
-10 <= nums[i] <= 10
```
 */
pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    let mut nums = nums;
    // 要去重就必须先排序
    nums.sort();
    let mut used = vec![false; nums.len()];
    do_subsets_with_dup(&nums, 0, &mut path, &mut res, &mut used);
    res
}

fn do_subsets_with_dup(
    nums: &Vec<i32>,
    start: usize,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    used: &mut Vec<bool>,
) {
    // 收集下一层递归的结果
    res.push(path.to_vec());

    if start >= nums.len() {
        // 递归终止条件
        return;
    }
    for i in start..nums.len() {
        // used[i - 1] == true，说明同一树枝candidates[i - 1]使用过
        // used[i - 1] == false，说明同一树层candidates[i - 1]使用过
        // 而我们要对同一树层使用过的元素进行跳过
        if i > 0 && nums[i] == nums[i - 1] && used[i - 1] == false {
            // 去重
            continue;
        }
        used[i] = true;
        // 放入结果
        path.push(nums[i]);
        // 递归
        do_subsets_with_dup(nums, i + 1, path, res, used);
        // 回溯
        used[i] = false;
        path.pop();
    }
}

/**
 * 491. 非递减子序列
中等
相关标签
相关企业
给你一个整数数组 nums ，找出并返回所有该数组中不同的递增子序列，递增子序列中 至少有两个元素 。你可以按 任意顺序 返回答案。

数组中可能含有重复元素，如出现两个整数相等，也可以视作递增序列的一种特殊情况。


```
示例 1：

输入：nums = [4,6,7,7]
输出：[[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
示例 2：

输入：nums = [4,4,3,2,1]
输出：[[4,4]]


提示：

1 <= nums.length <= 15
-100 <= nums[i] <= 100
```
 */
pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut path = Vec::new();
    let mut res = Vec::new();
    do_find_subsequences(&nums, 0, &mut path, &mut res);
    res
}

fn do_find_subsequences(
    nums: &Vec<i32>,
    start: usize,
    path: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if path.len() > 1 {
        // 收集结果
        res.push(path.to_vec());
    }

    // 定义哈希表来去重
    let mut cnt = vec![0; 201];
    for i in start..nums.len() {
        if (!path.is_empty() && nums[i] < *path.last().unwrap()) || cnt[nums[i] as usize + 100] > 0
        {
            // 如果当前元素小于path中最后的元素，或 nums中的数据已经使用过了
            continue;
        }
        // 使用过的数计数加一
        cnt[nums[i] as usize + 100] += 1;
        // 处理数据
        path.push(nums[i]);
        // 递归
        do_find_subsequences(nums, i + 1, path, res);
        // 回溯
        path.pop();
    }
    // 终止递归
    return;
}

/**
 * 1863. 找出所有子集的异或总和再求和

一个数组的 异或总和 定义为数组中所有元素按位 XOR 的结果；如果数组为 空 ，则异或总和为 0 。

例如，数组 [2,5,6] 的 异或总和 为 2 XOR 5 XOR 6 = 1 。
给你一个数组 nums ，请你求出 nums 中每个 子集 的 异或总和 ，计算并返回这些值相加之 和 。

注意：在本题中，元素 相同 的不同子集应 多次 计数。

数组 a 是数组 b 的一个 子集 的前提条件是：从 b 删除几个（也可能不删除）元素能够得到 a 。


```
示例 1：

输入：nums = [1,3]
输出：6
解释：[1,3] 共有 4 个子集：
- 空子集的异或总和是 0 。
- [1] 的异或总和为 1 。
- [3] 的异或总和为 3 。
- [1,3] 的异或总和为 1 XOR 3 = 2 。
0 + 1 + 3 + 2 = 6
示例 2：

输入：nums = [5,1,6]
输出：28
解释：[5,1,6] 共有 8 个子集：
- 空子集的异或总和是 0 。
- [5] 的异或总和为 5 。
- [1] 的异或总和为 1 。
- [6] 的异或总和为 6 。
- [5,1] 的异或总和为 5 XOR 1 = 4 。
- [5,6] 的异或总和为 5 XOR 6 = 3 。
- [1,6] 的异或总和为 1 XOR 6 = 7 。
- [5,1,6] 的异或总和为 5 XOR 1 XOR 6 = 2 。
0 + 5 + 1 + 6 + 4 + 3 + 7 + 2 = 28
示例 3：

输入：nums = [3,4,5,6,7,8]
输出：480
解释：每个子集的全部异或总和值之和为 480 。


提示：

1 <= nums.length <= 12
1 <= nums[i] <= 20
```
 */
pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let mut path = Vec::new();
    let mut res = Vec::new();
    do_subset_xor_sum(&nums, 0, &mut path, &mut res);
    res.iter().sum()
}

fn do_subset_xor_sum(nums: &Vec<i32>, start: usize, path: &mut Vec<i32>, res: &mut Vec<i32>) {
    // 收集结果
    let mut val = 0;
    for i in 0..path.len() {
        // 求异或和
        val ^= path[i];
    }
    res.push(val);

    // 递归终止条件
    if start >= nums.len() {
        return;
    }
    for i in start..nums.len() {
        // 放入结果
        path.push(nums[i]);
        // 递归
        do_subset_xor_sum(nums, i + 1, path, res);
        // 回溯
        path.pop();
    }
}

/**
 * 698. 划分为k个相等的子集

给定一个整数数组  nums 和一个正整数 k，找出是否有可能把这个数组分成 k 个非空子集，其总和都相等。


```
示例 1：

输入： nums = [4, 3, 2, 3, 5, 2, 1], k = 4
输出： True
说明： 有可能将其分成 4 个子集（5），（1,4），（2,3），（2,3）等于总和。
示例 2:

输入: nums = [1,2,3,4], k = 3
输出: false


提示：

1 <= k <= len(nums) <= 16
0 < nums[i] < 10000
每个元素的频率在 [1,4] 范围内
```
 */
pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let sum = nums.iter().sum::<i32>();
    if sum % k != 0 {
        return false;
    }
    let target = sum / k;
    let mut bucket = vec![0; k as usize];
    do_can_partition_k_subsets(&nums, k, 0, target, &mut bucket)
}

fn do_can_partition_k_subsets(
    nums: &Vec<i32>,
    k: i32,
    start: usize,
    target: i32,
    bucket: &mut Vec<i32>,
) -> bool {
    // 结束条件：已经处理完所有球
    if start == nums.len() {
        // 判断现在桶中的球是否符合要求 -> 路径是否满足要求
        for i in 0..k {
            if bucket[i as usize] != target {
                return false;
            }
        }
        return true;
    }

    for i in 0..k {
        // 剪枝：放入球后超过 target 的值，选择一下桶
        if bucket[i as usize] + nums[start] > target {
            continue;
        }
        // 做选择：放入 i 号桶
        bucket[i as usize] += nums[start];
        // 处理下一个球，即 nums[start + 1]
        if do_can_partition_k_subsets(nums, k, start + 1, target, bucket) {
            return true;
        }
        // 回溯，撤销选择：挪出 i 号桶
        bucket[i as usize] -= nums[start];
    }
    // k 个桶都不满足要求
    return false;
}

pub fn can_partition_k_subsets_ii(mut nums: Vec<i32>, k: i32) -> bool {
    let (sum, n) = (nums.iter().sum::<i32>(), nums.len() as i32);
    nums.sort();
    if sum % k != 0 || sum / k < nums[0] || sum / k < nums[n as usize - 1] {
        return false;
    }
    back_trace(
        &mut nums,
        &mut vec![false; n as usize],
        n - 1,
        k,
        0,
        sum / k,
    )
}

fn back_trace(
    nums: &mut Vec<i32>,
    used: &mut Vec<bool>,
    start: i32,
    k: i32,
    sum: i32,
    target: i32,
) -> bool {
    if sum > target {
        return false;
    }
    if k == 0 {
        return true;
    }
    if sum == target {
        return back_trace(nums, used, nums.len() as i32 - 1, k - 1, 0, target);
    }

    for i in (0..=start).rev() {
        if used[i as usize]
            || (i + 1 < nums.len() as i32
                && nums[i as usize] == nums[i as usize + 1]
                && !used[i as usize + 1])
            || nums[i as usize] > target
            || sum + nums[i as usize] > target
        {
            continue;
        }
        used[i as usize] = true;
        if back_trace(nums, used, i - 1, k, sum + nums[i as usize], target) {
            return true;
        }
        used[i as usize] = false;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        let nums = vec![1, 2, 3];
        let res = subsets(nums);
        assert_eq!(
            res,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ]
        );
    }

    #[test]
    fn test_subsets_with_dup() {
        let nums = vec![1, 2, 2];
        let res = subsets_with_dup(nums);
        assert_eq!(
            res,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2],
            ]
        );
    }

    #[test]
    fn test_find_subsequences() {
        let nums = vec![4, 6, 7, 7];
        let res = find_subsequences(nums);
        assert_eq!(
            res,
            vec![
                vec![4, 6],
                vec![4, 6, 7],
                vec![4, 6, 7, 7],
                vec![4, 7],
                vec![4, 7, 7],
                vec![6, 7],
                vec![6, 7, 7],
                vec![7, 7],
            ]
        );
    }

    #[test]
    fn test_subset_xor_sum() {
        let nums = vec![1, 3];
        let res = subset_xor_sum(nums);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_can_partition_k_subsets() {
        let nums = vec![4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        let res = can_partition_k_subsets(nums, k);
        assert_eq!(res, true);
    }
}
