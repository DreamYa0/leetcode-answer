/// 844. 比较含退格的字符串
/// 给定 s 和 t 两个字符串，当它们分别被输入到空白的文本编辑器后，如果两者相等，返回 true 。# 代表退格字符。
///
/// 注意：如果对空文本输入退格字符，文本继续为空。
///
///
///
/// 示例 1：
///
/// 输入：s = "ab#c", t = "ad#c"
/// 输出：true
/// 解释：s 和 t 都会变成 "ac"。
/// 示例 2：
///
/// 输入：s = "ab##", t = "c#d#"
/// 输出：true
/// 解释：s 和 t 都会变成 ""。
/// 示例 3：
///
/// 输入：s = "a#c", t = "b"
/// 输出：false
/// 解释：s 会变成 "c"，但 t 仍然是 "b"。
///
///
/// 提示：
///
/// 1 <= s.length, t.length <= 200
/// s 和 t 只含有小写字母以及字符 '#'
///
///
/// 进阶：
///
/// 你可以用 O(n) 的时间复杂度和 O(1) 的空间复杂度解决该问题吗？
///
/// 解题思路
///
/// 相信大家看到该题的第一反应应该是使用栈，或者直接删除字符串来解决，但是这样做的话，空间复杂度为： n+m

/// 这无疑不是更优解，下面，我将介绍一种常量级空间复杂度的解法：双指针，并且比官方解思路更简单清晰！

/// 由于 # 号只会消除左边的一个字符，所以对右边的字符无影响，所以我们选择从后往前遍历 SSS，TTT 字符串。

/// 思路解析：

/// 准备两个指针 i, j 分别指向 S，T 的末位字符，再准备两个变量 skipS，skipT 来分别存放 S，T 字符串中的 # 数量。
/// 从后往前遍历 S，所遇情况有三，如下所示：
/// 2.1 若当前字符是 #，则 skipS 自增 1；
/// 2.2 若当前字符不是 #，且 skipS 不为 0，则 skipS 自减 1；
/// 2.3 若当前字符不是 #，且 skipS 为 0，则代表当前字符不会被消除，我们可以用来和 T 中的当前字符作比较。
/// 若对比过程出现 S, T 当前字符不匹配，则遍历结束，返回 false，若 S，T 都遍历结束，且都能一一匹配，则返回 true。

/// 文字描述一般在不懂逻辑的时候都比较不容易理解，所以请结合图解来加快理解。
/// https://leetcode.cn/problems/backspace-string-compare/solutions/683776/shuang-zhi-zhen-bi-jiao-han-tui-ge-de-zi-8fn8/
pub fn backspace_compare(s: String, t: String) -> bool {
    // 转换为bytes
    let (s, t) = (s.as_bytes(), t.as_bytes());
    // 定义两个尾指针
    let (mut i, mut j) = (s.len(), t.len());

    loop {
        // 从后往前遍历s
        i -= 1;
        // 从后往前遍历t
        j -= 1;
        let mut skip = 0;
        while s.get(i) == Some(&b'#') || skip > 0 {
            if s.get(i) == Some(&b'#') {
                skip += 1;
            } else {
                skip -= 1;
            }
            i -= 1;
        }
        // 遍历完s后重置skip
        skip = 0;
        while t.get(j) == Some(&b'#') || skip > 0 {
            if t.get(j) == Some(&b'#') {
                skip += 1;
            } else {
                skip -= 1;
            }
            j -= 1;
        }
        match (s.get(i), t.get(j)) {
            (Some(a), Some(b)) if a == b => (),
            (None, None) => break true,
            _ => break false,
        }
    }
}

/// 颜色分类
///
/// 给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
///
/// 我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。
///
/// 必须在不使用库内置的 sort 函数的情况下解决这个问题。
///
/// 示例 1：
///
/// 输入：nums = [2,0,2,1,1,0]
///
/// 输出：[0,0,1,1,2,2]
///
/// 示例 2：
///
/// 输入：nums = [2,0,1]
///
/// 输出：[0,1,2]
///
/// 提示：
///
/// n == nums.length
///
/// 1 <= n <= 300
///
/// nums[i] 为 0、1 或 2
///
/// 进阶：
///
/// 你能想出一个仅使用常数空间的一趟扫描算法吗？
pub fn sort_colors(nums: &mut Vec<i32>) {
    if nums.len() < 2 {
        return;
    }
    // 定义循环不变量
    // [0,p0) 区间内的元素都是0
    // [p0,i) 区间内的元素都是1
    // [p2,len-1] 区间内的元素都是2
    // 初始化各个区间的指针
    let (mut p0, mut i, mut p2) = (0, 0, nums.len());
    while i < p2 {
        match nums[i] {
            0 => {
                // 交换数据
                nums.swap(i, p0);
                // 0 需要处于 [0,p0),所有p0需要右移
                p0 += 1;
                // i 需要右移
                i += 1;
            }
            1 => {
                // 1 需要处于 (p0,i),所有i需要右移,p0保持不动就能满足此区间
                i += 1;
            }
            2 => {
                // 2 需要处于 (p2,len-1],所有p2需要左移
                p2 -= 1;
                // 交换数据
                nums.swap(i, p2);
            }
            _ => {}
        }
    }
}

