pub mod kmp;
pub mod str_split;
pub mod str_cnt;

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

/**
 * 459. 重复的子字符串

给定一个非空的字符串 s ，检查是否可以通过由它的一个子串重复多次构成。



示例 1:

输入: s = "abab"
输出: true
解释: 可由子串 "ab" 重复两次构成。
示例 2:

输入: s = "aba"
输出: false
示例 3:

输入: s = "abcabcabcabc"
输出: true
解释: 可由子串 "abc" 重复四次构成。 (或子串 "abcabc" 重复两次构成。)


提示：

1 <= s.length <= 104
s 由小写英文字母组成
 */
pub fn repeated_substring_pattern(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let len = s.len();
    if len == 0 {
        return false;
    };
    let mut next = vec![0; len];
    get_next(&mut next, &s);
    if next[len - 1] != 0 && len % (len - (next[len - 1])) == 0 {
        return true;
    }
    return false;
}

fn get_next(next: &mut Vec<usize>, s: &Vec<char>) {
    let len = s.len();
    let mut j = 0;
    for i in 1..len {
        while j > 0 && s[i] != s[j] {
            j = next[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        next[i] = j;
    }
}

/**
 * 1812. 判断国际象棋棋盘中一个格子的颜色

给你一个坐标 coordinates ，它是一个字符串，表示国际象棋棋盘中一个格子的坐标。下图是国际象棋棋盘示意图。

<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/03/chessboard.png" />

如果所给格子的颜色是白色，请你返回 true，如果是黑色，请返回 false 。

给定坐标一定代表国际象棋棋盘上一个存在的格子。坐标第一个字符是字母，第二个字符是数字。



示例 1：

输入：coordinates = "a1"
输出：false
解释：如上图棋盘所示，"a1" 坐标的格子是黑色的，所以返回 false 。
示例 2：

输入：coordinates = "h3"
输出：true
解释：如上图棋盘所示，"h3" 坐标的格子是白色的，所以返回 true 。
示例 3：

输入：coordinates = "c7"
输出：false


提示：

coordinates.length == 2
'a' <= coordinates[0] <= 'h'
'1' <= coordinates[1] <= '8'
 */
pub fn square_is_white(coordinates: String) -> bool {
    // 奇偶性判断，如果两位都是奇数或者两位都是偶数则为白色，否则为黑色
    coordinates.chars().nth(0).unwrap() as u8 % 2 != coordinates.chars().nth(1).unwrap() as u8 % 2
}

/**
 * LCP 17. 速算机器人

小扣在秋日市集发现了一款速算机器人。店家对机器人说出两个数字（记作 x 和 y），请小扣说出计算指令：

"A" 运算：使 x = 2 * x + y；
"B" 运算：使 y = 2 * y + x。
在本次游戏中，店家说出的数字为 x = 1 和 y = 0，小扣说出的计算指令记作仅由大写字母 A、B 组成的字符串 s，
字符串中字符的顺序表示计算顺序，请返回最终 x 与 y 的和为多少。

示例 1：

输入：s = "AB"

输出：4

解释： 经过一次 A 运算后，x = 2, y = 0。 再经过一次 B 运算，x = 2, y = 2。 最终 x 与 y 之和为 4。

提示：

0 <= s.length <= 10
s 由 'A' 和 'B' 组成
 */
pub fn calculate(s: String) -> i32 {
    let mut x = 1;
    let mut y = 0;
    for c in s.chars() {
        if c == 'A' {
            x = 2 * x + y;
        } else {
            y = 2 * y + x;
        }
    }
    x + y
}

/**
 * 2011. 执行操作后的变量值

存在一种仅支持 4 种操作和 1 个变量 X 的编程语言：

++X 和 X++ 使变量 X 的值 加 1
--X 和 X-- 使变量 X 的值 减 1
最初，X 的值是 0

给你一个字符串数组 operations ，这是由操作组成的一个列表，返回执行所有操作后， X 的 最终值 。



示例 1：

输入：operations = ["--X","X++","X++"]
输出：1
解释：操作按下述步骤执行：
最初，X = 0
--X：X 减 1 ，X =  0 - 1 = -1
X++：X 加 1 ，X = -1 + 1 =  0
X++：X 加 1 ，X =  0 + 1 =  1
示例 2：

输入：operations = ["++X","++X","X++"]
输出：3
解释：操作按下述步骤执行：
最初，X = 0
++X：X 加 1 ，X = 0 + 1 = 1
++X：X 加 1 ，X = 1 + 1 = 2
X++：X 加 1 ，X = 2 + 1 = 3
示例 3：

输入：operations = ["X++","++X","--X","X--"]
输出：0
解释：操作按下述步骤执行：
最初，X = 0
X++：X 加 1 ，X = 0 + 1 = 1
++X：X 加 1 ，X = 1 + 1 = 2
--X：X 减 1 ，X = 2 - 1 = 1
X--：X 减 1 ，X = 1 - 1 = 0


提示：

1 <= operations.length <= 100
operations[i] 将会是 "++X"、"X++"、"--X" 或 "X--"
 */
pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut res = 0;
    for s in operations {
        if s.contains("++") {
            res += 1;
        } else {
            res -= 1;
        }
    }
    res
}

/**
 * 1876. 长度为三且各字符不同的子字符串

如果一个字符串不含有任何重复字符，我们称这个字符串为 好 字符串。

给你一个字符串 s ，请你返回 s 中长度为 3 的 好子字符串 的数量。

注意，如果相同的好子字符串出现多次，每一次都应该被记入答案之中。

子字符串 是一个字符串中连续的字符序列。



示例 1：

输入：s = "xyzzaz"
输出：1
解释：总共有 4 个长度为 3 的子字符串："xyz"，"yzz"，"zza" 和 "zaz" 。
唯一的长度为 3 的好子字符串是 "xyz" 。
示例 2：

输入：s = "aababcabc"
输出：4
解释：总共有 7 个长度为 3 的子字符串："aab"，"aba"，"bab"，"abc"，"bca"，"cab" 和 "abc" 。
好子字符串包括 "abc"，"bca"，"cab" 和 "abc" 。


提示：

1 <= s.length <= 100
s​​​​​​ 只包含小写英文字母。
 */
pub fn count_good_substrings(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let mut res = 0;
    if s.len() < 3 {
        return res;
    }
    for i in 0..s.len() - 2 {
        if s[i] != s[i + 1] && s[i + 1] != s[i + 2] && s[i] != s[i + 2] {
            res += 1;
        }
    }
    res
}

/**
 * 520. 检测大写字母

我们定义，在以下情况时，单词的大写用法是正确的：

全部字母都是大写，比如 "USA" 。
单词中所有字母都不是大写，比如 "leetcode" 。
如果单词不只含有一个字母，只有首字母大写， 比如 "Google" 。
给你一个字符串 word 。如果大写用法正确，返回 true ；否则，返回 false 。



示例 1：

输入：word = "USA"
输出：true
示例 2：

输入：word = "FlaG"
输出：false


提示：

1 <= word.length <= 100
word 由小写和大写英文字母组成
 */
pub fn detect_capital_use(word: String) -> bool {
    // 统计大些字符数
    let mut cnt = 0;
    let words = word.chars().collect::<Vec<char>>();
    // 首字母是否大写
    let first = words[0].is_uppercase();
    for w in &words {
        if w.is_uppercase() {
            cnt += 1;
        }
    }
    cnt == words.len() || (cnt == 1 && first) || cnt == 0
}

/**
 * 709. 转换成小写字母
简单
相关标签
相关企业
提示
给你一个字符串 s ，将该字符串中的大写字母转换成相同的小写字母，返回新的字符串。



示例 1：

输入：s = "Hello"
输出："hello"
示例 2：

输入：s = "here"
输出："here"
示例 3：

输入：s = "LOVELY"
输出："lovely"


提示：

1 <= s.length <= 100
s 由 ASCII 字符集中的可打印字符组成
 */
pub fn to_lower_case(s: String) -> String {
    let mut bytes = s.into_bytes();
    for i in 0..bytes.len() {
        // a - z ascii 是从 97 - 122
        // A - Z ascii 是从 65 - 90
        // 大写字母和小写字母直接相差32
        if bytes[i] <= 90 && bytes[i] >= 65 {
            bytes[i] += 32;
        }
    }
    String::from_utf8(bytes).unwrap()
}

/**
 * 1704. 判断字符串的两半是否相似

给你一个偶数长度的字符串 s 。将其拆分成长度相同的两半，前一半为 a ，后一半为 b 。

两个字符串 相似 的前提是它们都含有相同数目的元音（'a'，'e'，'i'，'o'，'u'，'A'，'E'，'I'，'O'，'U'）。注意，s 可能同时含有大写和小写字母。

如果 a 和 b 相似，返回 true ；否则，返回 false 。



示例 1：

输入：s = "book"
输出：true
解释：a = "bo" 且 b = "ok" 。a 中有 1 个元音，b 也有 1 个元音。所以，a 和 b 相似。
示例 2：

输入：s = "textbook"
输出：false
解释：a = "text" 且 b = "book" 。a 中有 1 个元音，b 中有 2 个元音。因此，a 和 b 不相似。
注意，元音 o 在 b 中出现两次，记为 2 个。


提示：

2 <= s.length <= 1000
s.length 是偶数
s 由 大写和小写 字母组成
 */
pub fn halves_are_alike(s: String) -> bool {
    let vowel = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let s = s.chars().collect::<Vec<char>>();
    // 计算中间值
    let mid = s.len() >> 1;
    let mut l_cnt = 0;
    let mut r_cnt = 0;
    for i in 0..s.len() {
        if vowel.contains(&s[i]) {
            if i <= mid - 1 {
                l_cnt += 1
            } else {
                r_cnt += 1;
            }
        }
    }
    l_cnt == r_cnt
}

/**
 * 1844. 将所有数字用字符替换

给你一个下标从 0 开始的字符串 s ，它的 偶数 下标处为小写英文字母，奇数 下标处为数字。

定义一个函数 shift(c, x) ，其中 c 是一个字符且 x 是一个数字，函数返回字母表中 c 后面第 x 个字符。

比方说，shift('a', 5) = 'f' 和 shift('x', 0) = 'x' 。
对于每个 奇数 下标 i ，你需要将数字 s[i] 用 shift(s[i-1], s[i]) 替换。

请你替换所有数字以后，将字符串 s 返回。题目 保证 shift(s[i-1], s[i]) 不会超过 'z' 。



示例 1：

输入：s = "a1c1e1"
输出："abcdef"
解释：数字被替换结果如下：
- s[1] -> shift('a',1) = 'b'
- s[3] -> shift('c',1) = 'd'
- s[5] -> shift('e',1) = 'f'
示例 2：

输入：s = "a1b2c3d4e"
输出："abbdcfdhe"
解释：数字被替换结果如下：
- s[1] -> shift('a',1) = 'b'
- s[3] -> shift('b',2) = 'd'
- s[5] -> shift('c',3) = 'f'
- s[7] -> shift('d',4) = 'h'


提示：

1 <= s.length <= 100
s 只包含小写英文字母和数字。
对所有 奇数 下标处的 i ，满足 shift(s[i-1], s[i]) <= 'z' 。
 */
pub fn replace_digits(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();
    let mut res = vec![];
    for i in 0..s.len() {
        if i % 2 == 0 {
            res.push(s[i]);
        } else {
            res.push((s[i - 1] as u8 + (s[i] as u8 - 48)) as char);
        }
    }
    res.iter().collect()
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

    #[test]
    fn test_replace_digits() {
        let s = "a1b2c3d4e".to_string();
        let replace_digits = replace_digits(s);
        assert_eq!(replace_digits, "abbdcfdhe");
    }
}
