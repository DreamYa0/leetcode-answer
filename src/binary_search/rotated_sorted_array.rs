/**
 * 33. 搜索旋转排序数组
整数数组 nums 按升序排列，数组中的值 互不相同 。

在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，
使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。
例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。

给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。

你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。



示例 1：

输入：nums = [4,5,6,7,0,1,2], target = 0
输出：4
示例 2：

输入：nums = [4,5,6,7,0,1,2], target = 3
输出：-1
示例 3：

输入：nums = [1], target = 0
输出：-1


提示：

1 <= nums.length <= 5000
-104 <= nums[i] <= 104
nums 中的每个值都 独一无二
题目数据保证 nums 在预先未知的某个下标上进行了旋转
-104 <= target <= 104
 */
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    // [left, right] 左闭右闭区间
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid as i32;
        }
        // 如果nums[left] <= nums[mid]，说明左半部分是有序的
        if nums[left as usize] <= nums[mid as usize] {
            if nums[left as usize] <= target && target < nums[mid as usize] {
                // target在左半部分
                right = mid - 1;
            } else {
                // target在右半部分
                left = mid + 1;
            }
        } else {
            // 如果nums[left] > nums[mid]，说明nums[mid]的右半部分是有序的
            if nums[mid as usize] < target && target <= nums[right as usize] {
                // target在右半部分
                left = mid + 1;
            } else {
                // target在左半部分
                right = mid - 1;
            }
        }
    }
    -1
}

/**
 * 81. 搜索旋转排序数组 II
已知存在一个按非降序排列的整数数组 nums ，数组中的值不必互不相同。

在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转 ，
使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。
例如， [0,1,2,4,4,4,5,6,6,7] 在下标 5 处经旋转后可能变为 [4,5,6,6,7,0,1,2,4,4] 。

给你 旋转后 的数组 nums 和一个整数 target ，请你编写一个函数来判断给定的目标值是否存在于数组中。
如果 nums 中存在这个目标值 target ，则返回 true ，否则返回 false 。

你必须尽可能减少整个操作步骤。



示例 1：

输入：nums = [2,5,6,0,0,1,2], target = 0
输出：true
示例 2：

输入：nums = [2,5,6,0,0,1,2], target = 3
输出：false


提示：

1 <= nums.length <= 5000
-104 <= nums[i] <= 104
题目数据保证 nums 在预先未知的某个下标上进行了旋转
-104 <= target <= 104


进阶：

这是 搜索旋转排序数组 的延伸题目，本题中的 nums  可能包含重复元素。
这会影响到程序的时间复杂度吗？会有怎样的影响，为什么？
 */
pub fn search_ii(nums: Vec<i32>, target: i32) -> bool {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return true;
        }
        // 如果nums[left] == nums[mid]，说明元素重复了，此时应该右移left指针跳过重复元素
        if nums[left as usize] == nums[mid as usize] {
            left += 1;
            continue;
        }
        // 如果nums[left] < nums[mid]，说明左半部分是有序的
        if nums[left as usize] < nums[mid as usize] {
            if nums[left as usize] <= target && target < nums[mid as usize] {
                // target在左半部分
                right = mid - 1;
            } else {
                // target在右半部分
                left = mid + 1;
            }
        } else {
            // 如果nums[left] > nums[mid]，说明nums[mid]的右半部分是有序的
            if nums[mid as usize] < target && target <= nums[right as usize] {
                // target在右半部分
                left = mid + 1;
            } else {
                // target在左半部分
                right = mid - 1;
            }
        }
    }
    false
}

/**
 * 153. 寻找旋转排序数组中的最小值
已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 旋转 后，得到输入数组。
例如，原数组 nums = [0,1,2,4,5,6,7] 在变化后可能得到：
若旋转 4 次，则可以得到 [4,5,6,7,0,1,2]
若旋转 7 次，则可以得到 [0,1,2,4,5,6,7]
注意，数组 [a[0], a[1], a[2], ..., a[n-1]] 旋转一次 的结果为数组 [a[n-1], a[0], a[1], a[2], ..., a[n-2]] 。

给你一个元素值 互不相同 的数组 nums ，它原来是一个升序排列的数组，并按上述情形进行了多次旋转。请你找出并返回数组中的 最小元素 。

你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。



示例 1：

输入：nums = [3,4,5,1,2]
输出：1
解释：原数组为 [1,2,3,4,5] ，旋转 3 次得到输入数组。
示例 2：

输入：nums = [4,5,6,7,0,1,2]
输出：0
解释：原数组为 [0,1,2,4,5,6,7] ，旋转 3 次得到输入数组。
示例 3：

输入：nums = [11,13,15,17]
输出：11
解释：原数组为 [11,13,15,17] ，旋转 4 次得到输入数组。


提示：

n == nums.length
1 <= n <= 5000
-5000 <= nums[i] <= 5000
nums 中的所有整数 互不相同
nums 原来是一个升序排序的数组，并进行了 1 至 n 次旋转
 */
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    // [left, right) 左闭右开区间
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] < nums[right as usize] {
            // 如果nums[mid] < nums[right]，说明右半部分是有序的
            right = mid;
        } else {
            // 如果nums[mid] > nums[right]，说明左半部分是有序的
            left = mid + 1;
        }
    }
    nums[left as usize]
}

