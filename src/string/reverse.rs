/// 541. 反转字符串II
///
/// https://leetcode.cn/problems/reverse-string-ii/
///
/// 思路
/// 这道题目其实也是模拟，实现题目中规定的反转规则就可以了。
///
/// 一些同学可能为了处理逻辑：每隔2k个字符的前k的字符，写了一堆逻辑代码或者再搞一个计数器，来统计2k，再统计前k个字符。
///
/// 其实在遍历字符串的过程中，只要让 i += (2 * k)，i 每次移动 2 * k 就可以了，然后判断是否需要有反转的区间。
///
/// 因为要找的也就是每2 * k 区间的起点，这样写，程序会高效很多。
///
/// 所以当需要固定规律一段一段去处理字符串的时候，要想想在在for循环的表达式上做做文章。
pub fn reverse_str(s: String, k: i32) -> String {
    let len = s.len();
    let k = k as usize;
    let mut s = s.chars().collect::<Vec<_>>();
    // 从0遍历到字符串长度，且步长为 2 * k
    for i in (0..len).step_by(2 * k) {
        if i + k < len {
            reverse(&mut s, i, i + k - 1);
        } else {
            reverse(&mut s, i, len - 1);
        }
    }
    s.iter().collect::<String>()
}

/// 对范围内的数组内容进行反转
fn reverse(s: &mut Vec<char>, mut begin: usize, mut end: usize) {
    // 反转begin到end之间的字符
    while begin < end {
        let temp = s[begin];
        s[begin] = s[end];
        s[end] = temp;
        begin += 1;
        end -= 1;
    }
}

/// 151. 反转字符串中的单词
/// 给你一个字符串 s ，请你反转字符串中 单词 的顺序。
///
/// 单词 是由非空格字符组成的字符串。s 中使用至少一个空格将字符串中的 单词 分隔开。
///
/// 返回 单词 顺序颠倒且 单词 之间用单个空格连接的结果字符串。
///
/// 注意：输入字符串 s中可能会存在前导空格、尾随空格或者单词间的多个空格。返回的结果字符串中，单词间应当仅用单个空格分隔，且不包含任何额外的空格。
///
///
/// 示例 1：
///
/// 输入：s = "the sky is blue"
/// 输出："blue is sky the"
/// 示例 2：
///
/// 输入：s = "  hello world  "
/// 输出："world hello"
/// 解释：反转后的字符串中不能存在前导空格和尾随空格。
/// 示例 3：
///
/// 输入：s = "a good   example"
/// 输出："example good a"
/// 解释：如果两个单词间有多余的空格，反转后的字符串需要将单词间的空格减少到仅有一个。
///
/// 提示：
///
/// 1 <= s.length <= 104
/// s 包含英文大小写字母、数字和空格 ' '
/// s 中 至少存在一个 单词
///
/// 进阶：如果字符串在你使用的编程语言中是一种可变数据类型，请尝试使用 O(1) 额外空间复杂度的 原地 解法。
///
/// 思路
/// 这道题目可以说是综合考察了字符串的多种操作。
///
/// 一些同学会使用split库函数，分隔单词，然后定义一个新的string字符串，最后再把单词倒序相加，那么这道题题目就是一道水题了，失去了它的意义。
///
/// 所以这里我还是提高一下本题的难度：不要使用辅助空间，空间复杂度要求为O(1)。
///
/// 不能使用辅助空间之后，那么只能在原字符串上下功夫了。
///
/// 想一下，我们将整个字符串都反转过来，那么单词的顺序指定是倒序了，只不过单词本身也倒序了，那么再把单词反转一下，单词不就正过来了。
///
/// 所以解题思路如下：
///
/// 移除多余空格
/// 将整个字符串反转
/// 将每个单词反转
/// 举个例子，源字符串为："the sky is blue "
///
/// 移除多余空格 : "the sky is blue"
/// 字符串反转："eulb si yks eht"
/// 单词反转："blue is sky the"
/// 这样我们就完成了翻转字符串里的单词。
pub fn reverse_words(s: String) -> String {
    let mut s = s.chars().collect::<Vec<char>>();
    // 去除空格
    remove_extra_spaces(s.as_mut());
    let len = s.len();
    // 反转整个字符串
    reverse(&mut s, 0, len - 1);

    // 起始位置
    let mut start = 0;
    for i in 0..=len {
        // 字符串末尾没有空格所以判断条件是i==len
        if i == len || s[i].is_ascii_whitespace() {
            reverse(&mut s, start, i - 1);
            // 移动start指针到i+1的位置
            start = i + 1;
        }
    }

    s.iter().collect::<String>()
}

