use std::collections::HashMap;

// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
// 你可以按任意顺序返回答案。
// 示例 1：
// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]
// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
// 示例 2：
// 输入：nums = [3,2,4], target = 6
// 输出：[1,2]
// 示例 3：
// 输入：nums = [3,3], target = 6
// 输出：[0,1]
// 提示：
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// 只会存在一个有效答案
// 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (index, val) in nums.iter().enumerate() {
        match map.get(&(target - val)) {
            // i为原本map中的数据索引，index为新加入的数据索引
            Some(&i) => return vec![i as i32, index as i32],
            // 不存在则把数据的值作为key，索引作为value存入map
            None => map.insert(val, index),
        };
    }
    // 返回空数组
    vec![]
}

// 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
// 请你将两个数相加，并以相同形式返回一个表示和的链表。
// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
// 示例 1：
// 输入：l1 = [2,4,3], l2 = [5,6,4]
// 输出：[7,0,8]
// 解释：342 + 465 = 807.
// 示例 2：
// 输入：l1 = [0], l2 = [0]
// 输出：[0]
// 示例 3：
// 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// 输出：[8,9,9,9,0,0,0,1]
// 提示：
// 每个链表中的节点数在范围 [1, 100] 内
// 0 <= Node.val <= 9
// 题目数据保证列表表示的数字不含前导零
#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    do_sum(l1, l2, 0)
}

fn do_sum(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_some() || l2.is_some() || carry > 0 {
        Some(Box::new(ListNode {
            next: do_sum(
                l1.and_then(|f| {
                    carry += f.val;
                    f.next
                }),
                l2.and_then(|f| {
                    carry += f.val;
                    f.next
                }),
                // 商作为进位的树
                carry / 10,
            ),
            // 余数作为当前节点的值
            val: carry % 10,
        }))
    } else {
        None
    }
}

/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
/// 复杂度分析
/// 时间复杂度：O(n)，其中 n 为 s 的长度。注意 left 至多增加 n 次，所以整个二重循环至多循环 O(n) 次。
/// 空间复杂度：O(∣Σ∣)，其中 ∣Σ∣ 为字符集合的大小，本题中字符均为 ASCII 字符，所以 ∣Σ∣≤128
pub fn length_of_longest_substring(st: String) -> i32 {
    let s = st.as_bytes();
    let mut ans = 0;
    let mut left = 0;
    let mut window = vec![false; 128]; // 也可以用 HashSet，这里为了效率用的 Vec
    for (right, &c) in s.iter().enumerate() {
        let c = c as usize;
        while window[c] {
            // 加入 c 后，窗口内会有重复元素
            window[s[left] as usize] = false;
            left += 1;
        }
        window[c] = true;
        ans = ans.max(right - left + 1); // 更新窗口长度最大值
    }
    ans as i32
}

// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
// 例如，121 是回文，而 123 不是。
// 示例 1：
// 输入：x = 121
// 输出：true
// 示例 2：
// 输入：x = -121
// 输出：false
// 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
// 示例 3：
// 输入：x = 10
// 输出：false
// 解释：从右向左读, 为 01 。因此它不是一个回文数。
// 提示：
// -231 <= x <= 231 - 1
// 进阶：你能不将整数转为字符串来解决这个问题吗？
pub fn is_palindrome(x: i32) -> bool {
    x.to_string().chars().rev().collect::<String>() == x.to_string()
}

/// 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
/// 字符          数值
/// I             1
/// V             5
/// X             10
///  L             50
///  C             100
///  D             500
///  M             1000
///  例如， 罗马数字 2 写做 II ，即为两个并列的 1 。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
///  通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
///  I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
///  X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
///  C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
///  给定一个罗马数字，将其转换成整数。
///  示例 1:
///  输入: s = "III"
///  输出: 3
///  示例 2:
///  输入: s = "IV"
///  输出: 4
///  示例 3:
///  输入: s = "IX"
///  输出: 9
///  示例 4:
///  输入: s = "LVIII"
///  输出: 58
///  解释: L = 50, V= 5, III = 3.
///  示例 5:
///  输入: s = "MCMXCIV"
///  输出: 1994
///  解释: M = 1000, CM = 900, XC = 90, IV = 4.
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
    for b in s.bytes().rev() {
        let n = match b {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => panic!(),
        };
        res += if n < last { -n } else { n };
        last = n;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let two_sum = two_sum(nums, target);
        println!("{:?}", two_sum)
    }

    #[test]
    fn test_add_two_numbers() {
        let l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        };

        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        };

        let mut numbers = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        // 遍历链表
        let mut vec = Vec::new();
        while let Some(node) = numbers {
            vec.push(node.val);
            numbers = node.next;
        }
        // 反转向量
        vec.reverse();
        println!("{:?}", vec)
    }

    #[test]
    fn test_length_of_longest_substring() {
        let st = "abcabcbb".to_string();
        let length = length_of_longest_substring(st);
        println!("{:?}", length)
    }

    #[test]
    fn test_is_palindrome() {
        println!("{:?}", is_palindrome(-121))
    }

    #[test]
    fn test_roman_to_int() {
        let roman_to_int = roman_to_int("MCMXCIV".to_string());
        println!("{:?}", roman_to_int)
    }
}
