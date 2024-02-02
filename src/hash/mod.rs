use std::collections::HashMap;

pub mod anagram;
pub mod intersection;
pub mod is_happy;
pub mod majority_sum;

/// 217. 存在重复元素
///
/// 给你一个整数数组 nums 。如果任一值在数组中出现 至少两次 ，返回 true ；如果数组中每个元素互不相同，返回 false 。
///
/// 示例 1：
///
/// 输入：nums = [1,2,3,1]
///
/// 输出：true
///
/// 示例 2：
///
/// 输入：nums = [1,2,3,4]
///
/// 输出：false
///
/// 示例 3：
///
/// 输入：nums = [1,1,1,3,3,4,3,2,4,2]
///
/// 输出：true
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// -109 <= nums[i] <= 109
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    // 定义hash表用来统计数组中元素出现的次数
    let mut cns = HashMap::with_capacity(220);
    // 遍历数组
    for i in 0..nums.len() {
        // 如果元素出现次数大于2，返回true
        if *cns.get(&nums[i as usize]).or(Some(&0)).unwrap() >= 1 {
            return true;
        }
        // 否则，将元素出现次数加1
        cns.entry(nums[i as usize])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    false
}

/// 128. 最长连续序列
/// 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
///
/// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
///
///
/// 示例 1：
///
/// 输入：nums = [100,4,200,1,3,2]
/// 输出：4
/// 解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
/// 示例 2：
///
/// 输入：nums = [0,3,7,2,5,8,4,6,0,1]
/// 输出：9
///
/// 提示：
///
/// 0 <= nums.length <= 105
/// -109 <= nums[i] <= 109
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut nums = nums;
    nums.sort_unstable();
    // 定义最长连续序列长度
    let mut max_len = 0;
    // 定义当前连续序列长度
    let mut cur_len = 1;
    // 遍历数组
    for i in 1..nums.len() {
        // 如果当前元素和前一个元素相等，跳过
        if nums[i] == nums[i - 1] {
            continue;
        }
        // 如果当前元素和前一个元素相差1，当前连续序列长度加1
        if nums[i] == nums[i - 1] + 1 {
            cur_len += 1;
        } else {
            // 否则，更新最长连续序列长度
            max_len = max_len.max(cur_len);
            // 重置当前连续序列长度
            cur_len = 1;
        }
    }
    max_len.max(cur_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn test_longest_consecutive() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }
}
