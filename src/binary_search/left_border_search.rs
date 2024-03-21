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
        } else if nums[mid] > target {
            right = mid;
        }
    }
    left as i32
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
        if sum == threshold {
            right = mid;
        } else if sum < threshold {
            right = mid;
        } else if sum > threshold {
            left = mid + 1;
        }
    }
    left
}

/**
 * 658. 找到 K 个最接近的元素
中等
相关标签
相关企业
给定一个 排序好 的数组 arr ，两个整数 k 和 x ，从数组中找到最靠近 x（两数之差最小）的 k 个数。返回的结果必须要是按升序排好的。

整数 a 比整数 b 更接近 x 需要满足：

|a - x| < |b - x| 或者
|a - x| == |b - x| 且 a < b


示例 1：

输入：arr = [1,2,3,4,5], k = 4, x = 3
输出：[1,2,3,4]
示例 2：

输入：arr = [1,2,3,4,5], k = 4, x = -1
输出：[1,2,3,4]


提示：

1 <= k <= arr.length
1 <= arr.length <= 104
arr 按 升序 排列
-104 <= arr[i], x <= 104
 */
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = arr.len() - k as usize;
    while left < right {
        let mid = left + (right - left) / 2;
        if x - arr[mid] > arr[mid + k as usize] - x {
            // 在右区间
            left = mid + 1;
        } else if x - arr[mid] < arr[mid + k as usize] - x {
            // 在左区间
            right = mid;
        } else if x - arr[mid] == arr[mid + k as usize] - x {
            // 在左区间
            right = mid;
        }
    }
    arr[left..(left + k as usize)].to_vec()
}

/**
 * 2187. 完成旅途的最少时间
中等
相关标签
相关企业
提示
给你一个数组 time ，其中 time[i] 表示第 i 辆公交车完成 一趟旅途 所需要花费的时间。

每辆公交车可以 连续 完成多趟旅途，也就是说，一辆公交车当前旅途完成后，可以 立马开始 下一趟旅途。

每辆公交车 独立 运行，也就是说可以同时有多辆公交车在运行且互不影响。

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
    // x 最大值就是总趟数 * 每趟最少的时间 即公交车最小总旅途时间
    let mut right = total_trips as i64 * *time.iter().min().unwrap() as i64 + 1;
    // [left,right) 左闭右闭区间
    while left < right {
        let m = left + right >> 1;
        if f_trops(&time, m) == total_trips as i64 {
            right = m;
        } else if f_trops(&time, m) > total_trips as i64 {
            right = m;
        } else if f_trops(&time, m) < total_trips as i64 {
            left = m + 1;
        }
    }
    left
}

fn f_trops(time: &Vec<i32>, x: i64) -> i64 {
    // x 为公交车总路途时间，f(x)为总趟数，当x增大时f(x)单调递增
    time.iter().map(|&t| x / (t as i64)).sum::<i64>()
}

/**
 * 875. 爱吃香蕉的珂珂
中等
相关标签
相关企业
珂珂喜欢吃香蕉。这里有 n 堆香蕉，第 i 堆中有 piles[i] 根香蕉。警卫已经离开了，将在 h 小时后回来。

珂珂可以决定她吃香蕉的速度 k （单位：根/小时）。每个小时，她将会选择一堆香蕉，从中吃掉 k 根。

如果这堆香蕉少于 k 根，她将吃掉这堆的所有香蕉，然后这一小时内不会再吃更多的香蕉。

珂珂喜欢慢慢吃，但仍然想在警卫回来前吃掉所有的香蕉。

返回她可以在 h 小时内吃掉所有香蕉的最小速度 k（k 为整数）。



示例 1：

输入：piles = [3,6,7,11], h = 8
输出：4
示例 2：

输入：piles = [30,11,23,4,20], h = 5
输出：30
示例 3：

输入：piles = [30,11,23,4,20], h = 6
输出：23


提示：

1 <= piles.length <= 104
piles.length <= h <= 109
1 <= piles[i] <= 109
 */
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    // 一小时最少吃一根
    let mut left = 1;
    // 一小时最多吃完一堆，也就是piles最大元素
    let mut right = 1000000000 + 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if f_hour(&piles, mid as i32) == h {
            right = mid;
        } else if f_hour(&piles, mid as i32) < h {
            right = mid;
        } else if f_hour(&piles, mid as i32) > h {
            left = mid + 1;
        }
    }
    left as i32
}

