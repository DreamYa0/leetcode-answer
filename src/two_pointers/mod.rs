pub mod fast_slow_pointer;
pub mod left_right_pointer;
pub mod palindrome_str;
pub mod rev_str;

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

/// 75. 颜色分类
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

/**
 * 88. 合并两个有序数组

提示
给你两个按 非递减顺序 排列的整数数组 nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。

请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。

注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。为了应对这种情况，nums1 的初始长度为 m + n，其中前 m 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。



示例 1：

输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
输出：[1,2,2,3,5,6]
解释：需要合并 [1,2,3] 和 [2,5,6] 。
合并结果是 [1,2,2,3,5,6] ，其中斜体加粗标注的为 nums1 中的元素。
示例 2：

输入：nums1 = [1], m = 1, nums2 = [], n = 0
输出：[1]
解释：需要合并 [1] 和 [] 。
合并结果是 [1] 。
示例 3：

输入：nums1 = [0], m = 0, nums2 = [1], n = 1
输出：[1]
解释：需要合并的数组是 [] 和 [1] 。
合并结果是 [1] 。
注意，因为 m = 0 ，所以 nums1 中没有元素。nums1 中仅存的 0 仅仅是为了确保合并结果可以顺利存放到 nums1 中。


提示：

nums1.length == m + n
nums2.length == n
0 <= m, n <= 200
1 <= m + n <= 200
-109 <= nums1[i], nums2[j] <= 109
 */
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

/**
 * 1768. 交替合并字符串

提示
给你两个字符串 word1 和 word2 。请你从 word1 开始，通过交替添加字母来合并字符串。
如果一个字符串比另一个字符串长，就将多出来的字母追加到合并后字符串的末尾。

返回 合并后的字符串 。



示例 1：

输入：word1 = "abc", word2 = "pqr"
输出："apbqcr"
解释：字符串合并情况如下所示：
word1：  a   b   c
word2：    p   q   r
合并后：  a p b q c r
示例 2：

输入：word1 = "ab", word2 = "pqrs"
输出："apbqrs"
解释：注意，word2 比 word1 长，"rs" 需要追加到合并后字符串的末尾。
word1：  a   b
word2：    p   q   r   s
合并后：  a p b q   r   s
示例 3：

输入：word1 = "abcd", word2 = "pq"
输出："apbqcd"
解释：注意，word1 比 word2 长，"cd" 需要追加到合并后字符串的末尾。
word1：  a   b   c   d
word2：    p   q
合并后：  a p b q c   d


提示：

1 <= word1.length, word2.length <= 100
word1 和 word2 由小写英文字母组成
 */
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

/**
 * 面试题 01.06. 字符串压缩

提示
字符串压缩。利用字符重复出现的次数，编写一种方法，实现基本的字符串压缩功能。
比如，字符串aabcccccaaa会变为a2b1c5a3。若“压缩”后的字符串没有变短，则返回原先的字符串。
你可以假设字符串中只包含大小写英文字母（a至z）。

示例1:

 输入："aabcccccaaa"
 输出："a2b1c5a3"
示例2:

 输入："abbccd"
 输出："abbccd"
 解释："abbccd"压缩后为"a1b2c2d1"，比原字符串长度更长。
提示：

字符串长度在[0, 50000]范围内。
 */
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

/**
 * 392. 判断子序列

给定字符串 s 和 t ，判断 s 是否为 t 的子序列。

字符串的一个子序列是原始字符串删除一些（也可以不删除）字符而不改变剩余字符相对位置形成的新字符串。
（例如，"ace"是"abcde"的一个子序列，而"aec"不是）。

进阶：

如果有大量输入的 S，称作 S1, S2, ... , Sk 其中 k >= 10亿，你需要依次检查它们是否为 T 的子序列。
在这种情况下，你会怎样改变代码？

致谢：

特别感谢 @pbrother 添加此问题并且创建所有测试用例。



示例 1：

输入：s = "abc", t = "ahbgdc"
输出：true
示例 2：

输入：s = "axc", t = "ahbgdc"
输出：false


提示：

0 <= s.length <= 100
0 <= t.length <= 10^4
两个字符串都只由小写字符组成。
 */
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

/**
 * 350. 两个数组的交集 II

给你两个整数数组 nums1 和 nums2 ，请你以数组形式返回两数组的交集。
返回结果中每个元素出现的次数，应与元素在两个数组中都出现的次数一致（如果出现次数不一致，则考虑取较小值）。
可以不考虑输出结果的顺序。



示例 1：

输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2,2]
示例 2:

输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[4,9]


提示：

1 <= nums1.length, nums2.length <= 1000
0 <= nums1[i], nums2[i] <= 1000


进阶：

如果给定的数组已经排好序呢？你将如何优化你的算法？
如果 nums1 的大小比 nums2 小，哪种方法更优？
如果 nums2 的元素存储在磁盘上，内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？
 */
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

