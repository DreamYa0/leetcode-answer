use std::collections::HashMap;

/// 238. 除自身以外数组的乘积
/// 给你一个整数数组 nums，返回 数组 answer ，其中 answer[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积 。
///
/// 题目数据 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在  32 位 整数范围内。
///
/// 请 不要使用除法，且在 O(n) 时间复杂度内完成此题。
///
///
/// 示例 1:
///
/// 输入: nums = [1,2,3,4]
/// 输出: [24,12,8,6]
/// 示例 2:
///
/// 输入: nums = [-1,1,0,-3,3]
/// 输出: [0,0,9,0,0]
///
/// 提示：
///
/// 2 <= nums.length <= 105
/// -30 <= nums[i] <= 30
/// 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在  32 位 整数范围内
///
/// 进阶：你可以在 O(1) 的额外空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组 不被视为 额外空间。）
///
/// 前言
/// 这似乎是一个简单的问题，可以在线性时间和空间内解决。先计算给定数组所有元素的乘积，然后对数组中的每个元素 x，将总的乘积除以 x 来求得除自身值的以外数组的乘积。
///
/// 然而这样的解决方法有一个问题，就是如果输入数组中出现 0，那么这个方法就失效了。而且在问题中说明了不允许使用除法运算。这增加了这个问题的难度。
///
/// 方法一：左右乘积列表
/// 思路
///
/// 我们不必将所有数字的乘积除以给定索引处的数字得到相应的答案，而是利用索引左侧所有数字的乘积和右侧所有数字的乘积（即前缀与后缀）相乘得到答案。
///
/// 对于给定索引 iii，我们将使用它左边所有数字的乘积乘以右边所有数字的乘积。下面让我们更加具体的描述这个算法。
///
/// 算法
///
/// 初始化两个空数组 L 和 R。对于给定索引 i，L[i] 代表的是 i 左侧所有数字的乘积，R[i] 代表的是 i 右侧所有数字的乘积。
/// 我们需要用两个循环来填充 L 和 R 数组的值。对于数组 L，L[0] 应该是 1，因为第一个元素的左边没有元素。对于其他元素：L[i] = L[i-1] * nums[i-1]。
/// 同理，对于数组 R，R[length-1] 应为 1。length 指的是输入数组的大小。其他元素：R[i] = R[i+1] * nums[i+1]。
/// 当 R 和 L 数组填充完成，我们只需要在输入数组上迭代，且索引 i 处的值为：L[i] * R[i]。
/// 让我们用以下图片看看算法是如何工作的：
/// <img src="https://assets.leetcode-cn.com/solution-static/238/1.PNG" />
///
/// 复杂度分析
///
/// 时间复杂度：O(N)，其中 NNN 指的是数组 nums 的大小。预处理 L 和 R 数组以及最后的遍历计算都是 O(N) 的时间复杂度。
/// 空间复杂度：O(N)，其中 NNN 指的是数组 nums 的大小。使用了 L 和 R 数组去构造答案，L 和 R 数组的长度为数组 nums 的大小。
/// 方法二：空间复杂度 O(1)O(1)O(1) 的方法
/// 思路
///
/// 尽管上面的方法已经能够很好的解决这个问题，但是空间复杂度并不为常数。
///
/// 由于输出数组不算在空间复杂度内，那么我们可以将 L 或 R 数组用输出数组来计算。先把输出数组当作 L 数组来计算，然后再动态构造 R 数组得到结果。
/// 让我们来看看基于这个思想的算法。
///
/// 算法
///
/// 初始化 answer 数组，对于给定索引 i，answer[i] 代表的是 i 左侧所有数字的乘积。
/// 构造方式与之前相同，只是我们试图节省空间，先把 answer 作为方法一的 L 数组。
/// 这种方法的唯一变化就是我们没有构造 R 数组。而是用一个遍历来跟踪右边元素的乘积。
/// 并更新数组 answer[i]=answer[i]∗R。
/// 然后 RRR 更新为 R=R∗nums[i]，其中变量 RRR 表示的就是索引右侧数字的乘积。
///
/// 复杂度分析
///
/// 时间复杂度：O(N)，其中 NNN 指的是数组 nums 的大小。分析与方法一相同。
/// 空间复杂度：O(1)，输出数组不算进空间复杂度中，因此我们只需要常数的空间存放变量。
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // 数组长度
    let len = nums.len();
    //  定义L数组，用于存放 num[i] 左边所以元素之积，但不包括 num[i]
    let mut l = vec![1; len];
    //  定义R数组，用于存放 num[i] 右边所以元素之积，但不包括 num[i]
    let mut r = vec![1; len];
    for left in 1..len {
        // 计算 i 之前数的积
        l[left] = nums[left - 1] * l[left - 1];
    }

    for right in (0..len - 1).rev() {
        // 计算 i 之后数的积
        r[right] = nums[right + 1] * r[right + 1];
    }

    // 结果集
    let mut res = Vec::with_capacity(len);
    for i in 0..len {
        res.push(l[i] * r[i])
    }
    res
}

pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let (n1, n2) = (nums1.len(), nums2.len());
    let (mut index1, mut index2) = (0__usize, 0__usize);
    while index1 != n1 && index2 != n2 {
        if nums1[index1][0] == nums2[index2][0] {
            res.push(vec![nums1[index1][0], nums1[index1][1] + nums2[index2][1]]);
            index1 += 1;
            index2 += 1;
        } else if nums1[index1][0] < nums2[index2][0] {
            res.push(vec![nums1[index1][0], nums1[index1][1]]);
            index1 += 1;
        } else {
            res.push(vec![nums2[index2][0], nums2[index2][1]]);
            index2 += 1;
        }
    }
    while index1 != n1 {
        res.push(vec![nums1[index1][0], nums1[index1][1]]);
        index1 += 1;
    }
    while index2 != n2 {
        res.push(vec![nums2[index2][0], nums2[index2][1]]);
        index2 += 1;
    }
    res
}

/// 219. 存在重复元素 II
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    // 定义哈希表,k为值，v为值对应的数组下标
    let mut hash_map = HashMap::new();
    // 遍历数组
    for (idx, v) in nums.iter().enumerate() {
        if (idx as i32).abs_diff(*hash_map.get(v).or(Some(&i32::MAX)).unwrap()) as i32 <= k {
            return true;
        } else {
            hash_map.insert(v, idx as i32);
        }
    }
    false
}

/// 1118. 一月有多少天
///
/// 预备知识
///
/// 闰年是公历中的名词。闰年分为普通闰年和世纪闰年，闰年的定义：
///
/// 普通闰年：公历年份是 4 的倍数，且不是 100 的倍数的，为闰年（如 2004 年就是闰年）。
///
/// 世纪闰年：公历年份是整百数的，必须是 400 的倍数才是世纪闰年（如 1900 年不是世纪闰年，2000 年是世纪闰年）。
///
/// 闰年是为了弥补因人为历法规定造成的年度天数与地球实际公转周期的时间差而设立的。补上时间差的年份为闰年。
/// 闰年共有 366 天（ 1-12 月分别为 31 天，29 天，31 天，30 天，31 天，30 天，31 天，31 天，30 天，31 天，30 天，31 天）。
///
/// [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]  闰年 2 月有 29 天
///
/// [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]  非闰年 2 月有 28 天
pub fn number_of_days(year: i32, month: i32) -> i32 {
    // 闰年
    let year_one = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    // 非闰年
    let year_two = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        year_one[month as usize - 1]
    } else {
        year_two[month as usize - 1]
    }
}

/// 349.两个数组的交集
/// 给定两个数组 nums1 和 nums2 ，返回 它们的交集 。输出结果中的每个元素一定是 唯一 的。我们可以 不考虑输出结果的顺序 。
///
///
/// 示例 1：
///
/// 输入：nums1 = [1,2,2,1], nums2 = [2,2]
/// 输出：[2]
/// 示例 2：
///
/// 输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
/// 输出：[9,4]
/// 解释：[4,9] 也是可通过的
///
/// 提示：
///
/// 1 <= nums1.length, nums2.length <= 1000
/// 0 <= nums1[i], nums2[i] <= 1000
///
/// 思路
/// 由于nums1和nums2的长度<=1000 且 数据大小范围是 0-1000，分布也不算太广所以可以使用数组这种哈希结构，否则需要使用Set这种数据结构
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    // 创建一个临时数组，由于nums1和nums2中最大数<=1000,所以创建一个1005大小的数组足够用
    let mut temp = [0; 1005];
    if nums1.len() < nums2.len() {
        // 如果发现nums1大小小于nums2，就交换一下他们
        let temp_nums = nums1;
        nums1 = nums2;
        nums2 = temp_nums;
    }

    for num in nums1.iter() {
        // nums1中的数据在数组中记录为1，重复的数字记录一次就可以了
        temp[*num as usize] = 1;
    }

    let mut set = std::collections::HashSet::with_capacity(nums1.len());
    for num in nums2 {
        if temp[num as usize] == 1 {
            set.insert(num);
        }
    }

    set.into_iter().collect()
}

