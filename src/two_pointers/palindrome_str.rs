/**
 * 680. 验证回文串 II

给你一个字符串 s，最多 可以从中删除一个字符。

请你判断 s 是否能成为回文字符串：如果能，返回 true ；否则，返回 false 。

 

示例 1：

输入：s = "aba"
输出：true
示例 2：

输入：s = "abca"
输出：true
解释：你可以删除字符 'c' 。
示例 3：

输入：s = "abc"
输出：false
 

提示：

1 <= s.length <= 105
s 由小写英文字母组成
 */
pub fn valid_palindrome(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = s.len() - 1;
    // 遍历字符串数组
    while left <= right {
        if s[left] == s[right] {
            left += 1;
            right -= 1;
        } else {
            return is_palindrome_iiii(s[left + 1..right + 1].to_vec())
                || is_palindrome_iiii(s[left..right].to_vec());
        }
    }
    true
}

fn is_palindrome_iiii(s: Vec<char>) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

/**
 * LCR 018. 验证回文串

给定一个字符串 s ，验证 s 是否是 回文串 ，只考虑字母和数字字符，可以忽略字母的大小写。

本题中，将空字符串定义为有效的 回文串 。

 

示例 1:

输入: s = "A man, a plan, a canal: Panama"
输出: true
解释："amanaplanacanalpanama" 是回文串
示例 2:

输入: s = "race a car"
输出: false
解释："raceacar" 不是回文串
 

提示：

1 <= s.length <= 2 * 105
字符串 s 由 ASCII 字符组成
 

注意：本题与主站 125 题相同： https://leetcode-cn.com/problems/valid-palindrome/
 */
pub fn is_palindrome_iii(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        // 如果不是字母或者数字就跳过
        if !(s[left].is_ascii_alphabetic() || s[left].is_numeric()) {
            left += 1;
        } else if !(s[right].is_ascii_alphabetic() || s[right].is_numeric()) {
            // 如果不是字母或者数字就跳过
            right -= 1;
        } else if s[left].to_ascii_lowercase() == s[right].to_ascii_lowercase() {
            // 左右指针移动
            left += 1;
            right -= 1;
        } else {
            return false;
        }
    }
    true
}

/// 125. 验证回文串
/// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
/// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
/// 例如，121 是回文，而 123 不是。
///
/// 示例 1：
/// 输入：x = 121
/// 输出：true
///
/// 示例 2：
/// 输入：x = -121
/// 输出：false
/// 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
///
/// 示例 3：
/// 输入：x = 10
/// 输出：false
/// 解释：从右向左读, 为 01 。因此它不是一个回文数。
///
/// 提示：
/// -231 <= x <= 231 - 1
/// 进阶：你能不将整数转为字符串来解决这个问题吗？
/// 解题思路：使用双指针法
pub fn is_palindrome(x: i32) -> bool {
    let mut bys = x.to_string().into_bytes();
    // 定义右指针
    let mut right = bys.len() - 1;
    // 左右指针往中间 bys.len() / 2 位置移动
    for left in 0..bys.len() / 2 {
        // 交换左右指针的值
        let temp = bys[left];
        bys[left] = bys[right];
        bys[right] = temp;
        // 右指针左移
        right -= 1;
    }
    // 将字节数组转换为字符串
    let s = String::from_utf8(bys).unwrap();
    s == x.to_string()
}

pub fn is_palindrome_ii(s: String) -> bool {
    let mut chars = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();
    // 定义有指针
    let mut right = chars.len() - 1;
    // 左右指针往中间 bys.len() / 2 位置移动
    for left in 0..chars.len() / 2 {
        // 交换左右指针的值
        let temp = chars[left];
        chars[left] = chars[right];
        chars[right] = temp;
        // 右指针左移
        right -= 1;
    }
    chars.iter().collect::<String>()
        == s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>()
}

