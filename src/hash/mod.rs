use std::collections::HashMap;

pub mod anagram;
pub mod intersection;
pub mod is_happy;
pub mod majority_sum;

/// 217. 存在重复元素
///
/// 给你一个整数数组 nums 。如果任一值在数组中出现 至少两次 ，返回 true ；如果数组中每个元素互不相同，返回 false 。
///
/// 示例 1：
///
/// 输入：nums = [1,2,3,1]
///
/// 输出：true
///
/// 示例 2：
///
/// 输入：nums = [1,2,3,4]
///
/// 输出：false
///
/// 示例 3：
///
/// 输入：nums = [1,1,1,3,3,4,3,2,4,2]
///
/// 输出：true
///
/// 提示：
///
/// 1 <= nums.length <= 105
///
/// -109 <= nums[i] <= 109
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    // 定义hash表用来统计数组中元素出现的次数
    let mut cns = HashMap::with_capacity(220);
    // 遍历数组
    for i in 0..nums.len() {
        // 如果元素出现次数大于2，返回true
        if *cns.get(&nums[i as usize]).or(Some(&0)).unwrap() >= 1 {
            return true;
        }
        // 否则，将元素出现次数加1
        cns.entry(nums[i as usize])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    false
}

/// 128. 最长连续序列
/// 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
///
/// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
///
///
/// 示例 1：
///
/// 输入：nums = [100,4,200,1,3,2]
/// 输出：4
/// 解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
/// 示例 2：
///
/// 输入：nums = [0,3,7,2,5,8,4,6,0,1]
/// 输出：9
///
/// 提示：
///
/// 0 <= nums.length <= 105
/// -109 <= nums[i] <= 109
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut nums = nums;
    nums.sort_unstable();
    // 定义最长连续序列长度
    let mut max_len = 0;
    // 定义当前连续序列长度
    let mut cur_len = 1;
    // 遍历数组
    for i in 1..nums.len() {
        // 如果当前元素和前一个元素相等，跳过
        if nums[i] == nums[i - 1] {
            continue;
        }
        // 如果当前元素和前一个元素相差1，当前连续序列长度加1
        if nums[i] == nums[i - 1] + 1 {
            cur_len += 1;
        } else {
            // 否则，更新最长连续序列长度
            max_len = max_len.max(cur_len);
            // 重置当前连续序列长度
            cur_len = 1;
        }
    }
    max_len.max(cur_len)
}

/// 290. 单词规律
///
/// 给定一种规律 pattern 和一个字符串 s ，判断 s 是否遵循相同的规律。
///
/// 这里的 遵循 指完全匹配，例如， pattern 里的每个字母和字符串 s 中的每个非空单词之间存在着双向连接的对应规律。
///
/// 示例1:
///
/// 输入: pattern = "abba", s = "dog cat cat dog"
///
/// 输出: true
///
/// 示例 2:
///
/// 输入:pattern = "abba", s = "dog cat cat fish"
///
/// 输出: false
///
/// 示例 3:
///
/// 输入: pattern = "aaaa", s = "dog cat cat dog"
///
/// 输出: false
///
/// 提示:
///
/// 1 <= pattern.length <= 300
///
/// pattern 只包含小写英文字母
///
/// 1 <= s.length <= 3000
///
/// s 只包含小写英文字母和 ' '
///
/// s 不包含 任何前导或尾随对空格
///
/// s 中每个单词都被 单个空格 分隔
///
/// 题目分析
///
/// 这道题是205.同构字符串的升级版，由字符之间的一一映射升级成了字符与字符串之间的一一映射。
///
/// 首先本质是一样的，要实现一一映射，就要用到两个哈希表分别记录字符到字符串的映射和字符串到字符的映射。
///
/// 其次，我们要对s中的单词进行提取，比较单词数量和pattern中的数量是否一致，如果数量上不一致，二者一定不匹配；
///
/// <img src="https://pic.leetcode.cn/1690248452-xPZuJG-image.png" />
pub fn word_pattern(pattern: String, s: String) -> bool {
    // 定义两个哈希表，分别用来记录字符到字符串的映射和字符串到字符的映射
    let mut p2s = HashMap::<char, String>::new();
    let mut s2p = HashMap::<String, char>::new();
    // 将字符串s按空格分割成单词
    let s: Vec<&str> = s.split_whitespace().collect();
    if pattern.len() != s.len() {
        // 如果单词数量和pattern中的数量不一致，返回false
        return false;
    }
    // 遍历pattern
    for i in 0..pattern.len() {
        // 如果字符到字符串的映射和字符串到字符的映射不一致，返回false
        if p2s
            .get(&pattern.chars().nth(i).unwrap())
            .unwrap_or(&s[i].to_string())
            != &s[i].to_string()
            || s2p.get(s[i]).unwrap_or(&pattern.chars().nth(i).unwrap())
                != &pattern.chars().nth(i).unwrap()
        {
            return false;
        }
        // 更新字符到字符串的映射和字符串到字符的映射
        p2s.insert(pattern.chars().nth(i).unwrap(), s[i].to_string());
        s2p.insert(s[i].to_string(), pattern.chars().nth(i).unwrap());
    }
    return true;
}