/**
 * 154. 寻找旋转排序数组中的最小值 II
困难
相关标签
相关企业
已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 旋转 后，得到输入数组。例如，原数组 nums = [0,1,4,4,5,6,7] 在变化后可能得到：
若旋转 4 次，则可以得到 [4,5,6,7,0,1,4]
若旋转 7 次，则可以得到 [0,1,4,4,5,6,7]
注意，数组 [a[0], a[1], a[2], ..., a[n-1]] 旋转一次 的结果为数组 [a[n-1], a[0], a[1], a[2], ..., a[n-2]] 。

给你一个可能存在 重复 元素值的数组 nums ，它原来是一个升序排列的数组，并按上述情形进行了多次旋转。请你找出并返回数组中的 最小元素 。

你必须尽可能减少整个过程的操作步骤。



示例 1：

输入：nums = [1,3,5]
输出：1
示例 2：

输入：nums = [2,2,2,0,1]
输出：0


提示：

n == nums.length
1 <= n <= 5000
-5000 <= nums[i] <= 5000
nums 原来是一个升序排序的数组，并进行了 1 至 n 次旋转


进阶：这道题与 寻找旋转排序数组中的最小值 类似，但 nums 可能包含重复元素。允许重复会影响算法的时间复杂度吗？会如何影响，为什么？
 */
pub fn find_min_ii(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    // [left, right) 左闭右开区间
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] < nums[right as usize] {
            // 如果nums[mid] < nums[right]，说明右半部分是有序的
            right = mid;
        } else if nums[mid as usize] > nums[right as usize] {
            // 如果nums[mid] > nums[right]，说明左半部分是有序的
            left = mid + 1;
        } else {
            // 如果nums[mid] == nums[right]，说明nums[mid]和nums[right]相等
            // 难点：因为数组原本是递增有序数组，所有最小值一定会在left位置，应该尽量保证left指针靠左，所以应该左移right指针来去重
            right -= 1;
        }
    }
    nums[left as usize]
}

/**
 * 面试题 10.03. 搜索旋转数组
尝试过
中等
相关标签
相关企业
提示
搜索旋转数组。给定一个排序后的数组，包含n个整数，但这个数组已被旋转过很多次了，次数不详。请编写代码找出数组中的某个元素，假设数组元素原先是按升序排列的。若有多个相同元素，返回索引值最小的一个。

示例1:

 输入: arr = [15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14], target = 5
 输出: 8（元素5在该数组中的索引）
示例2:

 输入：arr = [15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14], target = 11
 输出：-1 （没有找到）
提示:

arr 长度范围在[1, 1000000]之间

解题思路：

                                    arr[left] <= target
                              ┌─  && target <= arr[mid]   ──>  right = mid
                              │   （目标在左边的升序区间中）         （右边界移动到mid）
  ┌─  arr[left] < arr[mid] ─┼
  │     （左边区间升序）         │
  │                           └─    否则目标在右半边          ──>  left = mid + 1
  │                                                             （左边界移动到mid+1）
  │
  │                                 arr[left] <= target
  │                           ┌─  || target <= arr[mid]   ──>  right = mid
  │                           │    （目标在左半边）                （右边界移动到mid）
 ─┼─  arr[left] > arr[mid] ─┼
  │     （左边不是升序）         │
  │                           └─    否则目标在右半边          ──>  left = mid + 1
  │                                                              （左边界移动到mid+1）
  │
  │
  │                           ┌─   arr[left] != target    ──>  left++
  │                           │     （左值不等于目标               （需要逐一清理重复值）
  └─ arr[left] == arr[mid] ─┼         说明还没找到）
      （可能是已经找到了目标      │
        也可能是遇到了重复值）    └─   arr[left] == target    ──>  right = left
                                    （左值等于目标                 （将右边界移动到left，循环结束）
                                      已经找到最左边的目标值）
 */
pub fn search_iii(arr: Vec<i32>, target: i32) -> i32 {
    // 必须有以下 if 语句才能返回最小索引
    if arr[0] == target {
        return 0;
    }
    // 以下逻辑同 medium_81
    let (mut left, mut right) = (0, arr.len() - 1);
    while left < right {
        let mid = left + ((right - left) >> 1);
        // 如果arr[mid] < arr[right]，说明右半部分是有序的
        if arr[mid] < arr[right] {
            if arr[mid] < target && target <= arr[right] {
                // target在右半部分
                left = mid + 1;
            } else {
                // target在左半部分
                right = mid;
            }
        } else if arr[mid] > arr[right] {
            // 如果arr[mid] > arr[right]，说明左半部分是有序的
            if arr[left] <= target && target <= arr[mid] {
                // target在左半部分
                right = mid;
            } else {
                // target在右半部分
                left = mid + 1;
            }
        } else {
            // 如果arr[mid] == arr[right]，说明arr[mid]和arr[right]相等
            right -= 1;
        }
    }

    if arr[left] == target {
        // 如果arr[left] == target，说明找到了目标
        left as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        let nums = vec![3, 4, 5, 1, 2];
        let find_min = find_min(nums);
        assert_eq!(find_min, 1);
    }
}