/// 去除首尾和中间多余的空格
fn remove_extra_spaces(s: &mut Vec<char>) {
    // 定义慢指针
    let mut slow = 0;
    // 字符串长度
    let len = s.len();
    // 定义快指针
    let mut fast = 0;
    // 快指针控制整个字符串的遍历，注意这里不能用for循环，不然在里面那个while循环中对i的递增会失效
    while fast < len {
        if !s[fast].is_ascii_whitespace() {
            // 如果slow不等于0的时候说明不是第一个单词，所以需要再单词开头处插入空格
            if slow != 0 {
                s[slow] = ' ';
                // 右移动slow指针
                slow += 1;
            }

            // 此循环是用来操作字符串中的单词的，遇到空格就表示一个单词操作完成，循环结束
            while fast < len && !s[fast].is_ascii_whitespace() {
                // 如果遇到非空格，就把字符移动到slow指针的位置
                s[slow] = s[fast];
                // 右移动slow指针
                slow += 1;
                // 右移动fast指针
                fast += 1;
            }
        } else {
            // 如果遇到了空格就右移动fast指针
            fast += 1;
        }
    }

    s.resize(slow, ' ');
}

/// 右旋字符串
///
/// 字符串的右旋转操作是把字符串尾部的若干个字符转移到字符串的前面。给定一个字符串 s 和一个正整数 k，请编写一个函数，将字符串中的后面 k 个字符移到字符串的前面，实现字符串的右旋转操作。
///
/// 例如，对于输入字符串 "abcdefg" 和整数 2，函数应该将其转换为 "fgabcde"。
///
/// 输入：输入共包含两行，第一行为一个正整数 k，代表右旋转的位数。第二行为字符串 s，代表需要旋转的字符串。
///
/// 输出：输出共一行，为进行了右旋转操作后的字符串。
///
/// 样例输入：
///
/// 2
/// abcdefg
/// 样例输出：
///
/// fgabcde
/// 数据范围：1 <= k < 10000, 1 <= s.length < 10000;
///
/// #思路
/// 为了让本题更有意义，提升一下本题难度：不能申请额外空间，只能在本串上操作。 （Java不能在字符串上修改，所以使用java一定要开辟新空间）
///
/// 不能使用额外空间的话，模拟在本串操作要实现右旋转字符串的功能还是有点困难的。
///
/// 那么我们可以想一下上一题目字符串：花式反转还不够！ (opens new window)中讲过，使用整体反转+局部反转就可以实现反转单词顺序的目的。
///
/// 本题中，我们需要将字符串右移n位，字符串相当于分成了两个部分，如果n为2，符串相当于分成了两个部分，如图： （length为字符串长度）
///
///
///
/// 右移n位， 就是将第二段放在前面，第一段放在后面，先不考虑里面字符的顺序，是不是整体倒叙不就行了。如图：
///
///
///
/// 此时第一段和第二段的顺序是我们想要的，但里面的字符位置被我们倒叙，那么此时我们在把 第一段和第二段里面的字符再倒叙一把，这样字符顺序不就正确了。 如果：
///
///
///
/// 其实，思路就是 通过 整体倒叙，把两段子串顺序颠倒，两个段子串里的的字符在倒叙一把，负负得正，这样就不影响子串里面字符的顺序了。
///
/// 整体代码如下：
///
/// // 版本一
/// #include<iostream>
/// #include<algorithm>
/// using namespace std;
/// int main() {
///     int n;
///     string s;
///     cin >> n;
///     cin >> s;
///     int len = s.size(); //获取长度
///
///     reverse(s.begin(), s.end()); // 整体反转
///     reverse(s.begin(), s.begin() + n); // 先反转前一段，长度n
///     reverse(s.begin() + n, s.end()); // 再反转后一段
///
///     cout << s << endl;
///
/// }
/// 那么整体反正的操作放在下面，先局部反转行不行？
///
/// 可以的，不过，要记得 控制好 局部反转的长度，如果先局部反转，那么先反转的子串长度就是 len - n，如图：
///
///
///
/// 代码如下：
///
/// // 版本二
/// #include<iostream>
/// #include<algorithm>
/// using namespace std;
/// int main() {
///     int n;
///     string s;
///     cin >> n;
///     cin >> s;
///     int len = s.size(); //获取长度
///     reverse(s.begin(), s.begin() + len - n); // 先反转前一段，长度len-n ，注意这里是和版本一的区别
///     reverse(s.begin() + len - n, s.end()); // 再反转后一段
///     reverse(s.begin(), s.end()); // 整体反转
///     cout << s << endl;
///
/// }
/// #拓展
/// 大家在做剑指offer的时候，会发现 剑指offer的题目是左反转，那么左反转和右反转 有什么区别呢？
///
/// 其实思路是一样一样的，就是反转的区间不同而已。如果本题是左旋转n，那么实现代码如下：
///
/// #include<iostream>
/// #include<algorithm>
/// using namespace std;
/// int main() {
///     int n;
///     string s;
///     cin >> n;
///     cin >> s;
///     int len = s.size(); //获取长度
///     reverse(s.begin(), s.begin() + n); //  反转第一段长度为n
///     reverse(s.begin() + n, s.end()); // 反转第二段长度为len-n
///     reverse(s.begin(), s.end());  // 整体反转
///     cout << s << endl;
///
/// }
/// 大家可以感受一下 这份代码和 版本二的区别， 其实就是反转的区间不同而已。
///
/// 那么左旋转的话，可以不可以先整体反转，例如想版本一的那样呢？
///
/// 当然可以。
pub fn rotate_right(s: String, k: i32) -> String {
    let mut s = s.chars().collect::<Vec<char>>();
    let len = s.len();
    reverse(&mut s, 0, len - 1);
    reverse(&mut s, 0, k as usize - 1);
    reverse(&mut s, k as usize, len - 1);
    s.iter().collect::<String>()
}