/// 88. 合并两个有序数组
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // 为num1数组定义p1指针，指向 m-1 元素位置
    let mut p1 = m - 1;
    // 为num2数组定义p2指针，指向 n-1 元素位置
    let mut p2 = n - 1;
    // 定义tail指针，指向num1数组的末尾位置
    let mut tail = m + n - 1;
    // 定义当前元素
    let mut cur;
    while p1 >= 0 || p2 >= 0 {
        if p1 == -1 {
            // p1为-1说明m为0，说明nums1数组中没有元素，那么就取num2数组中最后一个元素
            cur = nums2[p2 as usize];
            // 左移p2指针
            p2 -= 1;
        } else if p2 == -1 {
            // p2为-1说明n为0，说明nums2数组中没有元素，那么就取num1数组中最后一个元素
            cur = nums1[p1 as usize];
            // 左移p1指针
            p1 -= 1;
        } else if nums1[p1 as usize] > nums2[p2 as usize] {
            // 说明nums1最后一位元素比nums2最后一位元素大，就取大的
            cur = nums1[p1 as usize];
            // 左移p1指针
            p1 -= 1;
        } else {
            // 说明nums2最后一位元素比nums1最后一位元素大，就取大的
            cur = nums2[p2 as usize];
            // 左移p2指针
            p2 -= 1;
        }
        // 当获取到元素后就放入nums1数组末尾
        nums1[tail as usize] = cur;
        // 左移tail指针
        tail -= 1;
    }
}

/// 1768. 交替合并字符串
pub fn merge_alternately(word1: String, word2: String) -> String {
    // 定义p1指针指向word1字符串起始位置
    let mut p1 = 0;
    // 定义p2指针指向word2字符串起始位置
    let mut p2 = 0;
    // 定义数组存放结果，数组大小为word1.len() + word2.len()
    let mut res = vec![' '; word1.len() + word2.len()];
    // 遍历res数组，i%2==0时取word1字符串中的字符，否则取word2字符串中的字符
    for i in 0..res.len() {
        if i % 2 == 0 {
            //  如果p1小于word1.len()，说明word1字符串还有字符，就取word1字符串中的字符，否则取word2字符串中的字符
            if p1 < word1.len() {
                res[i] = word1.chars().nth(p1).unwrap();
                p1 += 1;
            } else {
                res[i] = word2.chars().nth(p2).unwrap();
                p2 += 1;
            }
        } else {
            // 如果p2小于word2.len()，说明word2字符串还有字符，就取word2字符串中的字符，否则取word1字符串中的字符
            if p2 < word2.len() {
                res[i] = word2.chars().nth(p2).unwrap();
                p2 += 1;
            } else {
                res[i] = word1.chars().nth(p1).unwrap();
                p1 += 1;
            }
        }
    }
    // 将数组转换为字符串
    res.iter().collect()
}

/// 面试题 01.06. 字符串压缩
pub fn compress_string(s: String) -> String {
    // 定义结果字符串
    let mut res = String::new();
    // 定义字符数组
    let chars: Vec<char> = s.chars().collect();
    // 定义字符计数
    let mut count = 1;
    // 遍历字符数组
    for i in 0..chars.len() {
        // 如果当前字符和下一个字符相等，计数加1
        if i < chars.len() - 1 && chars[i] == chars[i + 1] {
            count += 1;
        } else {
            // 如果当前字符和下一个字符不相等，将当前字符和计数加入结果字符串
            res.push(chars[i]);
            res.push_str(&count.to_string());
            // 重置计数
            count = 1;
        }
    }
    // 如果结果字符串长度小于原字符串长度，返回结果字符串，否则返回原字符串
    if res.len() < s.len() {
        res
    } else {
        s
    }
}

/// 392. 判断子序列
pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() && t.is_empty() {
        return true;
    } else if s.is_empty() {
        return true;
    } else if t.is_empty() {
        return false;
    }
    // 把t,s转换为字符数组
    let t = t.chars().collect::<Vec<char>>();
    let s = s.chars().collect::<Vec<char>>();
    // s长度
    let len = s.len();
    // 定义s_p 指针指向s的开始位置
    let mut s_p = 0;
    // 遍历t
    for i in 0..t.len() {
        if (s_p < len) && (t[i] == s[s_p]) {
            s_p += 1;
        }
        if s_p == len {
            return true;
        }
    }
    false
}