/**
 * 面试题 10.01. 合并排序的数组

提示
给定两个排序后的数组 A 和 B，其中 A 的末端有足够的缓冲空间容纳 B。 编写一个方法，将 B 合并入 A 并排序。

初始化 A 和 B 的元素数量分别为 m 和 n。

示例:

输入:
A = [1,2,3,0,0,0], m = 3
B = [2,5,6],       n = 3

输出: [1,2,2,3,5,6]
说明:

A.length == n + m
 */
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

/**
 * 1089. 复写零

提示
给你一个长度固定的整数数组 arr ，请你将该数组中出现的每个零都复写一遍，并将其余的元素向右平移。

注意：请不要在超过该数组长度的位置写入元素。请对输入的数组 就地 进行上述修改，不要从函数返回任何东西。



示例 1：

输入：arr = [1,0,2,3,0,4,5,0]
输出：[1,0,0,2,3,0,0,4]
解释：调用函数后，输入的数组将被修改为：[1,0,0,2,3,0,0,4]
示例 2：

输入：arr = [1,2,3]
输出：[1,2,3]
解释：调用函数后，输入的数组将被修改为：[1,2,3]


提示：

1 <= arr.length <= 104
0 <= arr[i] <= 9
 */
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

/**
 * 925. 长按键入

你的朋友正在使用键盘输入他的名字 name。偶尔，在键入字符 c 时，按键可能会被长按，而字符可能被输入 1 次或多次。

你将会检查键盘输入的字符 typed。如果它对应的可能是你的朋友的名字（其中一些字符可能被长按），那么就返回 True。



示例 1：

输入：name = "alex", typed = "aaleex"
输出：true
解释：'alex' 中的 'a' 和 'e' 被长按。
示例 2：

输入：name = "saeed", typed = "ssaaedd"
输出：false
解释：'e' 一定需要被键入两次，但在 typed 的输出中不是这样。


提示：

1 <= name.length, typed.length <= 1000
name 和 typed 的字符都是小写字母
 */
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

/**
 * 922. 按奇偶排序数组 II

给定一个非负整数数组 nums，  nums 中一半整数是 奇数 ，一半整数是 偶数 。

对数组进行排序，以便当 nums[i] 为奇数时，i 也是 奇数 ；当 nums[i] 为偶数时， i 也是 偶数 。

你可以返回 任何满足上述条件的数组作为答案 。



示例 1：

输入：nums = [4,2,5,7]
输出：[4,5,2,7]
解释：[4,7,2,5]，[2,5,4,7]，[2,7,4,5] 也会被接受。
示例 2：

输入：nums = [2,3]
输出：[2,3]


提示：

2 <= nums.length <= 2 * 104
nums.length 是偶数
nums 中一半是偶数
0 <= nums[i] <= 1000


进阶：可以不使用额外空间解决问题吗？
 */
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

/**
 * 524. 通过删除字母匹配到字典里最长单词

给你一个字符串 s 和一个字符串数组 dictionary ，找出并返回 dictionary 中最长的字符串，该字符串可以通过删除 s 中的某些字符得到。

如果答案不止一个，返回长度最长且字母序最小的字符串。如果答案不存在，则返回空字符串。



示例 1：

输入：s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
输出："apple"
示例 2：

输入：s = "abpcplea", dictionary = ["a","b","c"]
输出："a"


提示：

1 <= s.length <= 1000
1 <= dictionary.length <= 1000
1 <= dictionary[i].length <= 1000
s 和 dictionary[i] 仅由小写英文字母组成
 */
pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
    let mut res = "".to_string();
    for dic in dictionary {
        let dic = dic.chars().collect::<Vec<char>>();
        let s = s.chars().collect::<Vec<char>>();
        // 定义p1指针指向dic的开始位置
        let mut p1 = 0;
        // 定义p2指针指向s的开始位置
        let mut p2 = 0;
        while p1 < dic.len() && p2 < s.len() {
            if dic[p1] == s[p2] {
                // 如果匹配上了，p1指针右移，准备匹配下一个字符
                p1 += 1;
            }
            // 无论匹配上没有，p2指针都右移来准备下一个字符匹配
            p2 += 1;
        }
        // 说明dic中的字符都匹配到了
        if p1 == dic.len() {
            // 保留长度最长或长度相同时字典序最小的字符串
            let dic = dic.iter().collect::<String>();
            // dic < res 比较字典序大小
            if dic.len() > res.len() || (dic.len() == res.len() && dic < res) {
                res = dic;
            }
        }
    }
    res
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

    #[test]
    fn test_find_longest_word() {
        let dictionary = vec![
            "ale".to_string(),
            "apple".to_string(),
            "monkey".to_string(),
            "plea".to_string(),
        ];
        let s = "abpcplea".to_string();
        let find_longest_word = find_longest_word(s, dictionary);
        assert_eq!(find_longest_word, "apple")
    }
}
