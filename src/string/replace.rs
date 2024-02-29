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

/// 680. 验证回文串 II
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
            return is_palindrome(s[left + 1..right + 1].to_vec())
                || is_palindrome(s[left..right].to_vec());
        }
    }
    true
}

fn is_palindrome(s: Vec<char>) -> bool {
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

/// LCR 018. 验证回文串
pub fn is_palindrome_ii(s: String) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        let s = "a1b2c3".to_string();
        let replace = replace(s);
        println!("{:?}", replace)
    }

    #[test]
    fn test_valid_palindrome() {
        let s = "abc".to_string();
        let valid_palindrome = valid_palindrome(s);
        println!("{:?}", valid_palindrome)
    }

    #[test]
    fn test_is_palindrome_ii() {
        let s = "0P".to_string();
        let is_palindrome_ii = is_palindrome_ii(s);
        println!("{:?}", is_palindrome_ii)
    }
}
