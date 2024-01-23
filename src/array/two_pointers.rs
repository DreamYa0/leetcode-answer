/// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
///
/// 请注意 ，必须在不复制数组的情况下原地对数组进行操作。
///
/// 示例 1:
/// 输入: nums = [0,1,0,3,12]
/// 输出: [1,3,12,0,0]
///
/// 示例 2:
/// 输入: nums = [0]
/// 输出: [0]
///
/// 提示:
/// 1 <= nums.length <= 104
/// -231 <= nums[i] <= 231 - 1
///
/// 进阶：你能尽量减少完成的操作次数吗？
///
/// 可以设置一个指针,就是专业收集不是零的数 收集一遍后,后面的一定是0,就再将空出来的位置设置为0,就解决问题了
pub fn move_zeroes(nums: &mut Vec<i32>) {
    // 定义一个指针来记录非零元素的位置
    let mut slow = 0;
    // 开始收集不是零的数
    // [0,1,0,3,12]
    // --> [1,0,0,3,12]
    // --> [1,3,0,0,12]
    // --> [1,3,12,0,0]
    for fast in 0..nums.len() {
        if nums[fast] != 0 {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }

    // 对num中temp位置之前的数据进行排序
    nums[..slow].sort();

    // 收集完毕后,后面自然就都是0了
    for i in slow..nums.len() {
        nums[i] = 0;
    }
}

/// 双指针法
///
/// 双指针法（快慢指针法）： 通过一个快指针和慢指针在一个for循环下完成两个for循环的工作。
///
/// 定义快慢指针
///
/// 快指针：寻找新数组的元素 ，新数组就是不含有目标元素的数组
/// 慢指针：指向更新 新数组下标的位置
/// 很多同学这道题目做的很懵，就是不理解 快慢指针究竟都是什么含义，所以一定要明确含义，后面的思路就更容易理解了。
///
/// 很多同学不了解
/// 双指针法（快慢指针法）在数组和链表的操作中是非常常见的，很多考察数组、链表、字符串等操作的面试题，都使用双指针法。
/// 删除过程如下：
/// <img class="marble" src="https://code-thinking.cdn.bcebos.com/gifs/27.%E7%A7%BB%E9%99%A4%E5%85%83%E7%B4%A0-%E5%8F%8C%E6%8C%87%E9%92%88%E6%B3%95.gif" alt="image.png" style="zoom:50%;" />
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // 定义一个指针用来存放非val的值
    let mut slow = 0;
    // 遍历数组
    for fast in 0..nums.len() {
        if nums[fast] != val {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    // 返回temp位置的索引就是数组的长度
    slow as i32
}

/// 给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。
///
/// 考虑 nums 的唯一元素的数量为 k ，你需要做以下事情确保你的题解可以被通过：
///
/// 更改数组 nums ，使 nums 的前 k 个元素包含唯一元素，并按照它们最初在 nums 中出现的顺序排列。nums 的其余元素与 nums 的大小不重要。
/// 返回 k 。
///
/// 示例 1：
///
/// 输入：nums = [1,1,2]
/// 输出：2, nums = [1,2,_]
/// 解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。
/// 示例 2：
///
/// 输入：nums = [0,0,1,1,1,2,2,3,3,4]
/// 输出：5, nums = [0,1,2,3,4]
/// 解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。
///
///
/// 提示：
///
/// 1 <= nums.length <= 3 * 104
/// -104 <= nums[i] <= 104
/// nums 已按 非严格递增 排列
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // 定义一个指针用来存放非val的值
    let mut slow = 0;
    // 遍历数组
    for fast in 0..nums.len() {
        // 如果快指针和慢指针的值不相等，就将快指针的值赋值给慢指针
        if nums[fast] != nums[slow] {
            // 慢指针右移
            slow += 1;
            // 将快指针的值赋值给慢指针
            nums[slow] = nums[fast];
        }
    }
    // 返回slow位置的索引+1就是数组的长度
    slow as i32 + 1
}

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

/// 977. 有序数组的平方
/// 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
///
/// 示例 1：
///
/// 输入：nums = [-4,-1,0,3,10]
/// 输出：[0,1,9,16,100]
/// 解释：平方后，数组变为 [16,1,0,9,100]
/// 排序后，数组变为 [0,1,9,16,100]
/// 示例 2：
///
/// 输入：nums = [-7,-3,2,3,11]
/// 输出：[4,9,9,49,121]
///
///
/// 提示：
///
/// 1 <= nums.length <= 104
/// -104 <= nums[i] <= 104
/// nums 已按 非递减顺序 排序
///
///
/// 进阶：
///
/// 请你设计时间复杂度为 O(n) 的算法解决本问题
///
/// 双指针法
/// 数组其实是有序的， 只不过负数平方之后可能成为最大数了。
///
/// 那么数组平方的最大值就在数组的两端，不是最左边就是最右边，不可能是中间。
///
/// 此时可以考虑双指针法了，i指向起始位置，j指向终止位置。
///
/// 定义一个新数组result，和A数组一样的大小，让k指向result数组终止位置。
///
/// 如果A[i] * A[i] < A[j] * A[j] 那么result[k--] = A[j] * A[j]; 。
///
/// 如果A[i] * A[i] >= A[j] * A[j] 那么result[k--] = A[i] * A[i]; 。
///
/// 如动画所示：
/// <img class="marble" src="https://code-thinking.cdn.bcebos.com/gifs/977.%E6%9C%89%E5%BA%8F%E6%95%B0%E7%BB%84%E7%9A%84%E5%B9%B3%E6%96%B9.gif" alt="image.png" style="zoom:50%;" />
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    // 定义一个结果数组
    let mut res = vec![0; nums.len()];
    // 定义两个指针
    let mut k = nums.len() - 1;
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = nums.len() - 1;
    while left <= right {
        if nums[left] * nums[left] < nums[right] * nums[right] {
            res[k] = nums[right] * nums[right];
            // 右指针左移
            right -= 1;
        } else {
            res[k] = nums[left] * nums[left];
            // 左指针右移
            left += 1;
        }
        // 结果数组的指针左移
        k -= 1;
    }
    res
}

/// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
/// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
/// 例如，121 是回文，而 123 不是。
///
/// 示例 1：
/// 输入：x = 121
/// 输出：true
///
/// 示例 2：
/// 输入：x = -121
/// 输出：false
/// 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
///
/// 示例 3：
/// 输入：x = 10
/// 输出：false
/// 解释：从右向左读, 为 01 。因此它不是一个回文数。
///
/// 提示：
/// -231 <= x <= 231 - 1
/// 进阶：你能不将整数转为字符串来解决这个问题吗？
/// 解题思路：使用双指针法
pub fn is_palindrome(x: i32) -> bool {
    let mut bys = x.to_string().into_bytes();
    // 定义右指针
    let mut right = bys.len() - 1;
    // 左右指针往中间 bys.len() / 2 位置移动
    for left in 0..bys.len() / 2 {
        // 交换左右指针的值
        let temp = bys[left];
        bys[left] = bys[right];
        bys[right] = temp;
        // 右指针左移
        right -= 1;
    }
    // 将字节数组转换为字符串
    let s = String::from_utf8(bys).unwrap();
    s == x.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 12, 3];
        move_zeroes(&mut nums);
        println!("{:?}", nums)
    }

    #[test]
    fn test_remove_element() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let val = 0;
        let len = remove_element(&mut nums, val);
        println!("{:?}", len);
    }

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let len = remove_duplicates(&mut nums);
        println!("{:?}", nums);
        println!("{:?}", len);
    }

    #[test]
    fn test_sorted_squares() {
        let nums = vec![-4, -1, 0, 3, 10];
        let res = sorted_squares(nums);
        println!("{:?}", res);
    }

    #[test]
    fn test_is_palindrome() {
        println!("{:?}", is_palindrome(-121))
    }
}