fn f_hour(piles: &Vec<i32>, x: i32) -> i32 {
    // 定义：速度为 x 时，需要 f(x) 小时吃完所有香蕉
    // f(x) 随着 x 的增加单调递减
    let mut hours = 0;
    for p in piles {
        // 计算一堆橡胶需要吃多久
        hours += *p / x;
        if p % x > 0 {
            // 如果有剩余还需要加一个小时
            hours += 1;
        }
    }
    hours
}

/**
 * 1011. 在 D 天内送达包裹的能力
中等
相关标签
相关企业
提示
传送带上的包裹必须在 days 天内从一个港口运送到另一个港口。

传送带上的第 i 个包裹的重量为 weights[i]。每一天，我们都会按给出重量（weights）的顺序往传送带上装载包裹。我们装载的重量不会超过船的最大运载重量。

返回能在 days 天内将传送带上的所有包裹送达的船的最低运载能力。



示例 1：

输入：weights = [1,2,3,4,5,6,7,8,9,10], days = 5
输出：15
解释：
船舶最低载重 15 就能够在 5 天内送达所有包裹，如下所示：
第 1 天：1, 2, 3, 4, 5
第 2 天：6, 7
第 3 天：8
第 4 天：9
第 5 天：10

请注意，货物必须按照给定的顺序装运，因此使用载重能力为 14 的船舶并将包装分成 (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) 是不允许的。
示例 2：

输入：weights = [3,2,2,4,1,4], days = 3
输出：6
解释：
船舶最低载重 6 就能够在 3 天内送达所有包裹，如下所示：
第 1 天：3, 2
第 2 天：2, 4
第 3 天：1, 4
示例 3：

输入：weights = [1,2,3,1,1], days = 4
输出：3
解释：
第 1 天：1
第 2 天：2
第 3 天：3
第 4 天：1, 1


提示：

1 <= days <= weights.length <= 5 * 104
1 <= weights[i] <= 500
 */
pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    // 船的最小载重应该是 weights 数组中元素的最大值，因为每次至少得装一件货物走，不能说装不下嘛。
    let mut left = 0;
    // 最大载重显然就是weights 数组所有元素之和，也就是一次把所有货物都装走。
    let mut right = 1;
    for w in &weights {
        left = left.max(*w);
        right += *w;
    }
    while left < right {
        let mid = (left + right) >> 1;
        if f_days(&weights, mid) == days {
            right = mid;
        } else if f_days(&weights, mid) < days {
            right = mid;
        } else if f_days(&weights, mid) > days {
            left = mid + 1;
        }
    }
    left as i32
}

fn f_days(weights: &Vec<i32>, x: i32) -> i32 {
    // 定义：当运载能力为 x 时，需要 f(x) 天运完所有货物
    // f(x) 随着 x 的增加单调递减
    let mut day = 0;
    let mut left = 0;
    while left < weights.len() {
        let mut capcity = x;
        while left < weights.len() {
            if weights[left] > capcity {
                break;
            }
            // 计算剩余运力
            capcity -= weights[left];
            left += 1;
        }
        day += 1;
    }
    day
}