/// 350. 两个数组的交集 II
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    // 对nums1数组进行排序
    nums1.sort();
    // 对nums2数组进行排序
    nums2.sort();
    // 定义nums1的p1指针，定义nums2的p2指针
    let mut p1 = 0;
    let mut p2 = 0;
    // 定义结果数组
    let mut ans = Vec::new();
    // 遍历nums1和nums2
    while p1 < nums1.len() && p2 < nums2.len() {
        if nums1[p1] < nums2[p2] {
            p1 += 1;
        } else if nums1[p1] > nums2[p2] {
            p2 += 1;
        } else {
            ans.push(nums1[p1]);
            p1 += 1;
            p2 += 1;
        }
    }
    ans
}

/// 面试题 10.01. 合并排序的数组
pub fn merge_ii(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
    // 为num1数组定义p1指针，指向 m-1 元素位置
    let mut p1 = m - 1;
    // 为num2数组定义p2指针，指向 n-1 元素位置
    let mut p2 = n - 1;
    // 定义tail指针，指向num1数组的末尾位置
    let mut tail = m + n - 1;
    // 定义当前元素
    let mut cur;
    while p1 >= 0 || p2 >= 0 {
        if p1 == -1 {
            // p1为-1说明m为0，说明nums1数组中没有元素，那么就取num2数组中最后一个元素
            cur = b[p2 as usize];
            // 左移p2指针
            p2 -= 1;
        } else if p2 == -1 {
            // p2为-1说明n为0，说明nums2数组中没有元素，那么就取num1数组中最后一个元素
            cur = a[p1 as usize];
            // 左移p1指针
            p1 -= 1;
        } else if a[p1 as usize] > b[p2 as usize] {
            // 说明nums1最后一位元素比nums2最后一位元素大，就取大的
            cur = a[p1 as usize];
            // 左移p1指针
            p1 -= 1;
        } else {
            // 说明nums2最后一位元素比nums1最后一位元素大，就取大的
            cur = b[p2 as usize];
            // 左移p2指针
            p2 -= 1;
        }
        // 当获取到元素后就放入nums1数组末尾
        a[tail as usize] = cur;
        // 左移tail指针
        tail -= 1;
    }
}

/// 1089. 复写零
pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let (mut offset, mut i) = (0, 0);
    // 计算需要偏移多少为0的元素，以及数组需要保留多少个元素
    while i + offset < arr.len() {
        if arr[i] == 0 {
            offset += 1
        }
        i += 1;
    }
    while offset > 0 {
        i -= 1;
        if i + offset < arr.len() {
            // 把需要保留元素的最末尾元素复制到数组末尾的位置，i-1就是需要保留元素的最末尾元素 i+offset就是数组末尾的位置
            arr[i + offset] = arr[i];
        }
        if arr[i] == 0 {
            offset -= 1;
            arr[i + offset] = arr[i];
        }
    }
}

/// 925. 长按键入
pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    if name.is_empty() && typed.is_empty() {
        return true;
    }
    if name.is_empty() || typed.is_empty() {
        return false;
    }
    if typed.len() < name.len() {
        return false;
    }
    let name = name.chars().collect::<Vec<char>>();
    let typed = typed.chars().collect::<Vec<char>>();
    // 定义p1指针指向typed的起始位置
    let mut p1 = 0;
    // 定义p2指针指向name的起始位置
    let mut p2 = 0;
    while p1 < typed.len() {
        if p2 < name.len() && typed[p1] == name[p2] {
            p2 += 1;
            p1 += 1;
        } else if p1 > 0 && p1 < typed.len() && typed[p1] == typed[p1 - 1] {
            p1 += 1;
        } else {
            return false;
        }
    }
    p2 == name.len()
}

/// 922. 按奇偶排序数组 II
pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut j = 1;
    for (i, _) in nums.clone().iter().enumerate().step_by(2) {
        // 如果nums[i]为基数
        if nums[i] % 2 == 1 {
            while nums[j] % 2 == 1 {
                j += 2;
            }
            let temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
        }
    }
    nums
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        println!("{:?}", nums)
    }

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        println!("{:?}", nums1)
    }

    #[test]
    fn test_merge_alternately() {
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();
        let res = merge_alternately(word1, word2);
        println!("{:?}", res)
    }

    #[test]
    fn test_is_subsequence() {
        let s = "b".to_string();
        let t = "abc".to_string();
        let res = is_subsequence(s, t);
        println!("{:?}", res)
    }

    #[test]
    fn test_intersect() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let res = intersect(nums1, nums2);
        println!("{:?}", res)
    }

    #[test]
    fn test_duplicate_zeros() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut arr);
        println!("{:?}", arr)
    }

    #[test]
    fn test_is_long_pressed_name() {
        let name = "leelee".to_string();
        let typed = "lleeelee".to_string();
        assert_eq!(is_long_pressed_name(name, typed), true);
    }

    #[test]
    fn test_sort_array_by_parity_ii() {
        let nums = vec![4, 2, 5, 7];
        let res = sort_array_by_parity_ii(nums);
        println!("{:?}", res)
    }
}
