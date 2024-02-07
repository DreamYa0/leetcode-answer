use std::collections::HashMap;
/// 1.两数之和
/// 
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
/// 
/// 思路
/// 很明显暴力的解法是两层for循环查找，时间复杂度是O(n^2)。
///
/// 建议大家做这道题目之前，先做一下这两道
///
/// 242. 有效的字母异位词(opens new window)
/// 349. 两个数组的交集(opens new window)
/// 242. 有效的字母异位词 (opens new window)这道题目是用数组作为哈希表来解决哈希问题，349. 两个数组的交集 (opens new window)这道题目是通过set作为哈希表来解决哈希问题。
///
/// 首先我再强调一下 什么时候使用哈希法，当我们需要查询一个元素是否出现过，或者一个元素是否在集合里的时候，就要第一时间想到哈希法。
///
/// 本题呢，我就需要一个集合来存放我们遍历过的元素，然后在遍历数组的时候去询问这个集合，某元素是否遍历过，也就是 是否出现在这个集合。
///
/// 那么我们就应该想到使用哈希法了。
///
/// 因为本题，我们不仅要知道元素有没有遍历过，还要知道这个元素对应的下标，需要使用 key value结构来存放，key来存元素，value来存下标，那么使用map正合适。
///
/// 再来看一下使用数组和set来做哈希法的局限。
///
/// 数组的大小是受限制的，而且如果元素很少，而哈希值太大会造成内存空间的浪费。
/// set是一个集合，里面放的元素只能是一个key，而两数之和这道题目，不仅要判断y是否存在而且还要记录y的下标位置，因为要返回x 和 y的下标。所以set 也不能用。
/// 此时就要选择另一种数据结构：map ，map是一种key value的存储结构，可以用key保存数值，用value再保存数值所在的下标。
///
/// C++中map，有三种类型：
///
/// ```
/// 映射	底层实现	是否有序	数值是否可以重复	能否更改数值	查询效率	增删效率
/// std::map	红黑树	key有序	key不可重复	key不可修改	O(log n)	O(log n)
/// std::multimap	红黑树	key有序	key可重复	key不可修改	O(log n)	O(log n)
/// std::unordered_map	哈希表	key无序	key不可重复	key不可修改	O(1)	O(1)
/// std::unordered_map 底层实现为哈希表，std::map 和std::multimap 的底层实现是红黑树。
/// ```
///
/// 同理，std::map 和std::multimap 的key也是有序的（这个问题也经常作为面试题，考察对语言容器底层的理解）。 更多哈希表的理论知识请看关于哈希表，你该了解这些！ (opens new window)。
///
/// 这道题目中并不需要key有序，选择std::unordered_map 效率更高！ 使用其他语言的录友注意了解一下自己所用语言的数据结构就行。
///
/// 接下来需要明确两点：
///
/// map用来做什么
/// map中key和value分别表示什么
/// map目的用来存放我们访问过的元素，因为遍历数组的时候，需要记录我们之前遍历过哪些元素和对应的下标，这样才能找到与当前元素相匹配的（也就是相加等于target）
///
/// 接下来是map中key和value分别表示什么。
///
/// 这道题 我们需要 给出一个元素，判断这个元素是否出现过，如果出现过，返回这个元素的下标。
///
/// 那么判断元素是否出现，这个元素就要作为key，所以数组中的元素作为key，有key对应的就是value，value用来存下标。
///
/// 所以 map中的存储结构为 {key：数据元素，value：数组元素对应的下标}。
///
/// 在遍历数组的时候，只需要向map去查询是否有和目前遍历元素匹配的数值，如果有，就找到的匹配对，如果没有，就把目前遍历的元素放进map中，因为map存放的就是我们访问过的元素。
///
/// 过程如下：
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20220711202638.png" />
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20230220223536.png" />
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

/// 第454题.四数相加II
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
        // 遍历nums1和nums2，把两个数之和储存到map中，重复出现的增加计数
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