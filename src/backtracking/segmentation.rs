/**
 * 131. 分割回文串
中等
相关标签
相关企业
给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是 回文串 。返回 s 所有可能的分割方案。

回文串 是正着读和反着读都一样的字符串。


```
示例 1：

输入：s = "aab"
输出：[["a","a","b"],["aa","b"]]
示例 2：

输入：s = "a"
输出：[["a"]]


提示：

1 <= s.length <= 16
s 仅由小写英文字母组成
```
 */
pub fn partition(s: String) -> Vec<Vec<String>> {
    let s = s.chars().collect::<Vec<char>>();
    let mut path = Vec::new();
    let mut res = Vec::new();
    do_partition(&s, 0, &mut path, &mut res);
    res
}

fn do_partition(s: &Vec<char>, start: usize, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
    // 递归终止条件
    if start == s.len() {
        res.push(path.to_vec());
        return;
    }
    // 单层回溯逻辑
    for i in start..s.len() {
        if is_palindrome(s, start, i) {
            // 如果是回文串
            path.push(s[start..=i].iter().collect::<String>());
        } else {
            // 如果不是回文串，就不需要进入下一层递归
            continue;
        }
        do_partition(s, i + 1, path, res);
        // 回溯
        path.pop();
    }
}

fn is_palindrome(s: &Vec<char>, start: usize, end: usize) -> bool {
    let mut start = start;
    let mut end = end;
    while start < end {
        if s[start] != s[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

/**
 * 93. 复原 IP 地址
中等
相关标签
相关企业
有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），整数之间用 '.' 分隔。

例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。
给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP 地址，这些地址可以通过在 s 中插入 '.' 来形成。你 不能 重新排序或删除 s 中的任何数字。你可以按 任何 顺序返回答案。


```
示例 1：

输入：s = "25525511135"
输出：["255.255.11.135","255.255.111.35"]
示例 2：

输入：s = "0000"
输出：["0.0.0.0"]
示例 3：

输入：s = "101023"
输出：["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]


提示：

1 <= s.length <= 20
s 仅由数字组成
```
 */
pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut res = Vec::new();
    if s.len() < 4 || s.len() > 12 {
        return res;
    }
    let mut s = s.chars().collect::<Vec<char>>();
    do_restore_ip_addresses(&mut s, 0, 0, &mut res);
    res
}

fn do_restore_ip_addresses(
    s: &mut Vec<char>,
    start: usize,
    mut point_num: usize,
    res: &mut Vec<String>,
) {
    if point_num == 3 {
        if is_vaild(s, start, s.len() - 1) {
            res.push(s.iter().collect::<String>())
        }
        return;
    }
    for i in start..s.len() {
        if is_vaild(s, start, i) {
            point_num += 1;
            s.insert(i + 1, '.');
            do_restore_ip_addresses(s, i + 2, point_num, res);
            // 回溯
            point_num -= 1;
            s.remove(i + 1);
        } else {
            break;
        }
    }
}

fn is_vaild(s: &Vec<char>, start: usize, end: usize) -> bool {
    if start > end {
        return false;
    }
    // 长度超过3
    if end - start > 3 {
        return false;
    }
    // 以0开头,但是不是0结尾
    if s[start] == '0' && start != end {
        return false;
    }
    // 大于255
    let num = s[start..=end]
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    num <= 255
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let s = "aab".to_string();
        let res = partition(s);
        assert_eq!(res, vec![vec!["a", "a", "b"], vec!["aa", "b"]]);
    }

    #[test]
    fn test_restore_ip_addresses() {
        let s = "25525511135".to_string();
        let res = restore_ip_addresses(s);
        assert_eq!(res, vec!["255.255.11.135", "255.255.111.35"]);
    }
}
