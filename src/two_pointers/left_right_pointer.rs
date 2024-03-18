use std::collections::HashSet;

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

/// 15. 三数之和
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

        // 固定i，寻找left和right，使用双指针法
        let mut left = i + 1;
        // 尾指针
        let mut right = len - 1;
        while left < right {
            if nums[left] + nums[right] + nums[i] < 0 {
                // 小于0，first右移
                left += 1;
            } else if nums[left] + nums[right] + nums[i] > 0 {
                // 大于0，last左移
                right -= 1;
            } else {
                // 等于0，加入结果集
                res.push(vec![nums[i], nums[left], nums[right]]);
                // first和last去重
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                // first和last继续移动
                left += 1;
                right -= 1;
            }
        }
    }

    res
}

/// 16. 最接近的三数之和
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

/// 18. 四数之和
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

/**
 * 2562. 找出数组的串联值

提示
给你一个下标从 0 开始的整数数组 nums 。

现定义两个数字的 串联 是由这两个数值串联起来形成的新数字。

例如，15 和 49 的串联是 1549 。
nums 的 串联值 最初等于 0 。执行下述操作直到 nums 变为空：

如果 nums 中存在不止一个数字，分别选中 nums 中的第一个元素和最后一个元素，将二者串联得到的值加到 nums 的 串联值 上，
然后从 nums 中删除第一个和最后一个元素。
如果仅存在一个元素，则将该元素的值加到 nums 的串联值上，然后删除这个元素。
返回执行完所有操作后 nums 的串联值。



示例 1：

输入：nums = [7,52,2,4]
输出：596
解释：在执行任一步操作前，nums 为 [7,52,2,4] ，串联值为 0 。
 - 在第一步操作中：
我们选中第一个元素 7 和最后一个元素 4 。
二者的串联是 74 ，将其加到串联值上，所以串联值等于 74 。
接着我们从 nums 中移除这两个元素，所以 nums 变为 [52,2] 。
 - 在第二步操作中：
我们选中第一个元素 52 和最后一个元素 2 。
二者的串联是 522 ，将其加到串联值上，所以串联值等于 596 。
接着我们从 nums 中移除这两个元素，所以 nums 变为空。
由于串联值等于 596 ，所以答案就是 596 。
示例 2：

输入：nums = [5,14,13,8,12]
输出：673
解释：在执行任一步操作前，nums 为 [5,14,13,8,12] ，串联值为 0 。
- 在第一步操作中：
我们选中第一个元素 5 和最后一个元素 12 。
二者的串联是 512 ，将其加到串联值上，所以串联值等于 512 。
接着我们从 nums 中移除这两个元素，所以 nums 变为 [14,13,8] 。
- 在第二步操作中：
我们选中第一个元素 14 和最后一个元素 8 。
二者的串联是 148 ，将其加到串联值上，所以串联值等于 660 。
接着我们从 nums 中移除这两个元素，所以 nums 变为 [13] 。
- 在第三步操作中：
nums 只有一个元素，所以我们选中 13 并将其加到串联值上，所以串联值等于 673 。
接着我们从 nums 中移除这个元素，所以 nums 变为空。
由于串联值等于 673 ，所以答案就是 673 。


提示：

1 <= nums.length <= 1000
1 <= nums[i] <= 104
 */
pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = nums.len() - 1;
    // 定义串联值
    let mut sum = 0;
    // 遍历数组
    while left < right {
        // 取左指针的值
        let top = nums[left];
        // 取右指针的值
        let last = nums[right];
        // 将左指针的值和右指针的值拼接成字符串，然后转换为i64类型
        let parse = (top.to_string() + &last.to_string())
            .parse::<i64>()
            .unwrap();
        // 然后累加到sum中
        sum += parse;
        // 左指针右移
        left += 1;
        // 右指针左移
        right -= 1;
    }
    // 如果左指针等于右指针，说明数组长度为奇数，那么就取中间值
    if left == right {
        sum += nums[left] as i64;
    }
    sum
}

/**
 * 2465. 不同的平均值数目
简单
相关标签
相关企业
提示
给你一个下标从 0 开始长度为 偶数 的整数数组 nums 。

只要 nums 不是 空数组，你就重复执行以下步骤：

找到 nums 中的最小值，并删除它。
找到 nums 中的最大值，并删除它。
计算删除两数的平均值。
两数 a 和 b 的 平均值 为 (a + b) / 2 。

比方说，2 和 3 的平均值是 (2 + 3) / 2 = 2.5 。
返回上述过程能得到的 不同 平均值的数目。

注意 ，如果最小值或者最大值有重复元素，可以删除任意一个。



示例 1：

输入：nums = [4,1,4,0,3,5]
输出：2
解释：
1. 删除 0 和 5 ，平均值是 (0 + 5) / 2 = 2.5 ，现在 nums = [4,1,4,3] 。
2. 删除 1 和 4 ，平均值是 (1 + 4) / 2 = 2.5 ，现在 nums = [4,3] 。
3. 删除 3 和 4 ，平均值是 (3 + 4) / 2 = 3.5 。
2.5 ，2.5 和 3.5 之中总共有 2 个不同的数，我们返回 2 。
示例 2：

输入：nums = [1,100]
输出：1
解释：
删除 1 和 100 后只有一个平均值，所以我们返回 1 。


提示：

2 <= nums.length <= 100
nums.length 是偶数。
0 <= nums[i] <= 100
 */
