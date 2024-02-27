pub mod replace;
pub mod reverse;

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
}
