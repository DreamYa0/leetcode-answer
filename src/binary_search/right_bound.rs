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

/**
 * 2226. 每个小孩最多能分到多少糖果
中等
相关标签
相关企业
提示
给你一个 下标从 0 开始 的整数数组 candies 。数组中的每个元素表示大小为 candies[i] 的一堆糖果。你可以将每堆糖果分成任意数量的 子堆 ，但 无法 再将两堆合并到一起。

另给你一个整数 k 。你需要将这些糖果分配给 k 个小孩，使每个小孩分到 相同 数量的糖果。每个小孩可以拿走 至多一堆 糖果，有些糖果可能会不被分配。

返回每个小孩可以拿走的 最大糖果数目 。



示例 1：

输入：candies = [5,8,6], k = 3
输出：5
解释：可以将 candies[1] 分成大小分别为 5 和 3 的两堆，然后把 candies[2] 分成大小分别为 5 和 1 的两堆。现在就有五堆大小分别为 5、5、3、5 和 1 的糖果。可以把 3 堆大小为 5 的糖果分给 3 个小孩。可以证明无法让每个小孩得到超过 5 颗糖果。
示例 2：

输入：candies = [2,5], k = 11
输出：0
解释：总共有 11 个小孩，但只有 7 颗糖果，但如果要分配糖果的话，必须保证每个小孩至少能得到 1 颗糖果。因此，最后每个小孩都没有得到糖果，答案是 0 。


提示：

1 <= candies.length <= 105
1 <= candies[i] <= 107
1 <= k <= 1012
 */
pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut left = 1;
    let mut right = *candies.iter().max().unwrap() as i32 + 1;
    while left < right {
        let mid = (left + right) >> 1;
        // f_candies单调递减，再右区间搜索条件为 >= k 如果单调递增在右区间搜索的条件为 <= k
        if k <= f_candies(&candies, mid as i64) {
            left = mid + 1;
        } else {
            right = mid
        }
    }
    left - 1
}

fn f_candies(candies: &Vec<i32>, x: i64) -> i64 {
    // 定义 x 每个小孩可以拿走的最大糖果数目 f(x) 可以分到糖果的小孩数量，如果x增大f(x)呈单调递减
    let mut sum = 0;
    for c in candies {
        sum += *c as i64 / x;
    }
    sum
}

/**
 * 69. x 的平方根
简单
相关标签
相关企业
提示
给你一个非负整数 x ，计算并返回 x 的 算术平方根 。

由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。

注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。



示例 1：

输入：x = 4
输出：2
示例 2：

输入：x = 8
输出：2
解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。


提示：

0 <= x <= 231 - 1
 */
pub fn my_sqrt(x: i32) -> i32 {
    let (mut left, mut right) = (0, x);
    let mut ans = -1;
    while left <= right {
        let mid = (left + right) >> 1;
        if mid * mid <= x {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    ans
}

/**
 * 367. 有效的完全平方数
已解答
简单
相关标签
相关企业
给你一个正整数 num 。如果 num 是一个完全平方数，则返回 true ，否则返回 false 。

完全平方数 是一个可以写成某个整数的平方的整数。换句话说，它可以写成某个整数和自身的乘积。

不能使用任何内置的库函数，如  sqrt 。

 

示例 1：

输入：num = 16
输出：true
解释：返回 true ，因为 4 * 4 = 16 且 4 是一个整数。
示例 2：

输入：num = 14
输出：false
解释：返回 false ，因为 3.742 * 3.742 = 14 但 3.742 不是一个整数。
 

提示：

1 <= num <= 231 - 1
 */
pub fn is_perfect_square(num: i32) -> bool {
    let (mut left, mut right) = (0, num);
    while left <= right {
        let mid = (left + right) >> 1;
        if mid * mid == num {
            return true;
        } else if mid * mid < num {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
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

    #[test]
    fn test_is_perfect_square() {
        let num = 16;
        assert_eq!(is_perfect_square(num), true);
    }
}
