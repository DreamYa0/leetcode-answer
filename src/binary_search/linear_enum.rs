/**
 * 167. 两数之和 II - 输入有序数组
中等
相关标签
相关企业
给你一个下标从 1 开始的整数数组 numbers ，该数组已按 非递减顺序排列  ，请你从数组中找出满足相加之和等于目标数 target 的两个数。如果设这两个数分别是 numbers[index1] 和 numbers[index2] ，则 1 <= index1 < index2 <= numbers.length 。

以长度为 2 的整数数组 [index1, index2] 的形式返回这两个整数的下标 index1 和 index2。

你可以假设每个输入 只对应唯一的答案 ，而且你 不可以 重复使用相同的元素。

你所设计的解决方案必须只使用常量级的额外空间。


示例 1：

输入：numbers = [2,7,11,15], target = 9
输出：[1,2]
解释：2 与 7 之和等于目标数 9 。因此 index1 = 1, index2 = 2 。返回 [1, 2] 。
示例 2：

输入：numbers = [2,3,4], target = 6
输出：[1,3]
解释：2 与 4 之和等于目标数 6 。因此 index1 = 1, index2 = 3 。返回 [1, 3] 。
示例 3：

输入：numbers = [-1,0], target = -1
输出：[1,2]
解释：-1 与 0 之和等于目标数 -1 。因此 index1 = 1, index2 = 2 。返回 [1, 2] 。


提示：

2 <= numbers.length <= 3 * 104
-1000 <= numbers[i] <= 1000
numbers 按 非递减顺序 排列
-1000 <= target <= 1000
仅存在一个有效答案
 */
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;
    let mut res = vec![-1, -1];
    while left < right {
        if numbers[left] + numbers[left + 1] > target {
            break;
        }
        if numbers[right] + numbers[right - 1] < target {
            break;
        }
        if left > 0 && numbers[left] == numbers[left - 1] {
            left += 1;
            continue;
        }
        if numbers[left] + numbers[right] == target {
            res[0] = left as i32 + 1;
            res[1] = right as i32 + 1;
            break;
        } else if numbers[left] + numbers[right] < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    res
}

/**
 * LCP 28. 采购方案
简单
相关标签
相关企业
小力将 N 个零件的报价存于数组 nums。小力预算为 target，假定小力仅购买两个零件，要求购买零件的花费不超过预算，请问他有多少种采购方案。

注意：答案需要以 1e9 + 7 (1000000007) 为底取模，如：计算初始结果为：1000000008，请返回 1

示例 1：

输入：nums = [2,5,3,5], target = 6

输出：1

解释：预算内仅能购买 nums[0] 与 nums[2]。

示例 2：

输入：nums = [2,2,1,9], target = 10

输出：4

解释：符合预算的采购方案如下： nums[0] + nums[1] = 4 nums[0] + nums[2] = 3 nums[1] + nums[2] = 3 nums[2] + nums[3] = 10

提示：

2 <= nums.length <= 10^5
1 <= nums[i], target <= 10^5
 */
pub fn purchase_plans(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut res = 0;
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        if nums[left] + nums[right] <= target {
            res += right - left;
            left += 1;
        } else {
            right -= 1;
        }
    }
    (res % 1000000007) as i32
}

/**
 * LCP 18. 早餐组合
简单
相关标签
相关企业
小扣在秋日市集选择了一家早餐摊位，一维整型数组 staple 中记录了每种主食的价格，一维整型数组 drinks 中记录了每种饮料的价格。小扣的计划选择一份主食和一款饮料，且花费不超过 x 元。请返回小扣共有多少种购买方案。

注意：答案需要以 1e9 + 7 (1000000007) 为底取模，如：计算初始结果为：1000000008，请返回 1

示例 1：

输入：staple = [10,20,5], drinks = [5,5,2], x = 15

输出：6

解释：小扣有 6 种购买方案，所选主食与所选饮料在数组中对应的下标分别是： 第 1 种方案：staple[0] + drinks[0] = 10 + 5 = 15； 第 2 种方案：staple[0] + drinks[1] = 10 + 5 = 15； 第 3 种方案：staple[0] + drinks[2] = 10 + 2 = 12； 第 4 种方案：staple[2] + drinks[0] = 5 + 5 = 10； 第 5 种方案：staple[2] + drinks[1] = 5 + 5 = 10； 第 6 种方案：staple[2] + drinks[2] = 5 + 2 = 7。

示例 2：

输入：staple = [2,1,1], drinks = [8,9,5,1], x = 9

输出：8

解释：小扣有 8 种购买方案，所选主食与所选饮料在数组中对应的下标分别是： 第 1 种方案：staple[0] + drinks[2] = 2 + 5 = 7； 第 2 种方案：staple[0] + drinks[3] = 2 + 1 = 3； 第 3 种方案：staple[1] + drinks[0] = 1 + 8 = 9； 第 4 种方案：staple[1] + drinks[2] = 1 + 5 = 6； 第 5 种方案：staple[1] + drinks[3] = 1 + 1 = 2； 第 6 种方案：staple[2] + drinks[0] = 1 + 8 = 9； 第 7 种方案：staple[2] + drinks[2] = 1 + 5 = 6； 第 8 种方案：staple[2] + drinks[3] = 1 + 1 = 2；

提示：

1 <= staple.length <= 10^5
1 <= drinks.length <= 10^5
1 <= staple[i],drinks[i] <= 10^5
1 <= x <= 2*10^5
 */
