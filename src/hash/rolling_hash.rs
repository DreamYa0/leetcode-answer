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

解题思路：滚动哈希
 */
pub fn longest_prefix(s: String) -> String {
    let s = s.into_bytes();
    // 定义前缀
    let mut prefix = 0;
    // 定义后缀
    let mut suffix = 0;
    // 进制
    let base = 31;
    // 模底
    let m = 1000000007;
    // 存储base的幂
    let mut mul = 1;
    let mut happy = 0;
    for i in 1..s.len() {
        // # 前缀哈希
        prefix = (prefix * base + (s[i - 1] - b'a') as i64) % m;
        // # 后缀哈希
        suffix = (suffix + (s[s.len() - i] - b'a') as i64 * mul) % m;
        if prefix == suffix {
            happy = i;
        }
        mul = mul * base % m;
    }
    String::from_utf8(s[..happy].to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_prefix() {
        let s = "level".to_string();
        assert_eq!(longest_prefix(s), "l".to_string());
    }
}
