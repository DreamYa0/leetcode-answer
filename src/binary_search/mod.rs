pub mod linear_enum;
pub mod rotated_sorted_array;
pub mod left_border_search;
pub mod right_border_search;

/**
704. 二分查找

给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。

```
示例 1:

输入: nums = [-1,0,3,5,9,12], target = 9
输出: 4
解释: 9 出现在 nums 中并且下标为 4
示例 2:

输入: nums = [-1,0,3,5,9,12], target = 2
输出: -1
解释: 2 不存在 nums 中因此返回 -1


提示：

你可以假设 nums 中的所有元素是不重复的。
n 将在 [1, 10000]之间。
nums 的每个元素都将在 [-9999, 9999]之间。
```
 */
pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    // 定义左右指针 [left,right] 闭区间
    let mut left: i32 = 0;
    let mut right: i32 = len as i32 - 1;
    while left <= right {
        // 计算中间位置
        let mid = left + (right - left) / 2;
        if nums[mid as usize] > target {
            // 目标值在左边
            right = mid - 1;
        } else if nums[mid as usize] < target {
            // 目标值在右边
            left = mid + 1;
        } else {
            // 目标值等于中间值
            return mid as i32;
        }
    }
    -1
}

/**
 * 面试题 10.05. 稀疏数组搜索

稀疏数组搜索。有个排好序的字符串数组，其中散布着一些空字符串，编写一种方法，找出给定字符串的位置。

示例1:

 输入: words = ["at", "", "", "", "ball", "", "", "car", "", "","dad", "", ""], s = "ta"
 输出：-1
 说明: 不存在返回-1。
示例2:

 输入：words = ["at", "", "", "", "ball", "", "", "car", "", "","dad", "", ""], s = "ball"
 输出：4
提示:

words的长度在[1, 1000000]之间
 */
pub fn find_string(words: Vec<String>, s: String) -> i32 {
    if words.len() < 3 {
        for i in 0..words.len() {
            if words[i] == s {
                return i as i32;
            }
        }
    } else {
        let mut left = 0;
        let mut right = words.len() - 1;
        while left <= right {
            // left指针跳过字符串
            while words[left].is_empty() && left < right {
                left += 1;
            }
            // right 指针跳过空字符串
            while words[right].is_empty() && left < right {
                right -= 1;
            }
            let mut mid = (left + right) >> 1;
            // mid 指针跳过空字符串
            while words[mid].is_empty() && mid < right {
                mid += 1;
            }
            if words[mid] < s {
                left = mid + 1;
            } else if words[mid] > s {
                right = mid - 1;
            } else {
                return mid as i32;
            }
        }
    }
    -1
}

/**
 * 34. 在排序数组中查找元素的第一个和最后一个位置
中等
相关标签
相关企业
给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 target，返回 [-1, -1]。

你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。



示例 1：

输入：nums = [5,7,7,8,8,10], target = 8
输出：[3,4]
示例 2：

输入：nums = [5,7,7,8,8,10], target = 6
输出：[-1,-1]
示例 3：

输入：nums = [], target = 0
输出：[-1,-1]


提示：

0 <= nums.length <= 105
-109 <= nums[i] <= 109
nums 是一个非递减数组
-109 <= target <= 109
 */
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }
    let left = left_border(&nums, target);
    let right = right_border(&nums, target);
    vec![left, right]
}

fn left_border(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right = nums.len() as i32;
    while left < right {
        let mid = (left + right) >> 1;
        if target <= nums[mid as usize] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    // 如果索引越界，说明数组中无目标元素，返回 -1
    if left < 0 || left >= nums.len() as i32 {
        return -1;
    }
    if nums[left as usize] == target {
        left as i32
    } else {
        -1
    }
}

fn right_border(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right = nums.len() as i32;
    while left < right {
        let mid = (left + right) >> 1;
        if target >= nums[mid as usize] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    // 判断 target 是否存在于 nums 中
    // left - 1 索引越界的话 target 肯定不存在
    if left - 1 < 0 || left - 1 >= nums.len() as i32 {
        return -1;
    }
    if nums[(left - 1) as usize] == target {
        (left - 1) as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let nums = vec![5];
        assert_eq!(binary_search(nums, -5), -1);
    }

    #[test]
    fn test_find_string() {
        let words = vec!["nd".to_string(), "ycYAoTJBUjonLxlLBy".to_string()];
        assert_eq!(find_string(words, "EOqIKUuWGxayVypXH XQ".to_string()), -1);
    }

    #[test]
    fn test_search_range() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        assert_eq!(search_range(nums, 8), vec![3, 4]);
    }
}