/**
 * 2697. 字典序最小回文串
简单
相关标签
提示
给你一个由 小写英文字母 组成的字符串 s ，你可以对其执行一些操作。在一步操作中，你可以用其他小写英文字母 替换  s 中的一个字符。

请你执行 尽可能少的操作 ，使 s 变成一个 回文串 。如果执行 最少 操作次数的方案不止一种，则只需选取 字典序最小 的方案。

对于两个长度相同的字符串 a 和 b ，在 a 和 b 出现不同的第一个位置，如果该位置上 a 中对应字母比 b 中对应字母在字母表中出现顺序更早，则认为 a 的字典序比 b 的字典序要小。

返回最终的回文字符串。



示例 1：

输入：s = "egcfe"
输出："efcfe"
解释：将 "egcfe" 变成回文字符串的最小操作次数为 1 ，修改 1 次得到的字典序最小回文字符串是 "efcfe"，只需将 'g' 改为 'f' 。
示例 2：

输入：s = "abcd"
输出："abba"
解释：将 "abcd" 变成回文字符串的最小操作次数为 2 ，修改 2 次得到的字典序最小回文字符串是 "abba" 。
示例 3：

输入：s = "seven"
输出："neven"
解释：将 "seven" 变成回文字符串的最小操作次数为 1 ，修改 1 次得到的字典序最小回文字符串是 "neven" 。


提示：

1 <= s.length <= 1000
s 仅由小写英文字母组成
 */
pub fn make_smallest_palindrome(s: String) -> String {
    // 把字符串转为字节数组
    let mut bytes = s.into_bytes();
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = bytes.len() - 1;
    // 遍历字节数组
    while left < right {
        if bytes[left] != bytes[right] {
            // 如果左右指针指向的元素不相等，则使用两个元素中最小的
            bytes[left] = bytes[left].min(bytes[right]);
            bytes[right] = bytes[left].min(bytes[right]);
        }
        // 右移左指针
        left += 1;
        // 左移右指针
        right -= 1;
    }
    String::from_utf8(bytes).unwrap()
}

/**
 * 2108. 找出数组中的第一个回文字符串
简单
相关标签
相关企业
提示
给你一个字符串数组 words ，找出并返回数组中的 第一个回文字符串 。如果不存在满足要求的字符串，返回一个 空字符串 "" 。

回文字符串 的定义为：如果一个字符串正着读和反着读一样，那么该字符串就是一个 回文字符串 。



示例 1：

输入：words = ["abc","car","ada","racecar","cool"]
输出："ada"
解释：第一个回文字符串是 "ada" 。
注意，"racecar" 也是回文字符串，但它不是第一个。
示例 2：

输入：words = ["notapalindrome","racecar"]
输出："racecar"
解释：第一个也是唯一一个回文字符串是 "racecar" 。
示例 3：

输入：words = ["def","ghi"]
输出：""
解释：不存在回文字符串，所以返回一个空字符串。


提示：

1 <= words.length <= 100
1 <= words[i].length <= 100
words[i] 仅由小写英文字母组成
 */
pub fn first_palindrome(words: Vec<String>) -> String {
    for word in words {
        let word = word.chars().collect::<Vec<char>>();
        if do_first_palindrome(&word) {
            return word.iter().collect();
        }
    }
    "".to_string()
}

fn do_first_palindrome(word: &Vec<char>) -> bool {
    let mut left = 0;
    let mut right = word.len() - 1;
    while left < right {
        if word[left] != word[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome() {
        let s = "abc".to_string();
        let valid_palindrome = valid_palindrome(s);
        println!("{:?}", valid_palindrome)
    }

    #[test]
    fn test_is_palindrome_iii() {
        let s = "0P".to_string();
        let is_palindrome_ii = is_palindrome_iii(s);
        println!("{:?}", is_palindrome_ii)
    }

    #[test]
    fn test_is_palindrome() {
        println!("{:?}", is_palindrome(-121))
    }

    #[test]
    fn test_is_palindrome_ii() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let res = is_palindrome_ii(s);
        println!("{:?}", res)
    }

    #[test]
    fn test_make_smallest_palindrome() {
        let s = "egcfe".to_string();
        let res = make_smallest_palindrome(s);
        println!("{:?}", res)
    }

    #[test]
    fn test_first_palindrome() {
        let words = vec![
            "abc".to_string(),
            "car".to_string(),
            "ada".to_string(),
            "racecar".to_string(),
            "cool".to_string(),
        ];
        let first_palindrome = first_palindrome(words);
        assert_eq!(first_palindrome, "ada")
    }
}