/// 给你一个长度为 n 的整数数组 nums ，请你判断在 最多 改变 1 个元素的情况下，该数组能否变成一个非递减数列。
///
/// 我们是这样定义一个非递减数列的： 对于数组中任意的 i (0 <= i <= n-2)，总满足 nums[i] <= nums[i + 1]。
///
/// 示例 1:
/// 输入: nums = [4,2,3]
/// 输出: true
/// 解释: 你可以通过把第一个 4 变成 1 来使得它成为一个非递减数列。
///
/// 示例 2:
/// 输入: nums = [4,2,1]
/// 输出: false
/// 解释: 你不能在只改变一个元素的情况下将其变为非递减数列。
///
/// 提示：
/// n == nums.length
/// 1 <= n <= 104
/// -105 <= nums[i] <= 105
///
/// 这道题给了我们一个数组，说我们最多有1次修改某个数字的机会，
///   问能不能将数组变为非递减数组。题目中给的例子太少，不能覆盖所有情况，我们再来看下面三个例子：
/// 	4，2，3
/// 	-1，4，2，3
/// 	2，3，3，2，4
///
/// 我们通过分析上面三个例子可以发现，当我们发现后面的数字小于前面的数字产生冲突后，
/// [1]有时候需要修改前面较大的数字(比如前两个例子需要修改4)，
/// [2]有时候却要修改后面较小的那个数字(比如前第三个例子需要修改2)，
/// 那么有什么内在规律吗？是有的，判断修改那个数字其实跟再前面一个数的大小有关系，
/// 首先如果再前面的数不存在，比如例子1，4前面没有数字了，我们直接修改前面的数字为当前的数字2即可。
///
/// 而当再前面的数字存在，并且小于当前数时，比如例子2，-1小于2，我们还是需要修改前面的数字4为当前数字2；
/// 如果再前面的数大于当前数，比如例子3，3大于2，我们需要修改当前数2为前面的数3。
pub fn check_possibility(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let len: usize = nums.len();
    // 记录前一个数比后一个数大的次数
    let mut count = 0;
    for i in 0..len - 1 {
        if nums[i] > nums[i + 1] {
            count += 1;
            // 从第二个数开始，如果前一个数比后一个数大，就把前一个数变成后一个数
            if i > 0 && nums[i - 1] > nums[i + 1] {
                nums[i + 1] = nums[i];
            }
        }
    }
    return count < 2;
}

/// 合并区间
/// 以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。
///
///
///
/// 示例 1：
///
/// 输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
/// 输出：[[1,6],[8,10],[15,18]]
/// 解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
/// 示例 2：
///
/// 输入：intervals = [[1,4],[4,5]]
/// 输出：[[1,5]]
/// 解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
///
///
/// 提示：
///
/// 1 <= intervals.length <= 104
/// intervals[i].length == 2
/// 0 <= starti <= endi <= 104
/// 这个问题可以通过以下步骤解决：
///
/// 首先，按照区间的开始位置对所有区间进行排序。
/// 然后，创建一个空的结果数组，并将第一个区间添加到结果数组中。
/// 遍历剩余的每个区间，如果当前区间的开始位置小于或等于结果数组中最后一个区间的结束位置，
/// 那么将结果数组中最后一个区间的结束位置更新为当前区间的结束位置和最后一个区间的结束位置中的较大值。
/// 否则，将当前区间添加到结果数组中。
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // 可变借用
    let mut intervals = intervals;
    // 使用数组中的第一个元素进行排序
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    // 创建新的数组，并把原始数组中最小的元素放入
    let mut res = vec![intervals[0].clone()];
    for i in 1..intervals.len() {
        // 循环从1开始，因为0已经放入了res数组中
        let last_index = res.last().unwrap().len() - 1;
        if res.last().unwrap()[last_index] < intervals[i][0] {
            // 如果res数组中最后一个数组的最后一个元素小于intervals[i]数组的第一个元素
            res.push(intervals[i].clone());
        } else {
            // res数组中最后一个元素
            let last_interval = res.last_mut().unwrap();
            // last_interval.len()-1 为最后一个元素的索引
            let last = last_interval.len() - 1;
            // 比较res数组中最后一个数组的最后一个元素和intervals[i]数组的最后一个元素，取最大值
            last_interval[last] = last_interval[last].max(intervals[i][intervals[i].len() - 1]);
        }
    }
    res
}