/// LCR 182. 动态口令
pub fn dynamic_password(password: String, target: i32) -> String {
    let len = password.len();
    let k = target as usize;
    let mut s = password.chars().collect::<Vec<_>>();
    // 先翻转整个字符数组
    reverse(&mut s, 0, len - 1);
    // 再翻转 0 -- (len - target - 1)
    reverse(&mut s, 0, len - k - 1);
    // 再翻转 (len - target) -- len - 1
    reverse(&mut s, len - k, len - 1);
    s.iter().collect()
}

/// 557. 反转字符串中的单词 III
pub fn reverse_words_iii(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let len = s.len();
    // 定义起始位置
    let mut start = 0;
    for i in 0..=len {
        if i == len || s[i].is_ascii_whitespace() {
            // 如果遇到空格或者字符串末尾，就反转 start -- i - 1
            reverse(&mut s, start, i - 1);
            start = i + 1;
        }
    }
    s.iter().collect::<String>()
}

/// LCR 181. 字符串中的单词反转
pub fn reverse_message(message: String) -> String {
    let mut s = message.chars().collect::<Vec<char>>();
    // 去除空格
    remove_extra_spaces(s.as_mut());
    if s.is_empty() {
        return "".to_string();
    }
    let len = s.len();
    // 先反转整个字符串
    reverse(&mut s, 0, len - 1);
    let mut start = 0;
    for i in 0..=len {
        if (i == len || s[i].is_ascii_whitespace()) && i > 0 {
            // 再反转每个单词
            reverse(&mut s, start, i - 1);
            start = i + 1;
        }
    }
    s.iter().collect()
}

/// 345. 反转字符串中的元音字母
pub fn reverse_vowels(s: String) -> String {
    // 定义元音字母
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut s = s.chars().collect::<Vec<char>>();
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = s.len() - 1;
    while left < right {
        if !vowels.contains(&s[left]) {
            left += 1;
        } else if !vowels.contains(&s[right]) {
            right -= 1;
        } else {
            let temp = s[left];
            s[left] = s[right];
            s[right] = temp;
            left += 1;
            right -= 1;
        }
    }
    s.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let s = "  hello world  ".to_string();
        let reverse_words = reverse_words(s);
        println!("{:?}", reverse_words);
    }

    #[test]
    fn test_rotate_right() {
        let rotate_right = rotate_right("abcdefg".to_string(), 2);
        println!("{:?}", rotate_right)
    }

    #[test]
    fn test_dynamic_password() {
        let password = "s3cur1tyC0d3".to_string();
        let target = 4;
        let res = dynamic_password(password, target);
        println!("{:?}", res)
    }

    #[test]
    fn test_reverse_words_iii() {
        let s = "Let's take LeetCode contest".to_string();
        let res = reverse_words_iii(s);
        println!("{:?}", res)
    }

    #[test]
    fn test_reverse_message() {
        let message = "a good   example".to_string();
        let res = reverse_message(message);
        println!("{:?}", res)
    }

    #[test]
    fn test_reverse_vowels() {
        let s = "hello".to_string();
        let res = reverse_vowels(s);
        println!("{:?}", res)
    }
}
