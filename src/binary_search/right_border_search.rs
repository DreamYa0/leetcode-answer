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
 * 744. 寻找比目标字母大的最小字母
简单
相关标签
相关企业
提示
给你一个字符数组 letters，该数组按非递减顺序排序，以及一个字符 target。letters 里至少有两个不同的字符。

返回 letters 中大于 target 的最小的字符。如果不存在这样的字符，则返回 letters 的第一个字符。



示例 1：

输入: letters = ["c", "f", "j"]，target = "a"
输出: "c"
解释：letters 中字典上比 'a' 大的最小字符是 'c'。
示例 2:

输入: letters = ["c","f","j"], target = "c"
输出: "f"
解释：letters 中字典顺序上大于 'c' 的最小字符是 'f'。
示例 3:

输入: letters = ["x","x","y","y"], target = "z"
输出: "x"
解释：letters 中没有一个字符在字典上大于 'z'，所以我们返回 letters[0]。


提示：

2 <= letters.length <= 104
letters[i] 是一个小写字母
letters 按非递减顺序排序
letters 最少包含两个不同的字母
target 是一个小写字母
 */
pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut left = 0;
    let mut right = letters.len();
    while left < right {
        let mid = (left + right) >> 1;
        if letters[mid] == target {
            left = mid + 1;
        } else if letters[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    // 如果right等于letters.len()，说明没有找到大于target的字符，返回letters[0]
    if right == letters.len() {
        letters[0]
    } else {
        letters[left]
    }
}

/**
 * 162. 寻找峰值
中等
相关标签
相关企业
峰值元素是指其值严格大于左右相邻值的元素。

给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。

你可以假设 nums[-1] = nums[n] = -∞ 。

你必须实现时间复杂度为 O(log n) 的算法来解决此问题。



示例 1：

输入：nums = [1,2,3,1]
输出：2
解释：3 是峰值元素，你的函数应该返回其索引 2。
示例 2：

输入：nums = [1,2,1,3,5,6,4]
输出：1 或 5
解释：你的函数可以返回索引 1，其峰值元素为 2；
     或者返回索引 5， 其峰值元素为 6。


提示：

1 <= nums.length <= 1000
-231 <= nums[i] <= 231 - 1
对于所有有效的 i 都有 nums[i] != nums[i + 1]
 */
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = (left + right) >> 1;
        if nums[mid] > nums[mid + 1] {
            // 如果nums[mid]>nums[mid+1]，说明峰值只会出现在mid的左边
            right = mid;
        } else if nums[mid] < nums[mid + 1] {
            left = mid + 1;
        } else if nums[mid] == nums[mid + 1] {
            left = mid + 1;
        }
    }
    left as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greatest_letter() {
        let letters = vec!['c', 'f', 'j'];
        assert_eq!(next_greatest_letter(letters, 'a'), 'c');
    }

    #[test]
    fn test_find_peak_element() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(find_peak_element(nums), 2);
    }
}