/// 寻找数组的中心索引
/// 给你一个整数数组 nums ，请计算数组的 中心下标 。
///
/// 数组 中心下标 是数组的一个下标，其左侧所有元素相加的和等于右侧所有元素相加的和。
///
/// 如果中心下标位于数组最左端，那么左侧数之和视为 0 ，因为在下标的左侧不存在元素。这一点对于中心下标位于数组最右端同样适用。
///
/// 如果数组有多个中心下标，应该返回 最靠近左边 的那一个。如果数组不存在中心下标，返回 -1 。
///
///
///
/// 示例 1：
///
/// 输入：nums = [1, 7, 3, 6, 5, 6]
/// 输出：3
/// 解释：
/// 中心下标是 3 。
/// 左侧数之和 sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11 ，
/// 右侧数之和 sum = nums[4] + nums[5] = 5 + 6 = 11 ，二者相等。
///
/// 示例 2：
///
/// 输入：nums = [1, 2, 3]
/// 输出：-1
/// 解释：
/// 数组中不存在满足此条件的中心下标。
///
/// 示例 3：
///
/// 输入：nums = [2, 1, -1]
/// 输出：0
/// 解释：
/// 中心下标是 0 。
/// 左侧数之和 sum = 0 ，（下标 0 左侧不存在元素），
/// 右侧数之和 sum = nums[1] + nums[2] = 1 + -1 = 0 。
///
/// 在Rust中，你可以通过以下步骤来找到数组的中心下标：
///
/// 首先，计算数组的总和。
/// 然后，从左到右遍历数组，对于每个元素，如果它的左侧所有元素的和等于总和减去它和它右侧所有元素的和，那么这个元素就是中心下标。
/// 如果遍历完数组都没有找到中心下标，那么返回-1。
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let total: i32 = nums.iter().sum();
    let mut left_sum = 0;
    for (i, &num) in nums.iter().enumerate() {
        // num元素右侧元素之和为：total - left_sum - num
        if left_sum == total - left_sum - num {
            return i as i32;
        }
        // 左侧元素之和
        left_sum += num;
    }
    -1
}

/// 旋转数组
/// 给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。
/// 示例 1:
/// 输入: nums = [1,2,3,4,5,6,7], k = 3
/// 输出: [5,6,7,1,2,3,4]
/// 解释:
/// 向右轮转 1 步: [7,1,2,3,4,5,6]
/// 向右轮转 2 步: [6,7,1,2,3,4,5]
/// 向右轮转 3 步: [5,6,7,1,2,3,4]
/// 示例 2:
/// 输入：nums = [-1,-100,3,99], k = 2
/// 输出：[3,99,-1,-100]
/// 解释:
/// 向右轮转 1 步: [99,-1,-100,3]
/// 向右轮转 2 步: [3,99,-1,-100]
/// 提示：
/// 1 <= nums.length <= 105
/// -231 <= nums[i] <= 231 - 1
/// 0 <= k <= 105
/// 进阶：
/// 尽可能想出更多的解决方案，至少有 三种 不同的方法可以解决这个问题。
/// 你可以使用空间复杂度为 O(1) 的 原地 算法解决这个问题吗？
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    // 核心点 k % len 因为k有可能超过数组长度
    let k = k as usize % len;
    reverse(nums, 0, len);
    reverse(nums, 0, k as usize);
    reverse(nums, k as usize, len);
}

/// 反转数组 , 左闭，右闭 区间
fn reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
    while start < end {
        let temp = nums[start];
        nums[start] = nums[end - 1];
        nums[end - 1] = temp;
        start += 1;
        end -= 1;
    }
}

