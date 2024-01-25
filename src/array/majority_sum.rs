use std::collections::HashMap;

/// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
/// 
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// 
/// 你可以按任意顺序返回答案。
/// 
/// 示例 1：
/// 
/// 输入：nums = [2,7,11,15], target = 9
/// 输出：[0,1]
/// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
/// 
/// 示例 2：
/// 
/// 输入：nums = [3,2,4], target = 6
/// 输出：[1,2]
/// 
/// 示例 3：
/// 
/// 输入：nums = [3,3], target = 6
/// 输出：[0,1]
/// 
/// 提示：
/// 
/// 2 <= nums.length <= 104
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// 
/// 只会存在一个有效答案
/// 
/// 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (index, val) in nums.iter().enumerate() {
        match map.get(&(target - val)) {
            // i为原本map中的数据索引，index为新加入的数据索引
            Some(&i) => return vec![i as i32, index as i32],
            // 不存在则把数据的值作为key，索引作为value存入map
            None => map.insert(val, index),
        };
    }
    // 返回空数组
    vec![]
}

/// 给你四个整数数组 nums1、nums2、nums3 和 nums4 ，数组长度都是 n ，请你计算有多少个元组 (i, j, k, l) 能满足：
///
/// 0 <= i, j, k, l < n
/// nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
///
/// 示例 1：
///
/// 输入：nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
/// 输出：2
/// 解释：
/// 两个元组如下：
/// 1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1) + 2 = 0
/// 2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1) + 0 = 0
/// 
/// 示例 2：
///
/// 输入：nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
/// 输出：1
///
///   提示：
///
/// n == nums1.length
/// n == nums2.length
/// n == nums3.length
/// n == nums4.length
/// 1 <= n <= 200
/// -228 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 228
/// 
/// 思路
/// 本题咋眼一看好像和0015.三数之和，0018.四数之和差不多，其实差很多。
///
/// 本题是使用哈希法的经典题目，而0015.三数之和，0018.四数之和并不合适使用哈希法，因为三数之和和四数之和这两道题目使用哈希法在不超时的情况下做到对结果去重是很困难的，很有多细节需要处理。
///
/// 而这道题目是四个独立的数组，只要找到A[i] + B[j] + C[k] + D[l] = 0就可以，不用考虑有重复的四个元素相加等于0的情况，所以相对于题目18. 四数之和，题目15.三数之和，还是简单了不少！
///
/// 如果本题想难度升级：就是给出一个数组（而不是四个数组），在这里找出四个元素相加等于0，答案中不可以包含重复的四元组，大家可以思考一下，后续的文章我也会讲到的。
///
/// 本题解题步骤：
///
/// 首先定义 一个unordered_map，key放a和b两数之和，value 放a和b两数之和出现的次数。
/// 遍历大A和大B数组，统计两个数组元素之和，和出现的次数，放到map中。
/// 定义int变量count，用来统计a+b+c+d = 0 出现的次数。
/// 在遍历大C和大D数组，找到如果 0-(c+d) 在map中出现过的话，就用count把map中key对应的value也就是出现次数统计出来。
/// 最后返回统计值 count 就可以了
pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;
    for ele in nums1 {
        for ele2 in &nums2 {
            map.insert(ele + ele2, map.get(&(ele + ele2)).unwrap_or(&0) + 1);
        }
    }

    for ele in nums3 {
        for ele2 in &nums4 {
            if let Some(&val) = map.get(&-(ele + ele2)) {
                count += val;
            }
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let two_sum = two_sum(nums, target);
        println!("{:?}", two_sum)
    }

    #[test]
    fn test_four_sum_count() {
        let nums1 = vec![1, 2];
        let nums2 = vec![-2, -1];
        let nums3 = vec![-1, 2];
        let nums4 = vec![0, 2];
        let result = four_sum_count(nums1, nums2, nums3, nums4);
        println!("result = {:?}", result);
    }
}