use crate::list::ListNode;

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
    // 遍历数组，循环不变量为 slow - fast 之间的元素都不是val
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

/// 三数之和
/// 给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a ，b ，c ，使得 a + b + c = 0 ？请找出所有和为 0 且 不重复 的三元组。
///
/// 示例 1：
///
/// 输入：nums = [-1,0,1,2,-1,-4]
/// 输出：[[-1,-1,2],[-1,0,1]]
/// 示例 2：
///
/// 输入：nums = []
/// 输出：[]
/// 示例 3：
///
/// 输入：nums = [0]
/// 输出：[]
///
/// 提示：
///
/// 0 <= nums.length <= 3000
/// -105 <= nums[i] <= 105
/// 解题思路：固定i元素，在通过头和尾双指针从头尾向中间遍历，找到所有符合条件的元素
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    // 数组长度
    let len = nums.len();
    let mut res = vec![];
    if len < 3 {
        return res;
    }
    // 先对数组进行排序
    nums.sort();

    for i in 0..len - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            // 去重
            continue;
        }

        let s = nums[i] + nums[i + 1] + nums[i + 2];
        if s > 0 {
            // 由于数组已经排序，后面无论怎么选，选出的三个数的和不会比 s 还小，所以不会找到比 s 更优的答案了。
            break;
        }
        let s = nums[i] + nums[len - 2] + nums[len - 1];
        if s < 0 {
            // 由于数组已经排序，nums[i] 加上后面任意两个数都不超过 s，所以下面的双指针就不需要跑了，只需要继续取下一个数即可。
            continue;
        }

        // 固定i，寻找first和last，使用双指针法
        let mut first = i + 1;
        // 尾指针
        let mut last = len - 1;
        while first < last {
            if nums[first] + nums[last] + nums[i] < 0 {
                // 小于0，first右移
                first += 1;
            } else if nums[first] + nums[last] + nums[i] > 0 {
                // 大于0，last左移
                last -= 1;
            } else {
                // 等于0，加入结果集
                res.push(vec![nums[i], nums[first], nums[last]]);
                // first和last去重
                while first < last && nums[first] == nums[first + 1] {
                    first += 1;
                }
                while first < last && nums[last] == nums[last - 1] {
                    last -= 1;
                }

                // first和last继续移动
                first += 1;
                last -= 1;
            }
        }
    }

    res
}

/// 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
///
/// 返回这三个数的和。
///
/// 假定每组输入只存在恰好一个解。
///
/// 示例 1：
///
/// 输入：nums = [-1,2,1,-4], target = 1
/// 输出：2
/// 解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
/// 示例 2：
///
/// 输入：nums = [0,0,0], target = 1
/// 输出：0
///
/// 提示：
///
/// 3 <= nums.length <= 1000
/// -1000 <= nums[i] <= 1000
/// -104 <= target <= 104
/// 有两个优化可以让代码击败接近 100%！（和三数之和一样）
///
/// 设 s = nums[i] + nums[i+1] + nums[i+2]。如果 s > target，由于数组已经排序，后面无论怎么选，
/// 选出的三个数的和不会比 s 还小，所以不会找到比 s 更优的答案了。所以只要 s > target，就可以直接 break 外层循环了。
/// 在 break 前判断 s 是否离 target 更近，如果更近，那么更新答案为 s。
///
/// 设 s = nums[i] + nums[n-2] + nums[n-1]。如果 s < target，由于数组已经排序，
/// nums[i] 加上后面任意两个数都不超过 s，所以下面的双指针就不需要跑了，无法找到比 s 更优的答案。
/// 但是后面还有更大的 nums[i]，可能找到一个离 target 更近的三数之和，所以还需要继续枚举，continue 外层循环。
/// 在 continue 前判断 s 是否离 target 更近，如果更近，那么更新答案为 s。
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    // 三数之和
    let mut ans = 0;
    // 最小差值
    let mut min_diff = i32::MAX;
    // 先对数组进行排序
    nums.sort();
    for i in 0..len - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            // 去重
            continue;
        }
        let count = nums[i] + nums[i + 1] + nums[i + 2];
        if count > target {
            // 由于数组已经排序，后面无论怎么选，选出的三个数的和不会比 s 还小，所以不会找到比 s 更优的答案了。
            // 所以只要 s > target，就可以直接 break 外层循环了。
            if count - target < min_diff {
                ans = count;
            }
            break;
        }
        let count = nums[i] + nums[len - 2] + nums[len - 1];
        if count < target {
            // 由于数组已经排序，nums[i] 加上后面任意两个数都不超过 s，所以下面的双指针就不需要跑了，
            // 无法找到比 s 更优的答案。但是后面还有更大的 nums[i]，可能找到一个离 target 更近的三数之和，
            // 所以还需要继续枚举，continue 外层循环。
            if target - count < min_diff {
                min_diff = target - count;
                ans = count;
            }
            continue;
        }

        // 头指针
        let mut left = i + 1;
        // 尾指针
        let mut right = len - 1;
        while left < right {
            let count = nums[i] + nums[left] + nums[right];
            if count == target {
                return count;
            } else if count < target {
                // 小于target，left右移
                left += 1;
            } else {
                // 大于target，right左移
                right -= 1;
            }
            if (target - count).abs() < min_diff {
                min_diff = (target - count).abs();
                ans = count;
            }
        }
    }
    ans
}

