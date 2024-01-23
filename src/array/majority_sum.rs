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

/// 四数之和
/// 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a], nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：
///
/// 0 <= a, b, c, d < n
/// a、b、c 和 d 互不相同
/// nums[a] + nums[b] + nums[c] + nums[d] == target
/// 你可以按 任意顺序 返回答案 。
///
/// 示例 1：
///
/// 输入：nums = [1,0,-1,0,-2,2], target = 0
/// 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
/// 示例 2：
///
/// 输入：nums = [2,2,2,2,2], target = 8
/// 输出：[[2,2,2,2]]
///
/// 提示：
///
/// 1 <= nums.length <= 200
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// 思路
/// 四数之和，和15.三数之和是一个思路，都是使用双指针法, 基本解法就是在15.三数之和 的基础上再套一层for循环。
///
/// 但是有一些细节需要注意，例如： 不要判断nums[k] > target 就返回了，三数之和 可以通过 nums[i] > 0 就返回了，因为 0 已经是确定的数了，四数之和这道题目 target是任意值。比如：数组是[-4, -3, -2, -1]，target是-10，不能因为-4 > -10而跳过。但是我们依旧可以去做剪枝，逻辑变成nums[i] > target && (nums[i] >=0 || target >= 0)就可以了。
///
/// 15.三数之和的双指针解法是一层for循环num[i]为确定值，然后循环内有left和right下标作为双指针，找到nums[i] + nums[left] + nums[right] == 0。
///
/// 四数之和的双指针解法是两层for循环nums[k] + nums[i]为确定值，依然是循环内有left和right下标作为双指针，找出nums[k] + nums[i] + nums[left] + nums[right] == target的情况，三数之和的时间复杂度是O(n^2)，四数之和的时间复杂度是O(n^3) 。
///
/// 那么一样的道理，五数之和、六数之和等等都采用这种解法。
///
/// 对于15.三数之和双指针法就是将原本暴力O(n^3)的解法，降为O(n^2)的解法，四数之和的双指针解法就是将原本暴力O(n^4)的解法，降为O(n^3)的解法。
///
/// 之前我们讲过哈希表的经典题目：454.四数相加II，相对于本题简单很多，因为本题是要求在一个集合中找出四个数相加等于target，同时四元组不能重复。
///
/// 而454.四数相加II是四个独立的数组，只要找到A[i] + B[j] + C[k] + D[l] = 0就可以，不用考虑有重复的四个元素相加等于0的情况，所以相对于本题还是简单了不少！
///
/// 我们来回顾一下，几道题目使用了双指针法。
///
/// 双指针法将时间复杂度：O(n^2)的解法优化为 O(n)的解法。也就是降一个数量级，题目如下：
///
/// 27.移除元素
/// 15.三数之和
/// 18.四数之和
/// 链表相关双指针题目：
/// 206.反转链表
/// 19.删除链表的倒数第N个节点
/// 面试题 02.07. 链表相交
/// 142题.环形链表II
/// 双指针法在字符串题目中还有很多应用，后面还会介绍到。
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(len);
    let mut nums = nums;
    nums.sort();
    for k in 0..len {
        // 剪枝, 因为是有序数组，所以如果nums[k] > target，那么后面的元素肯定都大于target，就不用再遍历了
        if nums[k] > target && (nums[k] > 0 || target > 0) {
            break;
        }
        // 去重
        if k > 0 && nums[k] == nums[k - 1] {
            continue;
        }
        for i in (k + 1)..len {
            // 剪枝，因为是有序数组，所以如果nums[k] + nums[i] > target，那么后面的元素肯定都大于target，就不用再遍历了
            if nums[k] + nums[i] > target && (nums[k] + nums[i] >= 0 || target >= 0) {
                break;
            }
            // 去重
            if i > k + 1 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, len - 1);
            while left < right {
                if nums[k] + nums[i] > target - (nums[left] + nums[right]) {
                    right -= 1;
                } else if nums[k] + nums[i] < target - (nums[left] + nums[right]) {
                    left += 1;
                } else {
                    result.push(vec![nums[k], nums[i], nums[left], nums[right]]);
                    // 去重
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }
    }
    result
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

    #[test]
    fn test_four_sum() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let result = four_sum(nums, target);
        println!("result = {:?}", result);
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