/**
 * 强化练习 1：最长快乐前缀
「快乐前缀」 是在原字符串中既是 非空 前缀也是后缀（不包括原字符串自身）的字符串。

给你一个字符串 s，请你返回它的 最长快乐前缀。如果不存在满足题意的前缀，则返回一个空字符串 "" 。



示例 1：

输入：s = "level"
输出："l"
解释：不包括 s 自己，一共有 4 个前缀（"l", "le", "lev", "leve"）和 4 个后缀（"l", "el", "vel", "evel"）。最长的既是前缀也是后缀的字符串是 "l" 。
示例 2：

输入：s = "ababab"
输出："abab"
解释："abab" 是最长的既是前缀也是后缀的字符串。题目允许前后缀在原字符串中重叠。


提示：

1 <= s.length <= 105
s 只含有小写英文字母

采用KMP算法
*/
pub fn longest_prefix_kmp(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();
    // 给定一个长度为 n 的字符串 s，其 前缀函数 被定义为一个长度为 n 的数组 pi。
    let pi = kmp(&s);
    s[..pi[s.len() - 1]].iter().collect()
}

/**
 * 28. 找出字符串中第一个匹配项的下标
简单
相关标签
相关企业
给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。如果 needle 不是 haystack 的一部分，则返回  -1 。



示例 1：

输入：haystack = "sadbutsad", needle = "sad"
输出：0
解释："sad" 在下标 0 和 6 处匹配。
第一个匹配项的下标是 0 ，所以返回 0 。
示例 2：

输入：haystack = "leetcode", needle = "leeto"
输出：-1
解释："leeto" 没有在 "leetcode" 中出现，所以返回 -1 。


提示：

1 <= haystack.length, needle.length <= 104
haystack 和 needle 仅由小写英文字符组成
 */
pub fn str_str(haystack: String, needle: String) -> i32 {
    let (haystack_len, needle_len) = (haystack.len(), needle.len());
    if haystack_len < needle_len {
        return -1;
    }
    let (haystack, needle) = (
        haystack.chars().collect::<Vec<char>>(),
        needle.chars().collect::<Vec<char>>(),
    );
    // 给定一个长度为 n 的字符串 s，其 前缀函数 被定义为一个长度为 n 的数组 pi。
    let pi: Vec<usize> = kmp(&needle);
    let mut j = 0;
    for i in 0..haystack_len {
        // KMP算法
        while j > 0 && haystack[i] != needle[j] {
            j = pi[j - 1];
        }
        if haystack[i] == needle[j] {
            j += 1;
        }
        if j == needle_len {
            return (i - needle_len + 1) as i32;
        }
    }
    return -1;
}

/**
 * 796. 旋转字符串
简单
相关标签
相关企业
给定两个字符串, s 和 goal。如果在若干次旋转操作之后，s 能变成 goal ，那么返回 true 。

s 的 旋转操作 就是将 s 最左边的字符移动到最右边。

例如, 若 s = 'abcde'，在旋转一次之后结果就是'bcdea' 。


示例 1:

输入: s = "abcde", goal = "cdeab"
输出: true
示例 2:

输入: s = "abcde", goal = "abced"
输出: false


提示:

1 <= s.length, goal.length <= 100
s 和 goal 由小写英文字母组成
 */
pub fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    let s = s.chars().collect::<Vec<char>>();
    // 预处理得到t的前缀函数
    let s_pi = kmp(&s);
    let goal = goal.chars().collect::<Vec<char>>();
    let mut j = 0;
    // 2n次遍历 2n次遍历的目的是因为需要循环匹配
    for i in 0..2 * s.len() {
        // i%n取模即可遍历s+s
        while j > 0 && s[i % s.len()] != goal[j] {
            j = s_pi[j - 1];
        }
        if s[i % s.len()] == goal[j] {
            j += 1;
            // s+s中匹配到了t
            if j == s.len() {
                return true;
            }
        }
    }
    false
}

/// KMP算法模板
fn kmp(s: &Vec<char>) -> Vec<usize> {
    let mut pi = vec![0; s.len()];
    let mut j = 0;
    for i in 1..s.len() {
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

/**
 * 214. 最短回文串
困难
相关标签
相关企业
给定一个字符串 s，你可以通过在字符串前面添加字符将其转换为
回文串
。找到并返回可以用这种方式转换的最短回文串。



示例 1：

输入：s = "aacecaaa"
输出："aaacecaaa"
示例 2：

输入：s = "abcd"
输出："dcbabcd"


提示：

0 <= s.length <= 5 * 104
s 仅由小写英文字母组成
 */
pub fn shortest_palindrome(s: String) -> String {
    // 构造一个新的字符串 s′=s+#+s^，其中 # 表示一个在 s 中未出现过的特殊字符。基于 KMP 计算出 s′的前缀函数 πs′，
    // 则 πs′数组的最后一位即表示 s 的最长回文前缀的长度。【这种做法好理解】
    let pi = kmp(&format!(
        "{}#{}",
        s.clone(),
        s.clone().chars().rev().collect::<String>()
    )
    .chars()
    .collect::<Vec<char>>());
    let s = s.chars().collect::<Vec<char>>();
    // 前缀函数的最后一位即为s的最长回文前缀的长度
    if pi[pi.len() - 1] == s.len() {
        return s.iter().collect();
    } else {
        let mut ans = s[pi[pi.len() - 1]..].iter().rev().collect::<String>();
        ans.push_str(&s.iter().collect::<String>());
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        assert_eq!(rotate_string(s, goal), true);
    }

    #[test]
    fn test_shortest_palindrome() {
        let s = "aacecaaa".to_string();
        assert_eq!(shortest_palindrome(s), "aaacecaaa".to_string());
    }
}
