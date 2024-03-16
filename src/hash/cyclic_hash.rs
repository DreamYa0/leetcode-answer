/**
 * 41. 缺失的第一个正数

给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。

请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。


示例 1：

输入：nums = [1,2,0]
输出：3
解释：范围 [1,2] 中的数字都在数组中。
示例 2：

输入：nums = [3,4,-1,1]
输出：2
解释：1 在数组中，但 2 没有。
示例 3：

输入：nums = [7,8,9,11,12]
输出：1
解释：最小的正数 1 没有出现。


提示：

1 <= nums.length <= 105
-231 <= nums[i] <= 231 - 1
 */
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    for i in 0..nums.len() {
        // 我们可以采取这样的思路：就把 1 这个数放到下标为 0 的位置， 2 这个数放到下标为 1 的位置，按照这种思路整理一遍数组。
        // 然后我们再遍历一次数组，第 1 个遇到的它的值不等于下标的那个数，就是我们要找的缺失的第一个正数。
        while nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[(nums[i] - 1) as usize] != nums[i]
        {
            let a = nums[i] as usize - 1;
            nums.swap(a, i)
        }
    }

    for i in 0..nums.len() {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }
    // 如果[1,N]中都找不到那么就返回N + 1
    nums.len() as i32 + 1
}

/**
 * LCR 120. 寻找文件副本
简单
相关标签
相关企业
设备中存有 n 个文件，文件 id 记于数组 documents。若文件 id 相同，则定义为该文件存在副本。请返回任一存在副本的文件 id。



示例 1：

输入：documents = [2, 5, 3, 0, 5, 0]
输出：0 或 5


提示：

0 ≤ documents[i] ≤ n-1
2 <= n <= 100000
 */
pub fn find_repeat_document(documents: Vec<i32>) -> i32 {
    let mut documents = documents;
    for i in 0..documents.len() {
        while documents[i] != documents[documents[i] as usize] {
            let a = documents[i] as usize;
            documents.swap(a, i);
        }
        // 如果当前位置的索引和值不相等，且和documents[documents[i]位置的值相等，那么就找到了重复的文件
        if documents[i] as usize != i && documents[i] == documents[documents[i] as usize] {
            return documents[i];
        }
    }
    -1
}

/**
 * 448. 找到所有数组中消失的数字
简单
相关标签
相关企业
提示
给你一个含 n 个整数的数组 nums ，其中 nums[i] 在区间 [1, n] 内。
请你找出所有在 [1, n] 范围内但没有出现在 nums 中的数字，并以数组的形式返回结果。



示例 1：

输入：nums = [4,3,2,7,8,2,3,1]
输出：[5,6]
示例 2：

输入：nums = [1,1]
输出：[2]


提示：

n == nums.length
1 <= n <= 105
1 <= nums[i] <= n
进阶：你能在不使用额外空间且时间复杂度为 O(n) 的情况下解决这个问题吗? 你可以假定返回的数组不算在额外空间内。
 */
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let len = nums.len();
    for i in 0..len {
        let num = nums[i];
        // 如果nums[i] - 1 位置的数大于0，就把变为负数
        if nums[(num.abs() - 1) as usize] > 0 {
            nums[(num.abs() - 1) as usize] *= -1;
        }
    }

    let mut res = Vec::with_capacity(len);
    for i in 0..len {
        // 没有变为负数的数字就是没出现的数字
        if nums[i] > 0 {
            res.push(i as i32 + 1)
        }
    }

    res
}

/**
 * 442. 数组中重复的数据

中等
相关标签
相关企业
给你一个长度为 n 的整数数组 nums ，其中 nums 的所有整数都在范围 [1, n] 内，且每个整数出现 一次 或 两次 。
请你找出所有出现 两次 的整数，并以数组形式返回。

你必须设计并实现一个时间复杂度为 O(n) 且仅使用常量额外空间的算法解决此问题。

 

示例 1：

输入：nums = [4,3,2,7,8,2,3,1]
输出：[2,3]
示例 2：

输入：nums = [1,1,2]
输出：[1]
示例 3：

输入：nums = [1]
输出：[]
 

提示：

n == nums.length
1 <= n <= 105
1 <= nums[i] <= n
nums 中的每个元素出现 一次 或 两次
 */
pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut count_num = vec![0; nums.len()];
    for val in nums.iter() {
        count_num[(*val as usize) - 1] += 1;
    }

    let mut res = Vec::with_capacity(nums.len());
    for (val, count) in count_num.iter().enumerate() {
        if *count == 2 {
            res.push(val as i32 + 1);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive() {
        let nums = vec![3, 4, -1, 1];
        let result = first_missing_positive(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_find_repeat_document() {
        let documents = vec![0, 1, 2, 3, 4, 11, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let result = find_repeat_document(documents);
        assert_eq!(result, 11);
    }
}