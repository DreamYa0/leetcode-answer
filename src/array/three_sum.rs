/// 三数之和
/// 给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a ，b ，c ，使得 a + b + c = 0 ？请找出所有和为 0 且 不重复 的三元组。
///
/// 示例 1：
///
/// 输入：nums = [-1,0,1,2,-1,-4]
/// 输出：[[-1,-1,2],[-1,0,1]]
/// 示例 2：
///
/// 输入：nums = []
/// 输出：[]
/// 示例 3：
///
/// 输入：nums = [0]
/// 输出：[]
///
/// 提示：
///
/// 0 <= nums.length <= 3000
/// -105 <= nums[i] <= 105
/// 解题思路：固定i元素，在通过头和尾双指针从头尾向中间遍历，找到所有符合条件的元素
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    // 数组长度
    let len = nums.len();
    let mut res = vec![];
    if len < 3 {
        return res;
    }
    // 先对数组进行排序
    nums.sort();
    
    for i in 0..len - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            // 去重
            continue;
        }

        let s = nums[i] + nums[i + 1] + nums[i + 2];
        if s > 0 {
            // 由于数组已经排序，后面无论怎么选，选出的三个数的和不会比 s 还小，所以不会找到比 s 更优的答案了。
            break;
        }
        let s = nums[i] + nums[len - 2] + nums[len - 1];
        if s < 0 {
            // 由于数组已经排序，nums[i] 加上后面任意两个数都不超过 s，所以下面的双指针就不需要跑了，只需要继续取下一个数即可。
            continue;
        }

        // 固定i，寻找first和last，使用双指针法
        let mut first = i + 1;
        // 尾指针
        let mut last = len - 1;
        while first < last {
            if nums[first] + nums[last] + nums[i] < 0 {
                // 小于0，first右移
                first += 1;
            } else if nums[first] + nums[last] + nums[i] > 0 {
                // 大于0，last左移
                last -= 1;
            } else {
                // 等于0，加入结果集
                res.push(vec![nums[i], nums[first], nums[last]]);
                // first和last去重
                while first < last && nums[first] == nums[first + 1] {
                    first += 1;
                }
                while first < last && nums[last] == nums[last - 1] {
                    last -= 1;
                }

                // first和last继续移动
                first += 1;
                last -= 1;
            }
        }
    }

    res
}

/// 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
///
/// 返回这三个数的和。
///
/// 假定每组输入只存在恰好一个解。
///
/// 示例 1：
///
/// 输入：nums = [-1,2,1,-4], target = 1
/// 输出：2
/// 解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
/// 示例 2：
///
/// 输入：nums = [0,0,0], target = 1
/// 输出：0
///
/// 提示：
///
/// 3 <= nums.length <= 1000
/// -1000 <= nums[i] <= 1000
/// -104 <= target <= 104
/// 有两个优化可以让代码击败接近 100%！（和三数之和一样）
///
/// 设 s = nums[i] + nums[i+1] + nums[i+2]。如果 s > target，由于数组已经排序，后面无论怎么选，
/// 选出的三个数的和不会比 s 还小，所以不会找到比 s 更优的答案了。所以只要 s > target，就可以直接 break 外层循环了。
/// 在 break 前判断 s 是否离 target 更近，如果更近，那么更新答案为 s。
///
/// 设 s = nums[i] + nums[n-2] + nums[n-1]。如果 s < target，由于数组已经排序，
/// nums[i] 加上后面任意两个数都不超过 s，所以下面的双指针就不需要跑了，无法找到比 s 更优的答案。
/// 但是后面还有更大的 nums[i]，可能找到一个离 target 更近的三数之和，所以还需要继续枚举，continue 外层循环。
/// 在 continue 前判断 s 是否离 target 更近，如果更近，那么更新答案为 s。
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    // 三数之和
    let mut ans = 0;
    // 最小差值
    let mut min_diff = i32::MAX;
    // 先对数组进行排序
    nums.sort();
    for i in 0..len - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            // 去重
            continue;
        }
        let count = nums[i] + nums[i + 1] + nums[i + 2];
        if count > target {
            // 由于数组已经排序，后面无论怎么选，选出的三个数的和不会比 s 还小，所以不会找到比 s 更优的答案了。
            // 所以只要 s > target，就可以直接 break 外层循环了。
            if count - target < min_diff {
                ans = count;
            }
            break;
        }
        let count = nums[i] + nums[len - 2] + nums[len - 1];
        if count < target {
            // 由于数组已经排序，nums[i] 加上后面任意两个数都不超过 s，所以下面的双指针就不需要跑了，
            // 无法找到比 s 更优的答案。但是后面还有更大的 nums[i]，可能找到一个离 target 更近的三数之和，
            // 所以还需要继续枚举，continue 外层循环。
            if target - count < min_diff {
                min_diff = target - count;
                ans = count;
            }
            continue;
        }

        // 头指针
        let mut left = i + 1;
        // 尾指针
        let mut right = len - 1;
        while left < right {
            let count = nums[i] + nums[left] + nums[right];
            if count == target {
                return count;
            } else if count < target {
                // 小于target，left右移
                left += 1;
            } else {
                // 大于target，right左移
                right -= 1;
            }
            if (target - count).abs() < min_diff {
                min_diff = (target - count).abs();
                ans = count;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = three_sum(nums);
        assert_eq!(res, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn test_three_sum_closest() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let res = three_sum_closest(nums, target);
        assert_eq!(res, 2);
    }
}
