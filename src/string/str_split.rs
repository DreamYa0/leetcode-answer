/**
 * 1805. 字符串中不同整数的数目

给你一个字符串 word ，该字符串由数字和小写英文字母组成。

请你用空格替换每个不是数字的字符。例如，"a123bc34d8ef34" 将会变成 " 123  34 8  34" 。
注意，剩下的这些整数为（相邻彼此至少有一个空格隔开）："123"、"34"、"8" 和 "34" 。

返回对 word 完成替换后形成的 不同 整数的数目。

只有当两个整数的 不含前导零 的十进制表示不同， 才认为这两个整数也不同。



示例 1：

输入：word = "a123bc34d8ef34"
输出：3
解释：不同的整数有 "123"、"34" 和 "8" 。注意，"34" 只计数一次。
示例 2：

输入：word = "leet1234code234"
输出：2
示例 3：

输入：word = "a1b01c001"
输出：1
解释："1"、"01" 和 "001" 视为同一个整数的十进制表示，因为在比较十进制值时会忽略前导零的存在。


提示：

1 <= word.length <= 1000
word 由数字和小写英文字母组成
 */
pub fn num_different_integers(word: String) -> i32 {
    word.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start_matches('0'))
        .collect::<std::collections::HashSet<_>>()
        .len() as i32
}

/**
 * 58. 最后一个单词的长度

给你一个字符串 s，由若干单词组成，单词前后用一些空格字符隔开。返回字符串中 最后一个 单词的长度。

单词 是指仅由字母组成、不包含任何空格字符的最大
子字符串
。



示例 1：

输入：s = "Hello World"
输出：5
解释：最后一个单词是“World”，长度为5。
示例 2：

输入：s = "   fly me   to   the moon  "
输出：4
解释：最后一个单词是“moon”，长度为4。
示例 3：

输入：s = "luffy is still joyboy"
输出：6
解释：最后一个单词是长度为6的“joyboy”。


提示：

1 <= s.length <= 104
s 仅有英文字母和空格 ' ' 组成
s 中至少存在一个单词
 */
pub fn length_of_last_word(s: String) -> i32 {
    // 去掉末尾的空格
    let s = s.trim_end();
    let mut res = 0;
    for c in s.chars().rev() {
        // 反向遍历，遇到空格就停止
        if c == ' ' {
            break;
        }
        res += 1;
    }
    res
}

/**
 * 434. 字符串中的单词数
统计字符串中的单词个数，这里的单词指的是连续的不是空格的字符。

请注意，你可以假定字符串里不包括任何不可打印的字符。

示例:

输入: "Hello, my name is John"
输出: 5
解释: 这里的单词是指连续的不是空格的字符，所以 "Hello," 算作 1 个单词。
 */
pub fn count_segments(s: String) -> i32 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }
    let mut res = 0;
    let s = s.chars().collect::<Vec<char>>();
    for i in 0..s.len() {
        if (i > 0 && s[i - 1] == ' ') && s[i] != ' ' {
            res += 1;
        }
    }
    res + 1
}

/**
 * 2042. 检查句子中的数字是否递增

句子是由若干 token 组成的一个列表，token 间用 单个 空格分隔，句子没有前导或尾随空格。
每个 token 要么是一个由数字 0-9 组成的不含前导零的 正整数 ，要么是一个由小写英文字母组成的 单词 。

示例，"a puppy has 2 eyes 4 legs" 是一个由 7 个 token 组成的句子："2" 和 "4" 是数字，其他像 "puppy" 这样的 tokens 属于单词。
给你一个表示句子的字符串 s ，你需要检查 s 中的 全部 数字是否从左到右严格递增（即，除了最后一个数字，s 中的 每个 数字都严格小于它 右侧 的数字）。

如果满足题目要求，返回 true ，否则，返回 false 。



示例 1：

example-1

输入：s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles"
输出：true
解释：句子中的数字是：1, 3, 4, 6, 12 。
这些数字是按从左到右严格递增的 1 < 3 < 4 < 6 < 12 。
示例 2：

输入：s = "hello world 5 x 5"
输出：false
解释：句子中的数字是：5, 5 。这些数字不是严格递增的。
示例 3：

example-3

输入：s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
输出：false
解释：s 中的数字是：7, 51, 50, 60 。这些数字不是严格递增的。
示例 4：

输入：s = "4 5 11 26"
输出：true
解释：s 中的数字是：4, 5, 11, 26 。
这些数字是按从左到右严格递增的：4 < 5 < 11 < 26 。


提示：

3 <= s.length <= 200
s 由小写英文字母、空格和数字 0 到 9 组成（包含 0 和 9）
s 中数字 token 的数目在 2 和 100 之间（包含 2 和 100）
s 中的 token 之间由单个空格分隔
s 中至少有 两个 数字
s 中的每个数字都是一个 小于 100 的 正 数，且不含前导零
s 不含前导或尾随空格
 */
pub fn are_numbers_ascending(s: String) -> bool {
    let (mut prev, mut curr) = (0, 0);
    for &ch in s.as_bytes() {
        if ch.is_ascii_digit() {
            curr = curr * 10 + (ch - b'0') as i32;
        } else if curr != 0 {
            if prev >= curr {
                return false;
            }
            prev = curr;
            curr = 0;
        }
    }
    curr == 0 || prev < curr
}

/**
 * 2678. 老人的数目
简单
相关标签
相关企业
提示
给你一个下标从 0 开始的字符串 details 。details 中每个元素都是一位乘客的信息，信息用长度为 15 的字符串表示，表示方式如下：

前十个字符是乘客的手机号码。
接下来的一个字符是乘客的性别。
接下来两个字符是乘客的年龄。
最后两个字符是乘客的座位号。
请你返回乘客中年龄 严格大于 60 岁 的人数。



示例 1：

输入：details = ["7868190130M7522","5303914400F9211","9273338290F4010"]
输出：2
解释：下标为 0 ，1 和 2 的乘客年龄分别为 75 ，92 和 40 。所以有 2 人年龄大于 60 岁。
示例 2：

输入：details = ["1313579440F2036","2921522980M5644"]
输出：0
解释：没有乘客的年龄大于 60 岁。


提示：

1 <= details.length <= 100
details[i].length == 15
details[i] 中的数字只包含 '0' 到 '9' 。
details[i][10] 是 'M' ，'F' 或者 'O' 之一。
所有乘客的手机号码和座位号互不相同。
 */
pub fn count_seniors(details: Vec<String>) -> i32 {
    let mut res = 0;
    for s in details {
        let age = s[11..13].parse::<i32>().unwrap();
        if age > 60 {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_numbers_ascending() {
        let s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string();
        assert_eq!(are_numbers_ascending(s), true);
    }

    #[test]
    fn test_count_seniors() {
        let details = vec![
            "7868190130M7522".to_string(),
            "5303914400F9211".to_string(),
            "9273338290F4010".to_string(),
        ];
        assert_eq!(count_seniors(details), 2);
    }
}
