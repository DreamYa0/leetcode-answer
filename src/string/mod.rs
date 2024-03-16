pub mod kmp;

/// 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
///
/// 字符          数值
/// I             1
/// V             5
/// X             10
///  L             50
///  C             100
///  D             500
///  M             1000
///
///  例如， 罗马数字 2 写做 II ，即为两个并列的 1 。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
///  通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
///  I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
///  X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
///  C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
///  给定一个罗马数字，将其转换成整数。
///
///  示例 1:
///  输入: s = "III"
///  输出: 3
///
///  示例 2:
///  输入: s = "IV"
///  输出: 4
///
///  示例 3:
///  输入: s = "IX"
///  输出: 9
///
///  示例 4:
///  输入: s = "LVIII"
///  输出: 58
///  解释: L = 50, V= 5, III = 3.
///
///  示例 5:
///  输入: s = "MCMXCIV"
///  输出: 1994
///  解释: M = 1000, CM = 900, XC = 90, IV = 4.
///
///  提示：
///  1 <= s.length <= 15
///  s 仅含字符 ('I', 'V', 'X', 'L', 'C', 'D', 'M')
///  题目数据保证 s 是一个有效的罗马数字，且表示整数在范围 [1, 3999] 内
///  题目所给测试用例皆符合罗马数字书写规则，不会出现跨位等情况。
///  IL 和 IM 这样的例子并不符合题目要求，49 应该写作 XLIX，999 应该写作 CMXCIX 。
/// 关于罗马数字的详尽书写规则，可以参考 罗马数字 - Mathematics 。
pub fn roman_to_int(s: String) -> i32 {
    // 用byte达到最高效率，反向迭代，若上一数字大于当前数字则减去，反之则加
    let mut res = 0;
    let mut last = 0;
    for c in s.chars().rev() {
        let n = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!(),
        };
        res += if n < last { -n } else { n };
        last = n;
    }
    res
}

/// 1614. 括号的最大嵌套深度
pub fn max_depth(s: String) -> i32 {
    // 定义最大嵌套深度
    let mut max_len = 0;
    // 定义当前嵌套深度
    let mut cur_len = 0;
    // 遍历栈找寻合适的左右括号
    for (_, c) in s.chars().enumerate() {
        if c == '(' {
            cur_len += 1;
        }
        if c == ')' {
            cur_len -= 1;
        }
        max_len = max_len.max(cur_len);
    }
    max_len as i32
}

/// 1021. 删除最外层的括号
pub fn remove_outer_parentheses(s: String) -> String {
    // 存放结果
    let mut cns = Vec::with_capacity(s.len());
    // 统计
    let mut cnt = 0;
    // 遍历字符串
    for c in s.chars() {
        if c == ')' {
            cnt -= 1;
        }
        if cnt >= 1 {
            cns.push(c)
        }
        if c == '(' {
            cnt += 1;
        }
    }
    cns.iter().collect()
}

/// 1332. 删除回文子序列
pub fn remove_palindrome_sub(s: String) -> i32 {
    let len = s.len();
    let s = s.chars().collect::<Vec<char>>();
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = len - 1;
    while left < right {
        // 如果不相等说明字符串自身不是回文字符串，则需要删除两次，否则只需要删除一次
        if s[left] != s[right] {
            return 2;
        }
        left += 1;
        right -= 1;
    }
    1
}

/// 替换数字
/// 给定一个字符串 s，它包含小写字母和数字字符，请编写一个函数，将字符串中的字母字符保持不变，而将每个数字字符替换为number。
///
/// 例如，对于输入字符串 "a1b2c3"，函数应该将其转换为 "anumberbnumbercnumber"。
///
/// 对于输入字符串 "a5b"，函数应该将其转换为 "anumberb"
///
/// 输入：一个字符串 s,s 仅包含小写字母和数字字符。
///
/// 输出：打印一个新的字符串，其中每个数字字符都被替换为了number
///
/// 样例输入：a1b2c3
///
/// 样例输出：anumberbnumbercnumber
///
/// 数据范围：1 <= s.length < 10000。
///
/// 思路
/// 如果想把这道题目做到极致，就不要只用额外的辅助空间了！ （不过使用Java刷题的录友，一定要使用辅助空间，因为Java里的string不能修改）
///
/// 首先扩充数组到每个数字字符替换成 "number" 之后的大小。
///
/// 例如 字符串 "a5b" 的长度为3，那么 将 数字字符变成字符串 "number" 之后的字符串为 "anumberb" 长度为 8。
///
/// 如图：
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20231030165201.png" />
///
/// 然后从后向前替换数字字符，也就是双指针法，过程如下：i指向新长度的末尾，j指向旧长度的末尾。
///
/// <img src="https://code-thinking-1253855093.file.myqcloud.com/pics/20231030173058.png" />
///
/// 有同学问了，为什么要从后向前填充，从前向后填充不行么？
///
/// 从前向后填充就是O(n^2)的算法了，因为每次添加元素都要将添加元素之后的所有元素整体向后移动。
///
/// 其实很多数组填充类的问题，其做法都是先预先给数组扩容带填充后的大小，然后在从后向前进行操作。
///
/// 这么做有两个好处：
///
/// 不用申请新数组。
/// 从后向前填充元素，避免了从前向后填充元素时，每次添加元素都要将添加元素之后的所有元素向后移动的问题。
pub fn replace(s: String) -> String {
    // 统计字符串中有多少数字
    let mut s = s.chars().collect::<Vec<char>>();
    // 统计数字的个数
    let num_count = s.iter().filter(|c| '0' <= **c && **c <= '9').count();
    let len = s.len();
    // 对数组进行扩容，扩容后数组大小为：len + 5 * num_count，设置 'c' 来占位
    s.resize(len + 5 * num_count, 'c');
    // 新长度的右指针
    let mut new_right = s.len() - 1;
    // 老长度的右指针
    let mut old_rigth = len - 1;

    // old_rigth和new_right都为0时终止循环
    while old_rigth < new_right {
        if s[old_rigth] > '9' || s[old_rigth] < '0' {
            // 如果不是数字就移动到新的位置
            s[new_right] = s[old_rigth];
        } else {
            // 替换数字
            s[new_right] = 'r';
            s[new_right - 1] = 'e';
            s[new_right - 2] = 'b';
            s[new_right - 3] = 'm';
            s[new_right - 4] = 'u';
            s[new_right - 5] = 'n';
            new_right -= 5;
        }
        // 左移指针
        old_rigth -= 1;
        new_right -= 1;
    }

    s.iter().collect::<String>()
}

/// 459.重复的子字符串
/// https://leetcode.cn/problems/repeated-substring-pattern/description/
///
pub fn repeated_substring_pattern(_s: String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let roman_to_int = roman_to_int("MCMXCIV".to_string());
        println!("{:?}", roman_to_int)
    }

    #[test]
    fn test_max_depth() {
        let s = "(1+(2*3)+((8)/4))+1".to_string();
        assert_eq!(max_depth(s), 3);
    }

    #[test]
    fn test_remove_outer_parentheses() {
        let s = "(()())(())".to_string();
        assert_eq!(remove_outer_parentheses(s), "()()()");
    }

    #[test]
    fn test_remove_palindrome_sub() {
        let s = "ababa".to_string();
        assert_eq!(remove_palindrome_sub(s), 1);
    }

    #[test]
    fn test_replace() {
        let s = "a1b2c3".to_string();
        let replace = replace(s);
        println!("{:?}", replace)
    }
}