/// 四数之和
/// 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a], nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：
///
/// 0 <= a, b, c, d < n
/// a、b、c 和 d 互不相同
/// nums[a] + nums[b] + nums[c] + nums[d] == target
/// 你可以按 任意顺序 返回答案 。
///
/// 示例 1：
///
/// 输入：nums = [1,0,-1,0,-2,2], target = 0
/// 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
/// 示例 2：
///
/// 输入：nums = [2,2,2,2,2], target = 8
/// 输出：[[2,2,2,2]]
///
/// 提示：
///
/// 1 <= nums.length <= 200
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// 思路
/// 四数之和，和15.三数之和是一个思路，都是使用双指针法, 基本解法就是在15.三数之和 的基础上再套一层for循环。
///
/// 但是有一些细节需要注意，例如： 不要判断nums[k] > target 就返回了，三数之和 可以通过 nums[i] > 0 就返回了，因为 0 已经是确定的数了，四数之和这道题目 target是任意值。比如：数组是[-4, -3, -2, -1]，target是-10，不能因为-4 > -10而跳过。但是我们依旧可以去做剪枝，逻辑变成nums[i] > target && (nums[i] >=0 || target >= 0)就可以了。
///
/// 15.三数之和的双指针解法是一层for循环num[i]为确定值，然后循环内有left和right下标作为双指针，找到nums[i] + nums[left] + nums[right] == 0。
///
/// 四数之和的双指针解法是两层for循环nums[k] + nums[i]为确定值，依然是循环内有left和right下标作为双指针，找出nums[k] + nums[i] + nums[left] + nums[right] == target的情况，三数之和的时间复杂度是O(n^2)，四数之和的时间复杂度是O(n^3) 。
///
/// 那么一样的道理，五数之和、六数之和等等都采用这种解法。
///
/// 对于15.三数之和双指针法就是将原本暴力O(n^3)的解法，降为O(n^2)的解法，四数之和的双指针解法就是将原本暴力O(n^4)的解法，降为O(n^3)的解法。
///
/// 之前我们讲过哈希表的经典题目：454.四数相加II，相对于本题简单很多，因为本题是要求在一个集合中找出四个数相加等于target，同时四元组不能重复。
///
/// 而454.四数相加II是四个独立的数组，只要找到A[i] + B[j] + C[k] + D[l] = 0就可以，不用考虑有重复的四个元素相加等于0的情况，所以相对于本题还是简单了不少！
///
/// 我们来回顾一下，几道题目使用了双指针法。
///
/// 双指针法将时间复杂度：O(n^2)的解法优化为 O(n)的解法。也就是降一个数量级，题目如下：
///
/// 27.移除元素
/// 15.三数之和
/// 18.四数之和
/// 链表相关双指针题目：
/// 206.反转链表
/// 19.删除链表的倒数第N个节点
/// 面试题 02.07. 链表相交
/// 142题.环形链表II
/// 双指针法在字符串题目中还有很多应用，后面还会介绍到。
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(len);
    let mut nums = nums;
    nums.sort();
    for k in 0..len {
        // 剪枝, 因为是有序数组，所以如果nums[k] > target，那么后面的元素肯定都大于target，就不用再遍历了
        if nums[k] > target && (nums[k] > 0 || target > 0) {
            break;
        }
        // 去重
        if k > 0 && nums[k] == nums[k - 1] {
            continue;
        }
        for i in (k + 1)..len {
            // 剪枝，因为是有序数组，所以如果nums[k] + nums[i] > target，那么后面的元素肯定都大于target，就不用再遍历了
            if nums[k] + nums[i] > target && (nums[k] + nums[i] >= 0 || target >= 0) {
                break;
            }
            // 去重
            if i > k + 1 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, len - 1);
            while left < right {
                if nums[k] + nums[i] > target - (nums[left] + nums[right]) {
                    right -= 1;
                } else if nums[k] + nums[i] < target - (nums[left] + nums[right]) {
                    left += 1;
                } else {
                    result.push(vec![nums[k], nums[i], nums[left], nums[right]]);
                    // 去重
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }
    }
    result
}

