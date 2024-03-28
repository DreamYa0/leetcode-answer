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

/**
 * 1170. 比较字符串最小字母出现频次
中等
相关标签
相关企业
提示
定义一个函数 f(s)，统计 s  中（按字典序比较）最小字母的出现频次 ，其中 s 是一个非空字符串。

例如，若 s = "dcce"，那么 f(s) = 2，因为字典序最小字母是 "c"，它出现了 2 次。

现在，给你两个字符串数组待查表 queries 和词汇表 words 。对于每次查询 queries[i] ，
需统计 words 中满足 f(queries[i]) < f(W) 的 词的数目 ，W 表示词汇表 words 中的每个词。

请你返回一个整数数组 answer 作为答案，其中每个 answer[i] 是第 i 次查询的结果。



示例 1：

输入：queries = ["cbd"], words = ["zaaaz"]
输出：[1]
解释：查询 f("cbd") = 1，而 f("zaaaz") = 3 所以 f("cbd") < f("zaaaz")。
示例 2：

输入：queries = ["bbb","cc"], words = ["a","aa","aaa","aaaa"]
输出：[1,2]
解释：第一个查询 f("bbb") < f("aaaa")，第二个查询 f("aaa") 和 f("aaaa") 都 > f("cc")。


提示：

1 <= queries.length <= 2000
1 <= words.length <= 2000
1 <= queries[i].length, words[i].length <= 10
queries[i][j]、words[i][j] 都由小写英文字母组成

解题思路：本题由于是要去f(s)<f(w)的所以是寻找右侧边界的二分查找，如果f(s)<=f(w)的话那么就是寻找左侧边界的二分查找
 */
pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    // 把queries和words中字符字典序最小出现的次数统计出来放到数组中
    let mut q = Vec::new();
    queries.iter().for_each(|s| {
        // 用hash表来统计
        let mut map = [0; 26];
        s.chars().for_each(|c| {
            map[c as usize - 'a' as usize] += 1;
        });
        // 从左到右找到第一个不为0的元素
        let mut i = 0;
        while i < 26 && map[i] == 0 {
            i += 1;
        }
        q.push(map[i]);
    });
    let mut w = Vec::new();
    words.iter().for_each(|s| {
        let mut map = [0; 26];
        s.chars().for_each(|c| {
            map[c as usize - 'a' as usize] += 1;
        });
        let mut i = 0;
        while i < 26 && map[i] == 0 {
            i += 1;
        }
        w.push(map[i]);
    });
    // 对w进行排序
    w.sort_unstable();
    let mut res = Vec::new();
    for i in 0..q.len() {
        let mut left = 0;
        let mut right = w.len();
        while left < right {
            let mid = (left + right) >> 1;
            if q[i] == w[mid] {
                left = mid + 1;
            } else if q[i] < w[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        // w[left - 1] 是等于 q[i] 的 满足 寻找右侧边界的二分查找 定律
        res.push((w.len() - left) as i32);
    }
    res
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

    #[test]
    fn test_num_smaller_by_frequency() {
        let queries = vec!["bbb".to_string(), "cc".to_string()];
        let words = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
        ];
        assert_eq!(num_smaller_by_frequency(queries, words), vec![1, 2]);
    }
}