pub fn distinct_averages(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    // 定义左指针
    let mut left = 0;
    // 定义右指针
    let mut right = nums.len() - 1;
    // 对数组进行排序
    nums.sort();
    // 定义Set
    let mut hash_set = HashSet::<i32>::with_capacity(nums.len());
    // 遍历数组
    while left < right {
        hash_set.insert(nums[left] + nums[right]);
        left += 1;
        right -= 1;
    }
    hash_set.len() as i32
}

/**
 * 2824. 统计和小于目标的下标对数目

提示
给你一个下标从 0 开始长度为 n 的整数数组 nums 和一个整数 target ，
请你返回满足 0 <= i < j < n 且 nums[i] + nums[j] < target 的下标对 (i, j) 的数目。


示例 1：

输入：nums = [-1,1,2,3,1], target = 2
输出：3
解释：总共有 3 个下标对满足题目描述：
- (0, 1) ，0 < 1 且 nums[0] + nums[1] = 0 < target
- (0, 2) ，0 < 2 且 nums[0] + nums[2] = 1 < target
- (0, 4) ，0 < 4 且 nums[0] + nums[4] = 0 < target
注意 (0, 3) 不计入答案因为 nums[0] + nums[3] 不是严格小于 target 。
示例 2：

输入：nums = [-6,2,5,-2,-7,-1,3], target = -2
输出：10
解释：总共有 10 个下标对满足题目描述：
- (0, 1) ，0 < 1 且 nums[0] + nums[1] = -4 < target
- (0, 3) ，0 < 3 且 nums[0] + nums[3] = -8 < target
- (0, 4) ，0 < 4 且 nums[0] + nums[4] = -13 < target
- (0, 5) ，0 < 5 且 nums[0] + nums[5] = -7 < target
- (0, 6) ，0 < 6 且 nums[0] + nums[6] = -3 < target
- (1, 4) ，1 < 4 且 nums[1] + nums[4] = -5 < target
- (3, 4) ，3 < 4 且 nums[3] + nums[4] = -9 < target
- (3, 5) ，3 < 5 且 nums[3] + nums[5] = -3 < target
- (4, 5) ，4 < 5 且 nums[4] + nums[5] = -8 < target
- (4, 6) ，4 < 6 且 nums[4] + nums[6] = -4 < target


提示：

1 <= nums.length == n <= 50
-50 <= nums[i], target <= 50
 */
pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    // 排序数组
    nums.sort();
    // 统计
    let mut cnt = 0;
    // 遍历数组
    for i in 1..nums.len() {
        // 定义左指针
        let mut left = 0;
        // 定义右指针
        let mut right = i - 1;
        while left < right {
            let mid = left + right + 1 >> 1;
            if nums[i] + nums[mid] < target {
                // 右移左指针
                left = mid;
            } else {
                // 左移右指针
                right = mid - 1;
            }
        }
        if nums[i] + nums[right] < target {
            cnt += right + 1;
        }
    }
    cnt as i32
}

/// 215. 数组中的第K个最大元素
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

fn partition(arr: &mut [i32], start: i32, end: i32) -> i32 {
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

/**
 * 面试题 16.24. 数对和
中等
相关标签
相关企业
提示
设计一个算法，找出数组中两数之和为指定值的所有整数对。一个数只能属于一个数对。

示例 1:

输入: nums = [5,6,5], target = 11
输出: [[5,6]]
示例 2:

输入: nums = [5,6,5,6], target = 11
输出: [[5,6],[5,6]]
提示：

nums.length <= 100000
-10^5 <= nums[i], target <= 10^5
 */
pub fn pair_sums(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![];
    }
    let mut nums = nums;
    nums.sort();
    let mut res = vec![];
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let sum = nums[left] + nums[right];
        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            res.push(vec![nums[left], nums[right]]);
            left += 1;
            right -= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        let nums = vec![-4, -1, 0, 3, 10];
        let res = sorted_squares(nums);
        println!("{:?}", res);
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
    fn test_find_the_array_conc_val() {
        let nums = vec![1, 2, 3, 4, 5];
        let res = find_the_array_conc_val(nums);
        println!("{:?}", res)
    }

    #[test]
    fn test_distinct_averages() {
        let nums = vec![2, 1, 3, 4];
        let res = distinct_averages(nums);
        println!("{:?}", res)
    }

    #[test]
    fn test_count_pairs() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = 2;
        let res = count_pairs(nums, target);
        println!("{:?}", res)
    }

    #[test]
    fn test_pair_sums() {
        let nums = vec![5, 6, 5, 6];
        let target = 11;
        assert_eq!(pair_sums(nums, target), vec![vec![5, 6], vec![5, 6]]);
    }
}