/// 反转字符串
/// https://leetcode.cn/problems/reverse-string/
pub fn reverse_string(s: &mut Vec<char>) {
    // 定义右指针，指向末尾
    let mut rigth = s.len() - 1;
    for left in 0..s.len() / 2 {
        let temp = s[left];
        s[left] = s[rigth];
        s[rigth] = temp;
        // 右指针左移
        rigth -= 1;
    }
}

/// 876. 链表的中间结点
///
/// 给你单链表的头结点 head ，请你找出并返回链表的中间结点。
///
/// 如果有两个中间结点，则返回第二个中间结点。
///
/// 示例 1：
///
/// 输入：head = [1,2,3,4,5]
///
/// 输出：[3,4,5]
///
/// 解释：链表只有一个中间结点，值为 3 。
///
/// 示例 2：
///
/// 输入：head = [1,2,3,4,5,6]
///
/// 输出：[4,5,6]
///
/// 解释：该链表有两个中间结点，值分别为 3 和 4 ，返回第二个结点。
///
/// 提示：
///
/// 链表的结点数范围是 [1, 100]
///
/// 1 <= Node.val <= 100
////
/// 解题思路
///
/// 考虑借助快慢双指针 fast, slow ，「快指针 fast」每轮走 2 步，「慢指针 slow」每轮走 1 步。fast 的步数恒为 slow 的 2 倍，
/// 因此当快指针遍历完链表时，慢指针就指向链表中间节点。而由于长度为偶数的链表有两个中间节点，因此需要分两种情况考虑：
///
/// 链表长度为奇数： 当 fast 走到链表「尾节点」时，slow 正好走到「中间节点」。
///
/// 链表长度为偶数： 当 fast 走到「null」时（越过「尾节点」后），slow 正好走到「第二个中间节点」。
///
/// 总结以上规律，应在当 fast 遇到或越过尾节点 时跳出循环，并返回 slow 即可。
///
/// <img src="https://pic.leetcode-cn.com/1656953441-Kshqch-figures.gif" alt="链表-删除节点" style="zoom:50%;" />
pub fn middle_node(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
    // 快慢指针
    let mut slow = head.clone();
    let mut fast = head;
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = slow.unwrap().next;
        // 快指针走两步，慢指针走一步
        fast = fast.unwrap().next.unwrap().next;
    }
    slow
}

/// 最长连续递增序列
///
/// 给定一个未经排序的整数数组，找到最长且 连续递增的子序列，并返回该序列的长度。
///
/// 连续递增的子序列 可以由两个下标 l 和 r（l < r）确定，如果对于每个 l <= i < r，都有 nums[i] < nums[i + 1] ，
/// 那么子序列 [nums[l], nums[l + 1], ..., nums[r - 1], nums[r]] 就是连续递增子序列。
///
/// 示例 1：
///
/// 输入：nums = [1,3,5,4,7]
///
/// 输出：3
///
/// 解释：最长连续递增序列是 [1,3,5], 长度为3。
///
/// 尽管 [1,3,5,7] 也是升序的子序列, 但它不是连续的，因为 5 和 7 在原数组里被 4 隔开。
///
/// 示例 2：
///
/// 输入：nums = [2,2,2,2,2]
///
/// 输出：1
///
/// 解释：最长连续递增序列是 [2], 长度为1。
///
/// 提示：
///
/// 1 <= nums.length <= 104
///
/// -109 <= nums[i] <= 109
pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    // 定义结果
    let mut max_len = 0;
    // 定义慢指针
    let mut slow = 0;
    // 定义快指针
    let mut fast = 0;
    // 循环不变量是 [slow,fast) 单调递增
    while fast < nums.len() {
        if fast > 0 && nums[fast - 1] >= nums[fast] {
            // 不满足单调递增条件，右移慢指针
            slow = fast;
        }
        // 快指针右移，由于是左闭右开区间，所以这里应该先右移再计算长度
        fast += 1;
        // 统计最大长度
        max_len = max_len.max(fast - slow);
    }
    max_len as i32
}

