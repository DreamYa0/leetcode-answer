/// 寻找数组的中心索引
/// 给你一个整数数组 nums ，请计算数组的 中心下标 。
///
/// 数组 中心下标 是数组的一个下标，其左侧所有元素相加的和等于右侧所有元素相加的和。
///
/// 如果中心下标位于数组最左端，那么左侧数之和视为 0 ，因为在下标的左侧不存在元素。这一点对于中心下标位于数组最右端同样适用。
///
/// 如果数组有多个中心下标，应该返回 最靠近左边 的那一个。如果数组不存在中心下标，返回 -1 。
///
///  
///
/// 示例 1：
///
/// 输入：nums = [1, 7, 3, 6, 5, 6]
/// 输出：3
/// 解释：
/// 中心下标是 3 。
/// 左侧数之和 sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11 ，
/// 右侧数之和 sum = nums[4] + nums[5] = 5 + 6 = 11 ，二者相等。
/// 
/// 示例 2：
///
/// 输入：nums = [1, 2, 3]
/// 输出：-1
/// 解释：
/// 数组中不存在满足此条件的中心下标。
/// 
/// 示例 3：
///
/// 输入：nums = [2, 1, -1]
/// 输出：0
/// 解释：
/// 中心下标是 0 。
/// 左侧数之和 sum = 0 ，（下标 0 左侧不存在元素），
/// 右侧数之和 sum = nums[1] + nums[2] = 1 + -1 = 0 。
///
/// 在Rust中，你可以通过以下步骤来找到数组的中心下标：
///
/// 首先，计算数组的总和。
/// 然后，从左到右遍历数组，对于每个元素，如果它的左侧所有元素的和等于总和减去它和它右侧所有元素的和，那么这个元素就是中心下标。
/// 如果遍历完数组都没有找到中心下标，那么返回-1。
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let total: i32 = nums.iter().sum();
    let mut left_sum = 0;
    for (i, &num) in nums.iter().enumerate() {
        // num元素右侧元素之和为：total - left_sum - num
        if left_sum == total - left_sum - num {
            return i as i32;
        }
        // 左侧元素之和
        left_sum += num;
    }
    -1
}

#[test]
fn test_pivot_index() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    assert_eq!(pivot_index(nums), 3);
}