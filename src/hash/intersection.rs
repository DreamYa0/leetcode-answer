/// 349.两个数组的交集
/// 给定两个数组 nums1 和 nums2 ，返回 它们的交集 。输出结果中的每个元素一定是 唯一 的。我们可以 不考虑输出结果的顺序 。
///
///
/// 示例 1：
///
/// 输入：nums1 = [1,2,2,1], nums2 = [2,2]
/// 输出：[2]
/// 示例 2：
///
/// 输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
/// 输出：[9,4]
/// 解释：[4,9] 也是可通过的
///
/// 提示：
///
/// 1 <= nums1.length, nums2.length <= 1000
/// 0 <= nums1[i], nums2[i] <= 1000
/// 
/// 思路
/// 由于nums1和nums2的长度<=1000 且 数据大小范围是 0-1000，分布也不算太广所以可以使用数组这种哈希结构，否则需要使用Set这种数据结构
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    // 创建一个临时数组，由于nums1和nums2中最大数<=1000,所以创建一个1005大小的数组足够用
    let mut temp = [0; 1005];
    if nums1.len() < nums2.len() {
        // 如果发现nums1大小小于nums2，就交换一下他们
        let temp_nums = nums1;
        nums1 = nums2;
        nums2 = temp_nums;
    }

    for num in nums1.iter() {
        // nums1中的数据在数组中记录为1，重复的数字记录一次就可以了
        temp[*num as usize] = 1;
    }

    let mut set = std::collections::HashSet::with_capacity(nums1.len());
    for num in nums2 {
        if temp[num as usize] == 1 {
            set.insert(num);
        }
    }

    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let nums1 = [1, 2, 2, 1].to_vec();
        let nums2 = [2, 2].to_vec();
        let intersection = intersection(nums1, nums2);
        println!("{:?}", intersection)
    }
}