/// 删除排序数组中的重复项 II
///
/// 给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使得出现次数超过两次的元素只出现两次 ，返回删除后数组的新长度。
///
/// 不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。
///
/// 说明：
///
/// 为什么返回数值是整数，但输出的答案是数组呢？
///
/// 请注意，输入数组是以「引用」方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。
///
/// 你可以想象内部操作如下:
///
/// nums 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
///
/// ```
/// int len = removeDuplicates(nums);
///
/// /// 在函数里修改输入数组对于调用者是可见的。
/// /// 根据你的函数返回的长度, 它会打印出数组中 该长度范围内 的所有元素。
/// for (int i = 0; i < len; i++) {
///     print(nums[i]);
/// }
/// ```
///
/// 示例 1：
///
/// 输入：nums = [1,1,1,2,2,3]
///
/// 输出：5, nums = [1,1,2,2,3]
///
/// 解释：函数应返回新长度 length = 5, 并且原数组的前五个元素被修改为 1, 1, 2, 2, 3。 不需要考虑数组中超出新长度后面的元素。
///
/// 示例 2：
///
/// 输入：nums = [0,0,1,1,1,1,2,3,3]
///
/// 输出：7, nums = [0,0,1,1,2,3,3]
///
/// 解释：函数应返回新长度 length = 7, 并且原数组的前七个元素被修改为 0, 0, 1, 1, 2, 3, 3。不需要考虑数组中超出新长度后面的元素。
///
/// 提示：
///
/// 1 <= nums.length <= 3 * 104
///
/// -104 <= nums[i] <= 104
///
/// nums 已按升序排列
pub fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32 {
    // 定义慢指针
    let mut flow = 0;
    // 遍历数组 , 循环不变量是 [0,len) 区间内的元素是符合条件的
    for fast in 0..nums.len() {
        // 如果慢指针小于2 或者 当前元素不等于慢指针前两个元素
        if flow < 2 || nums[fast] != nums[flow - 2] {
            // 交换数据
            nums[flow] = nums[fast];
            // 慢指针右移
            flow += 1;
        }
    }
    // 返回慢指针的位置
    flow as i32
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

/// 数组中的第 K 个最大元素
///
/// 给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。
///
/// 请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。
///
/// 你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。
///
/// 示例 1:
///
/// ```
/// 输入: [3,2,1,5,6,4], k = 2
/// 输出: 5
/// ```
///
/// 示例 2:
///
/// ```
/// 输入: [3,2,3,1,2,4,5,5,6], k = 4
/// 输出: 4
/// ```
///
/// 提示：
///
/// ```
/// 1 <= k <= nums.length <= 105
/// -104 <= nums[i] <= 104
/// ```
#[allow(unused)]
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    // 目标位置
    let target = len - k as usize;
    // 定义左右指针
    let mut left = 0;
    let mut right = len - 1;
    // 循环不变量是 [left,right] 区间内的元素都是大于等于 pivot 的
    while left < right {
        // 切分元素
        let index = partition(&mut nums, left as i32, right as i32);
        if index as usize == target {
            // 如果切分元素的位置等于目标位置，直接返回
            return nums[index as usize];
        } else if (index as usize) < target {
            // 如果切分元素的位置小于目标位置，左指针右移
            left = index as usize + 1;
        } else {
            // 如果切分元素的位置大于目标位置，右指针左移
            right = index as usize - 1;
        }
    }
    // 返回目标位置的元素
    nums[left]
}

fn partition(arr: &mut Vec<i32>, start: i32, end: i32) -> i32 {
    // 切分元素
    let pivot = arr[start as usize];
    // [start + 1 .. le] <= pivot
    // (le..i] > pivot
    // 注意：一定要设置成 start ，否则交换会出错
    // 把小于等于 pivot 的元素放到左边
    let mut le = start;
    for i in start + 1..=end {
        if arr[i as usize] <= pivot {
            le += 1;
            arr.swap(le as usize, i as usize);
        }
    }
    arr.swap(start as usize, le as usize);
    le
}

#[cfg(test)]
mod tests {
    use crate::list::ListNode;

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

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = three_sum(nums);
        assert_eq!(res, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn test_three_sum_closest() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let res = three_sum_closest(nums, target);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_four_sum() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let result = four_sum(nums, target);
        println!("result = {:?}", result);
    }

    #[test]
    fn test_reverse_string() {
        let mut s = ['h', 'e', 'l', 'l', 'o'].to_vec();
        reverse_string(&mut s);
        println!("{:?}", s)
    }

    #[test]
    fn test_middle_node() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        let res = middle_node(head);
        println!("{:?}", res)
    }

    #[test]
    fn test_find_length_of_lcis() {
        let nums = vec![1, 3, 5, 4, 7];
        let res = find_length_of_lcis(nums);
        println!("{:?}", res)
    }

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        println!("{:?}", nums)
    }
}
