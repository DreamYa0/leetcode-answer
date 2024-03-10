/// 搜索插入位置
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
}
