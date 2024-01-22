// 搜索插入位置
// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

// 请必须使用时间复杂度为 O(log n) 的算法。

//

// 示例 1:

// 输入: nums = [1,3,5,6], target = 5
// 输出: 2
// 示例 2:

// 输入: nums = [1,3,5,6], target = 2
// 输出: 1
// 示例 3:

// 输入: nums = [1,3,5,6], target = 7
// 输出: 4

// 提示:

// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums 为 无重复元素 的 升序 排列数组
// -104 <= target <= 104
// 这个问题可以通过二分查找来解决。二分查找是一种在有序数组中查找特定元素的搜索算法。
// 搜索过程从数组的中间元素开始，如果中间元素正好是目标值，则搜索过程结束
// 如果目标值大于或小于中间元素，则在数组大于或小于中间元素的那一半中查找，
// 而且跟开始一样从中间元素开始比较。如果在某一步骤数组为空，则代表找不到。
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as i32
}

// 二分查找
pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    if len < 3 {
        if nums[0] == target {
            return 0;
        } else if nums[len - 1] == target {
            return (len - 1) as i32;
        } else {
            return -1;
        }
    }
    // 定义左右指针
    let mut left = 0;
    let mut right = len - 1;
    while left <= right {
        // 计算中间位置
        let mid = left + (right - left) / 2;
        // 获取中间值
        let mid_val = nums[mid];
        if mid_val > target {
            // 目标值在左边
            right = mid - 1;
        } else if mid_val < target {
            // 目标值在右边
            left = mid + 1;
        } else {
            // 目标值等于中间值
            return mid as i32;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(search_insert(nums, 5), 2);
    }

    #[test]
    fn test_binary_search() {
        let nums = vec![5];
        assert_eq!(binary_search(nums, -5), -1);
    }
}