/// 旋转函数
/// 给定一个长度为 n 的整数数组 nums 。
///
/// 假设 arrk 是数组 nums 顺时针旋转 k 个位置后的数组，我们定义 nums 的 旋转函数  F 为：
///
/// F(k) = 0 * arrk[0] + 1 * arrk[1] + ... + (n - 1) * arrk[n - 1]
/// 返回 F(0), F(1), ..., F(n-1)中的最大值 。
///
/// 生成的测试用例让答案符合 32 位 整数。
///
/// 示例 1:
///
/// 输入: nums = [4,3,2,6]
/// 输出: 26
/// 解释:
/// F(0) = (0 * 4) + (1 * 3) + (2 * 2) + (3 * 6) = 0 + 3 + 4 + 18 = 25
/// F(1) = (0 * 6) + (1 * 4) + (2 * 3) + (3 * 2) = 0 + 4 + 6 + 6 = 16
/// F(2) = (0 * 2) + (1 * 6) + (2 * 4) + (3 * 3) = 0 + 6 + 8 + 9 = 23
/// F(3) = (0 * 3) + (1 * 2) + (2 * 6) + (3 * 4) = 0 + 2 + 12 + 12 = 26
/// 所以 F(0), F(1), F(2), F(3) 中的最大值是 F(3) = 26 。
/// 示例 2:
///
/// 输入: nums = [100]
/// 输出: 0
///
/// 提示:
///
/// n == nums.length
/// 1 <= n <= 105
/// -100 <= nums[i] <= 100
///
/// 解题思路
/// 假设数组为[a, b, c, d, e]，长度为n，数组和为sum，可以发现旋转函数如下：
///
/// F(0) = 0 * a + 1 * b + 2 * c + 3 * d + 4 * e
/// F(1) = 1 * a + 2 * b + 3 * c + 4 * d + 0 * e
/// F(2) = 2 * a + 3 * b + 4 * c + 0 * d + 1 * e
/// ...
/// F(1) - F(0) = a + b + c + d + e - 5 * e
/// F(2) - F(1) = a + b + c + d + e - 5 * d
/// ...
/// 所以 F(k) = F(k - 1) + sum - n * nums[n - k]，遍历过程每次取最大值即可。
/// 当k = 0时，数组不回发生顺时针旋转，所以F(0) = 0 * a + 1 * b + 2 * c + 3 * d + 4 * e，所以可以先计算出F(0)的值。
/// 向右旋转一次，就相当于把当前结果加上整个数组的和，再减去数组大小乘以当前最后一位
pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    // 数组和
    let sum: i32 = nums.iter().sum();
    // 数组长度
    let n = nums.len();
    // 计算F(0)
    let mut f = nums
        .iter()
        .enumerate()
        // 这个闭包接受两个参数，第一个参数是累加器的当前值，第二个参数是一个元组，这个元组包含一个元素的索引和这个元素的值。
        // 闭包的作用是将元素的值与其索引的乘积加到累加器中。
        .fold(0, |acc, (i, x)| acc + (i as i32) * x);
    // 记录最大的结果
    let mut res = f;
    for i in 1..n {
        // 计算F(1) 到 F(n - 1)
        f = f + sum - (n as i32) * nums[n - i];
        res = res.max(f)
    }
    res
}

/**
 * 121. 买卖股票的最佳时机

简单
相关标签
相关企业

给定一个数组 prices ，它的第 i 个元素 prices[i] 表示一支给定股票第 i 天的价格。

你只能选择 某一天 买入这只股票，并选择在 未来的某一个不同的日子 卖出该股票。设计一个算法来计算你所能获取的最大利润。

返回你可以从这笔交易中获取的最大利润。如果你不能获取任何利润，返回 0 。


```
示例 1：

输入：[7,1,5,3,6,4]
输出：5
解释：在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
     注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
示例 2：

输入：prices = [7,6,4,3,1]
输出：0
解释：在这种情况下, 没有交易完成, 所以最大利润为 0。


提示：

1 <= prices.length <= 105
0 <= prices[i] <= 104
```
 */
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut res = 0;
    for p in prices {
        min_price = min_price.min(p);
        res = res.max(p - min_price);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        let nums = [-1, 1, 0, -3, 3].to_vec();
        let product_except_self = product_except_self(nums);
        println!("{:?}", product_except_self)
    }

    #[test]
    fn test_contains_nearby_duplicate() {
        let nums = [1, 2, 3, 1].to_vec();
        let contains_nearby_duplicate = contains_nearby_duplicate(nums, 3);
        println!("{:?}", contains_nearby_duplicate)
    }

    #[test]
    fn test_intersection() {
        let nums1 = [1, 2, 2, 1].to_vec();
        let nums2 = [2, 2].to_vec();
        let intersection = intersection(nums1, nums2);
        println!("{:?}", intersection)
    }

    #[test]
    fn test_check_possibility() {
        let nums = vec![4, 2, 3];
        let check_possibility = check_possibility(nums);
        println!("{:?}", check_possibility)
    }

    #[test]
    fn test_merge() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(
            merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test_pivot_index() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(nums), 3);
    }

    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        println!("{:?}", nums)
    }

    #[test]
    fn test_max_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let max_profit = max_profit(prices);
        assert_eq!(max_profit, 5);
    }
}
