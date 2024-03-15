/// 35. 搜索插入位置
/// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
///
/// 请必须使用时间复杂度为 O(log n) 的算法。
///
///
///
/// 示例 1:
///
/// 输入: nums = [1,3,5,6], target = 5
/// 输出: 2
/// 示例 2:
///
/// 输入: nums = [1,3,5,6], target = 2
/// 输出: 1
/// 示例 3:
///
/// 输入: nums = [1,3,5,6], target = 7
/// 输出: 4
///
/// 提示:
///
/// 1 <= nums.length <= 104
/// -104 <= nums[i] <= 104
/// nums 为 无重复元素 的 升序 排列数组
/// -104 <= target <= 104
/// 这个问题可以通过二分查找来解决。二分查找是一种在有序数组中查找特定元素的搜索算法。
/// 搜索过程从数组的中间元素开始，如果中间元素正好是目标值，则搜索过程结束
/// 如果目标值大于或小于中间元素，则在数组大于或小于中间元素的那一半中查找，
/// 而且跟开始一样从中间元素开始比较。如果在某一步骤数组为空，则代表找不到。
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    // [left,right)左闭右开区间
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
 * 275. H 指数 II
中等
相关标签
相关企业
提示

给你一个整数数组 citations ，其中 citations[i] 表示研究者的第 i 篇论文被引用的次数，citations 已经按照 升序排列 。

计算并返回该研究者的 h 指数。

h 指数的定义：h 代表“高引用次数”（high citations），
一名科研人员的 h 指数是指他（她）的 （n 篇论文中）至少 有 h 篇论文分别被引用了至少 h 次。

请你设计并实现对数时间复杂度的算法解决此问题。


```
示例 1：

输入：citations = [0,1,3,5,6]
输出：3
解释：给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 0, 1, 3, 5, 6 次。
     由于研究者有3篇论文每篇 至少 被引用了 3 次，其余两篇论文每篇被引用 不多于 3 次，所以她的 h 指数是 3 。
示例 2：

输入：citations = [1,2,100]
输出：2


提示：

n == citations.length
1 <= n <= 105
0 <= citations[i] <= 1000
citations 按 升序排列
```
 */
pub fn h_index(citations: Vec<i32>) -> i32 {
    // 在区间 [left, right] 内询问
    let n = citations.len();
    let mut left = 1;
    let mut right = n;
    while left <= right {
        // 区间不为空
        // 循环不变量：
        // left-1 的回答一定为「是」
        // right+1 的回答一定为「否」
        let mid = (left + right) / 2;
        // 引用次数最多的 mid 篇论文，引用次数均 >= mid
        if citations[n - mid] >= mid as i32 {
            left = mid + 1; // 询问范围缩小到 [mid+1, right]
        } else {
            right = mid - 1; // 询问范围缩小到 [left, mid-1]
        }
    }
    // 循环结束后 right 等于 left-1，回答一定为「是」
    // 根据循环不变量，right 现在是最大的回答为「是」的数
    right as i32
}

/**
 * 1283. 使结果不超过阈值的最小除数
中等
相关标签
相关企业
提示
给你一个整数数组 nums 和一个正整数 threshold  ，你需要选择一个正整数作为除数，然后将数组里每个数都除以它，并对除法结果求和。

请你找出能够使上述结果小于等于阈值 threshold 的除数中 最小 的那个。

每个数除以除数后都向上取整，比方说 7/3 = 3 ， 10/2 = 5 。

题目保证一定有解。


```
示例 1：

输入：nums = [1,2,5,9], threshold = 6
输出：5
解释：如果除数为 1 ，我们可以得到和为 17 （1+2+5+9）。
如果除数为 4 ，我们可以得到和为 7 (1+1+2+3) 。如果除数为 5 ，和为 5 (1+1+1+2)。
示例 2：

输入：nums = [2,3,5,7,11], threshold = 11
输出：3
示例 3：

输入：nums = [19], threshold = 5
输出：4


提示：

1 <= nums.length <= 5 * 10^4
1 <= nums[i] <= 10^6
nums.length <= threshold <= 10^6
```
 */
pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut left = 1;
    let mut right = *nums.iter().max().unwrap();
    // [left,right) 左闭右闭区间
    while left < right {
        let mid = right + left >> 1;
        let sum: i32 = nums.iter().map(|&x| (x + mid - 1) / mid).sum();
        if sum <= threshold {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

/**
 * 2187. 完成旅途的最少时间
中等
相关标签
相关企业
提示
给你一个数组 time ，其中 time[i] 表示第 i 辆公交车完成 一趟旅途 所需要花费的时间。

每辆公交车可以 连续 完成多趟旅途，也就是说，一辆公交车当前旅途完成后，可以 立马开始 下一趟旅途。每辆公交车 独立 运行，也就是说可以同时有多辆公交车在运行且互不影响。

给你一个整数 totalTrips ，表示所有公交车 总共 需要完成的旅途数目。请你返回完成 至少 totalTrips 趟旅途需要花费的 最少 时间。



示例 1：

输入：time = [1,2,3], totalTrips = 5
输出：3
解释：
- 时刻 t = 1 ，每辆公交车完成的旅途数分别为 [1,0,0] 。
  已完成的总旅途数为 1 + 0 + 0 = 1 。
- 时刻 t = 2 ，每辆公交车完成的旅途数分别为 [2,1,0] 。
  已完成的总旅途数为 2 + 1 + 0 = 3 。
- 时刻 t = 3 ，每辆公交车完成的旅途数分别为 [3,1,1] 。
  已完成的总旅途数为 3 + 1 + 1 = 5 。
所以总共完成至少 5 趟旅途的最少时间为 3 。
示例 2：

输入：time = [2], totalTrips = 1
输出：2
解释：
只有一辆公交车，它将在时刻 t = 2 完成第一趟旅途。
所以完成 1 趟旅途的最少时间为 2 。


提示：

1 <= time.length <= 105
1 <= time[i], totalTrips <= 107
 */
pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    let mut left: i64 = 1;
    let mut right = total_trips as i64 * *time.iter().min().unwrap() as i64;
    // [left,right) 左闭右闭区间
    while left < right {
        let m = left + right >> 1;
        if time.iter().map(|&t| m / (t as i64)).sum::<i64>() >= total_trips as i64 {
            right = m;
        } else {
            left = m + 1;
        }
    }
    right
}

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
    fn test_search_insert() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(search_insert(nums, 5), 2);
    }

    #[test]
    fn test_binary_search() {
        let nums = vec![5];
        assert_eq!(binary_search(nums, -5), -1);
    }

    #[test]
    fn test_smallest_divisor() {
        let nums = vec![1, 2, 5, 9];
        assert_eq!(smallest_divisor(nums, 6), 5);
    }

    #[test]
    fn test_minimum_time() {
        let time = vec![1, 2, 3];
        assert_eq!(minimum_time(time, 5), 3);
    }

    #[test]
    fn test_find_min() {
        let nums = vec![3, 4, 5, 1, 2];
        let find_min = find_min(nums);
        assert_eq!(find_min, 1);
    }
}
