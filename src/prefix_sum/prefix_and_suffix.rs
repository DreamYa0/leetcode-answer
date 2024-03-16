/**
 * 1685. 有序数组中差绝对值之和
中等
相关标签
相关企业
提示
给你一个 非递减 有序整数数组 nums 。

请你建立并返回一个整数数组 result，它跟 nums 长度相同，且result[i] 等于 nums[i] 与数组中所有其他元素差的绝对值之和。

换句话说， result[i] 等于 sum(|nums[i]-nums[j]|) ，其中 0 <= j < nums.length 且 j != i （下标从 0 开始）。



示例 1：

输入：nums = [2,3,5]
输出：[4,3,5]
解释：假设数组下标从 0 开始，那么
result[0] = |2-2| + |2-3| + |2-5| = 0 + 1 + 3 = 4，
result[1] = |3-2| + |3-3| + |3-5| = 1 + 0 + 2 = 3，
result[2] = |5-2| + |5-3| + |5-5| = 3 + 2 + 0 = 5。
示例 2：

输入：nums = [1,4,6,8,10]
输出：[24,15,13,15,21]


提示：

2 <= nums.length <= 105
1 <= nums[i] <= nums[i + 1] <= 104

解题思路

首先计算出前缀和prenum，然后遍历nums，以当前元素nums[i]为分界点，i前面的元素一定是比nums[i]小；
i后面的元素一定比nums[i]大，此时前缀区间为[0,i+1),后缀区间为 [i+1,prenum.size())

由于前缀区间的每个元素都比nums[i]小，所以我们可以用nums[i]×前缀区间的个数 - 前缀区间和 便是前缀区间的结果
同样地，后缀区间中的每个元素都比nums[i]大，所以nums[i]减其中的元素一定为负数，因为我们可以反过来，
用后缀区间和 - nums[i]×后缀区间个数 便是后缀区间结果两者相加，push到ans中即可
 */
pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }
    let mut res = vec![0; nums.len()];
    for i in 0..nums.len() {
        // nums[i]*前缀区间的个数 - 前缀区间
        let pre = nums[i] * (i as i32 + 1) - prefix[i + 1];
        // 后缀区间和 - nums[i]*后缀区间个数
        let suffix =
            prefix[prefix.len() - 1] - prefix[i + 1] - nums[i] * ((nums.len() - 1 - i) as i32);
        res[i] = pre + suffix;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_absolute_differences() {
        let nums = vec![2, 3, 5];
        assert_eq!(get_sum_absolute_differences(nums), vec![4, 3, 5]);
    }
}