pub fn breakfast_number(staple: Vec<i32>, drinks: Vec<i32>, x: i32) -> i32 {
    let mut staple = staple;
    let mut drinks = drinks;
    staple.sort();
    drinks.sort();
    let mut res = 0;
    let mut left = 0;
    let mut right = drinks.len() as i32 - 1;
    while left < staple.len() && right >= 0 {
        if staple[left] + drinks[right as usize] <= x {
            res += right + 1;
            res %= 1000000007;
            left += 1;
        } else {
            right -= 1;
        }
    }
    res as i32
}

/**
 * 1855. 下标对中的最大距离
中等
相关标签
相关企业
提示
给你两个 非递增 的整数数组 nums1​​​​​​ 和 nums2​​​​​​ ，数组下标均 从 0 开始 计数。

下标对 (i, j) 中 0 <= i < nums1.length 且 0 <= j < nums2.length 。如果该下标对同时满足 i <= j 且 nums1[i] <= nums2[j] ，则称之为 有效 下标对，该下标对的 距离 为 j - i​​ 。​​

返回所有 有效 下标对 (i, j) 中的 最大距离 。如果不存在有效下标对，返回 0 。

一个数组 arr ，如果每个 1 <= i < arr.length 均有 arr[i-1] >= arr[i] 成立，那么该数组是一个 非递增 数组。



示例 1：

输入：nums1 = [55,30,5,4,2], nums2 = [100,20,10,10,5]
输出：2
解释：有效下标对是 (0,0), (2,2), (2,3), (2,4), (3,3), (3,4) 和 (4,4) 。
最大距离是 2 ，对应下标对 (2,4) 。
示例 2：

输入：nums1 = [2,2,2], nums2 = [10,10,1]
输出：1
解释：有效下标对是 (0,0), (0,1) 和 (1,1) 。
最大距离是 1 ，对应下标对 (0,1) 。
示例 3：

输入：nums1 = [30,29,19,5], nums2 = [25,25,25,25,25]
输出：2
解释：有效下标对是 (2,2), (2,3), (2,4), (3,3) 和 (3,4) 。
最大距离是 2 ，对应下标对 (2,4) 。


提示：

1 <= nums1.length <= 105
1 <= nums2.length <= 105
1 <= nums1[i], nums2[j] <= 105
nums1 和 nums2 都是 非递增 数组
 */
pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] > nums2[j] {
            i += 1;
        } else {
            res = res.max(j as i32 - i as i32);
            j += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let numbers = vec![2, 7, 11, 15];
        assert_eq!(two_sum(numbers, 9), vec![1, 2]);
    }

    #[test]
    fn test_purchase_plans() {
        let nums = vec![2, 5, 3, 5];
        assert_eq!(purchase_plans(nums, 6), 1);
    }

    #[test]
    fn test_breakfast_number() {
        let staple = vec![10, 20, 5];
        let drinks = vec![5, 5, 2];
        assert_eq!(breakfast_number(staple, drinks, 15), 6);
    }

    #[test]
    fn test_max_distance() {
        let nums1 = vec![55, 30, 5, 4, 2];
        let nums2 = vec![100, 20, 10, 10, 5];
        assert_eq!(max_distance(nums1, nums2), 2);
    }
}