/**
 * 410. 分割数组的最大值
困难
相关标签
相关企业
给定一个非负整数数组 nums 和一个整数 k ，你需要将这个数组分成 k 个非空的连续子数组。

设计一个算法使得这 k 个子数组各自和的最大值最小。



示例 1：

输入：nums = [7,2,5,10,8], k = 2
输出：18
解释：
一共有四种方法将 nums 分割为 2 个子数组。
其中最好的方式是将其分为 [7,2,5] 和 [10,8] 。
因为此时这两个子数组各自的和的最大值为18，在所有情况中最小。
示例 2：

输入：nums = [1,2,3,4,5], k = 2
输出：9
示例 3：

输入：nums = [1,4,4], k = 3
输出：4


提示：

1 <= nums.length <= 1000
0 <= nums[i] <= 106
1 <= k <= min(50, nums.length)

解题思路：
其实，这道题和上面讲的运输问题是一模一样的，不相信的话我给你改写一下题目：

你只有一艘货船，现在有若干货物，每个货物的重量是 nums[i]，现在你需要在 m 天内将这些货物运走，请问你的货船的最小载重是多少？

这不就是刚才我们解决的力扣第 1011 题「在 D 天内送达包裹的能力」吗？

货船每天运走的货物就是 nums 的一个子数组；在 m 天内运完就是将 nums 划分成 m 个子数组；
让货船的载重尽可能小，就是让所有子数组中最大的那个子数组元素之和尽可能小。
 */
pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 1;
    for w in &nums {
        left = left.max(*w);
        right += *w;
    }
    while left < right {
        let mid = (left + right) >> 1;
        if f_days(&nums, mid) == k {
            right = mid;
        } else if f_days(&nums, mid) < k {
            right = mid;
        } else if f_days(&nums, mid) > k {
            left = mid + 1;
        }
    }
    left as i32
}

/**
 * 2300. 咒语和药水的成功对数
中等
相关标签
相关企业
提示
给你两个正整数数组 spells 和 potions ，长度分别为 n 和 m ，其中 spells[i] 表示第 i 个咒语的能量强度，potions[j] 表示第 j 瓶药水的能量强度。

同时给你一个整数 success 。一个咒语和药水的能量强度 相乘 如果 大于等于 success ，那么它们视为一对 成功 的组合。

请你返回一个长度为 n 的整数数组 pairs，其中 pairs[i] 是能跟第 i 个咒语成功组合的 药水 数目。



示例 1：

输入：spells = [5,1,3], potions = [1,2,3,4,5], success = 7
输出：[4,0,3]
解释：
- 第 0 个咒语：5 * [1,2,3,4,5] = [5,10,15,20,25] 。总共 4 个成功组合。
- 第 1 个咒语：1 * [1,2,3,4,5] = [1,2,3,4,5] 。总共 0 个成功组合。
- 第 2 个咒语：3 * [1,2,3,4,5] = [3,6,9,12,15] 。总共 3 个成功组合。
所以返回 [4,0,3] 。
示例 2：

输入：spells = [3,1,2], potions = [8,5,8], success = 16
输出：[2,0,2]
解释：
- 第 0 个咒语：3 * [8,5,8] = [24,15,24] 。总共 2 个成功组合。
- 第 1 个咒语：1 * [8,5,8] = [8,5,8] 。总共 0 个成功组合。
- 第 2 个咒语：2 * [8,5,8] = [16,10,16] 。总共 2 个成功组合。
所以返回 [2,0,2] 。


提示：

n == spells.length
m == potions.length
1 <= n, m <= 105
1 <= spells[i], potions[i] <= 105
1 <= success <= 1010
 */
pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut potions = potions;
    // 对potions进行排序
    potions.sort_unstable();
    let mut res = vec![0; spells.len()];
    for (i, spell) in spells.iter().enumerate() {
        // 定义左右指针
        let mut left = 0;
        let mut right = potions.len();
        // [left,right)
        while left < right {
            let mid = (left + right) >> 1;
            let product = potions[mid] as i64 * *spell as i64;
            if product == success {
                right = mid;
            } else if product < success {
                left = mid + 1;
            } else if product > success {
                right = mid;
            }
        }
        res[i] = (potions.len() - left) as i32;
    }
    res
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
    fn test_smallest_divisor() {
        let nums = vec![1, 2, 5, 9];
        assert_eq!(smallest_divisor(nums, 6), 5);
    }

    #[test]
    fn test_find_closest_elements() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(find_closest_elements(arr, 4, 3), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_minimum_time() {
        let time = vec![1, 2, 3];
        assert_eq!(minimum_time(time, 5), 3);
    }

    #[test]
    fn test_successful_pairs() {
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;
        let successful_pairs = successful_pairs(spells, potions, success);
        assert_eq!(successful_pairs, vec![2, 0, 2])
    }
}