/// 532. 数组中的 k-diff 数对
///
/// 给你一个整数数组 nums 和一个整数 k，请你在数组中找出 不同的 k-diff 数对，并返回不同的 k-diff 数对 的数目。
///
/// k-diff 数对定义为一个整数对 (nums[i], nums[j]) ，并满足下述全部条件：
///
/// 0 <= i, j < nums.length
///
/// i != j
///
/// |nums[i] - nums[j]| == k
///
/// 注意，|val| 表示 val 的绝对值。
///
/// 示例 1：
///
/// 输入：nums = [3, 1, 4, 1, 5], k = 2
///
/// 输出：2
///
/// 解释：数组中有两个 2-diff 数对, (1, 3) 和 (3, 5)。
///
/// 尽管数组中有两个 1 ，但我们只应返回不同的数对的数量。
///
/// 示例 2：
///
/// 输入：nums = [1, 2, 3, 4, 5], k = 1
///
/// 输出：4
///
/// 解释：数组中有四个 1-diff 数对, (1, 2), (2, 3), (3, 4) 和 (4, 5) 。
///
/// 示例 3：
///
/// 输入：nums = [1, 3, 1, 5, 4], k = 0
///
/// 输出：1
///
/// 解释：数组中只有一个 0-diff 数对，(1, 1) 。
///
/// 提示：
///
/// 1 <= nums.length <= 104
///
/// -107 <= nums[i] <= 107
///
/// 0 <= k <= 107
pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
    // 定义哈希表用来记录数组中元素出现的次数,数组中的元素分布比较广，所以这个地方不适合用数组来记录元素出现的次数
    let mut cns = HashMap::<i32, i32>::new();
    // 遍历数组
    for i in 0..nums.len() {
        // 更新元素出现的次数
        cns.entry(nums[i]).and_modify(|e| *e += 1).or_insert(1);
    }
    // 定义结果
    let mut res = 0;
    // 遍历数组
    for i in 0..nums.len() {
        // 如果k为0，只需要统计数组中元素出现次数大于1的元素个数
        if k == 0 {
            if *cns.get(&nums[i]).unwrap_or(&0) > 1 {
                res += 1;
                // 更新元素出现的次数, 避免重复统计
                cns.insert(nums[i] + k, 0);
            }
        } else {
            // 否则，统计数组中元素加k后的元素个数
            if *cns.get(&(nums[i] + k)).unwrap_or(&0) > 0 {
                res += 1;
                // 更新元素出现的次数, 避免重复统计
                cns.insert(nums[i] + k, 0);
            }
        }
    }
    res
}

/// 205. 同构字符串
///
/// 给定两个字符串 s 和 t ，判断它们是否是同构的。
///
/// 如果 s 中的字符可以按某种映射关系替换得到 t ，那么这两个字符串是同构的。
///
/// 每个出现的字符都应当映射到另一个字符，同时不改变字符的顺序。不同字符不能映射到同一个字符上，相同字符只能映射到同一个字符上，字符可以映射到自己本身。
///
/// 示例 1:
///
/// 输入：s = "egg", t = "add"
///
/// 输出：true
///
/// 示例 2：
///
/// 输入：s = "foo", t = "bar"
///
/// 输出：false
///
/// 示例 3：
///
/// 输入：s = "paper", t = "title"
///
/// 输出：true
///
/// 提示：
///
/// 1 <= s.length <= 5 * 104
///
/// t.length == s.length
///
/// s 和 t 由任意有效的 ASCII 字符组成
///
/// 解题思路
///
/// 首先复习一下数学中映射的相关概念定义。设集合 s , t 中的某字符为 x , y ，
///
/// 单射：对于任意 x ，都有唯一的 y 与之对应。
///
/// 满射：对于任意 y ，至少存在一个 x 与之对应。
///
/// 双射：既是单射又是满射，又称为一一对应。
///
/// <img src="https://pic.leetcode-cn.com/1656945936-BsSBMu-Slide1.png" />
///
/// 接下来，抽象理解题目给定条件，
///
/// “每个出现的字符都应当映射到另一个字符”。代表字符集合 s , t 之间是「满射」。
///
/// “相同字符只能映射到同一个字符上，不同字符不能映射到同一个字符上”。代表字符集合 s , t 之间是「单射」。
///
/// 因此， s 和 t 之间是「双射」，满足一一对应。考虑遍历字符串，使用哈希表 s2t , t2s 分别记录 s→t t→s 的映射，
/// 当发现任意「一对多」的关系时返回 false 即可。
pub fn is_isomorphic(s: String, t: String) -> bool {
    // 定义两个hash表来存储两个字符串中字符的映射关系
    let mut s2t = HashMap::<char, char>::new();
    let mut t2s = HashMap::<char, char>::new();
    if s.len() != t.len() {
        // 如果两个字符串长度不一致，返回false
        return false;
    }
    // t字符串对应的字符数组
    let t_array = t.chars().collect::<Vec<char>>();
    let s_array = s.chars().collect::<Vec<char>>();
    // 遍历字符串s
    for i in 0..s.len() {
        if s2t.get(&s_array[i]).unwrap_or(&t_array[i]) != &t_array[i]
            || t2s.get(&t_array[i]).unwrap_or(&s_array[i]) != &s.chars().nth(i).unwrap()
        {
            return false;
        }
        s2t.insert(s_array[i], t_array[i]);
        t2s.insert(t_array[i], s_array[i]);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn test_longest_consecutive() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn test_find_pairs() {
        let nums = vec![3, 1, 4, 1, 5];
        let k = 2;
        assert_eq!(find_pairs(nums, k), 2);
    }

    #[test]
    fn test_is_isomorphic() {
        let s = "egg".to_string();
        let t = "add".to_string();
        assert_eq!(is_isomorphic(s, t), true);
    }
}
