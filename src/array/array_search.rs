use std::{
    cmp::max,
    collections::{BTreeSet, HashMap},
};

/// 搜索插入位置
/// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
///
/// 请必须使用时间复杂度为 O(log n) 的算法。
///
///
///
/// 示例 1:
///
/// 输入: nums = [1,3,5,6], target = 5
/// 输出: 2
/// 示例 2:
///
/// 输入: nums = [1,3,5,6], target = 2
/// 输出: 1
/// 示例 3:
///
/// 输入: nums = [1,3,5,6], target = 7
/// 输出: 4
///
/// 提示:
///
/// 1 <= nums.length <= 104
/// -104 <= nums[i] <= 104
/// nums 为 无重复元素 的 升序 排列数组
/// -104 <= target <= 104
/// 这个问题可以通过二分查找来解决。二分查找是一种在有序数组中查找特定元素的搜索算法。
/// 搜索过程从数组的中间元素开始，如果中间元素正好是目标值，则搜索过程结束
/// 如果目标值大于或小于中间元素，则在数组大于或小于中间元素的那一半中查找，
/// 而且跟开始一样从中间元素开始比较。如果在某一步骤数组为空，则代表找不到。
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as i32
}

/// 二分查找
pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    if len < 3 {
        if nums[0] == target {
            return 0;
        } else if nums[len - 1] == target {
            return (len - 1) as i32;
        } else {
            return -1;
        }
    }
    // 定义左右指针
    let mut left = 0;
    let mut right = len - 1;
    while left <= right {
        // 计算中间位置
        let mid = left + (right - left) / 2;
        // 获取中间值
        let mid_val = nums[mid];
        if mid_val > target {
            // 目标值在左边
            right = mid - 1;
        } else if mid_val < target {
            // 目标值在右边
            left = mid + 1;
        } else {
            // 目标值等于中间值
            return mid as i32;
        }
    }
    return -1;
}

/// 645. 错误的集合
/// 集合 s 包含从 1 到 n 的整数。不幸的是，因为数据错误，导致集合里面某一个数字复制了成了集合里面的另外一个数字的值，导致集合 丢失了一个数字 并且 有一个数字重复 。
///
/// 给定一个数组 nums 代表了集合 S 发生错误后的结果。
///
/// 请你找出重复出现的整数，再找到丢失的整数，将它们以数组的形式返回。
///
/// 示例 1：
///
/// 输入：nums = [1,2,2,4]
/// 输出：[2,3]
/// 示例 2：
///
/// 输入：nums = [1,1]
/// 输出：[1,2]
///
/// 提示：
///
/// 2 <= nums.length <= 104
/// 1 <= nums[i] <= 104
///
/// 解题思路
/// 哈希表
/// 重复的数字在数组中出现 222 次，丢失的数字在数组中出现 000 次，其余的每个数字在数组中出现 111 次。
/// 因此可以使用哈希表记录每个元素在数组中出现的次数，然后遍历从 111 到 nnn 的每个数字，
/// 分别找到出现 222 次和出现 000 次的数字，即为重复的数字和丢失的数字。
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut hash = HashMap::with_capacity(nums.len());
    for n in &nums {
        hash.insert(n, hash.get(&n).or(Some(&0)).unwrap() + 1);
    }

    let mut error_num = [0; 2];
    for i in 0..=nums.len() {
        match hash.get(&(i as i32)).or(Some(&0)) {
            Some(count) => {
                if *count == 2 {
                    error_num[0] = i as i32;
                } else if *count == 0 {
                    error_num[1] = i as i32;
                }
            }
            None => {}
        }
    }
    error_num.to_vec()
}

/// 697. 数组的度
/// 给定一个非空且只包含非负数的整数数组 nums，数组的 度 的定义是指数组里任一元素出现频数的最大值。
///
/// 你的任务是在 nums 中找到与 nums 拥有相同大小的度的最短连续子数组，返回其长度。
///
///
/// 示例 1：
///
/// 输入：nums = [1,2,2,3,1]
/// 输出：2
/// 解释：
/// 输入数组的度是 2 ，因为元素 1 和 2 的出现频数最大，均为 2 。
/// 连续子数组里面拥有相同度的有如下所示：
/// [1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
/// 最短连续子数组 [2, 2] 的长度为 2 ，所以返回 2 。
/// 示例 2：
///
/// 输入：nums = [1,2,2,3,1,4,2]
/// 输出：6
/// 解释：
/// 数组的度是 3 ，因为元素 2 重复出现 3 次。
/// 所以 [2,2,3,1,4,2] 是最短子数组，因此返回 6 。
///
/// 提示：
///
/// nums.length 在 1 到 50,000 范围内。
/// nums[i] 是一个在 0 到 49,999 范围内的整数。
///
/// 哈希表
/// 思路及解法
///
/// 记原数组中出现次数最多的数为 xxx，那么和原数组的度相同的最短连续子数组，必然包含了原数组中的全部 xxx，且两端恰为 xxx 第一次出现和最后一次出现的位置。
///
/// 因为符合条件的 xxx 可能有多个，即多个不同的数在原数组中出现次数相同。
/// 所以为了找到这个子数组，我们需要统计每一个数出现的次数，同时还需要统计每一个数第一次出现和最后一次出现的位置。
///
/// 在实际代码中，我们使用哈希表实现该功能，每一个数映射到一个长度为 333 的数组，数组中的三个元素分别代表这个数出现的次数、
/// 这个数在原数组中第一次出现的位置和这个数在原数组中最后一次出现的位置。
/// 当我们记录完所有信息后，我们需要遍历该哈希表，找到元素出现次数最多，且前后位置差最小的数。
pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    // 定义一个hashmap来统计数字：出现的频率、出现的起始位置、出现的结束位置
    let mut hash = HashMap::<i32, Vec<i32>>::with_capacity(nums.len());
    for (i, val) in nums.iter().enumerate() {
        if hash.contains_key(val) {
            hash.get_mut(val).unwrap()[0] += 1;
            hash.get_mut(val).unwrap()[2] = i as i32;
        } else {
            hash.insert(*val, vec![1, i as i32, i as i32]);
        }
    }
    // 出现的最大频次、最小子串长度
    let (mut max_fre, mut min_len) = (0, i32::MAX);
    for value in hash.values() {
        // 当频次大于之前记录的最大值时
        if value[0] > max_fre {
            // 记录频次
            max_fre = value[0];
            // 记录最小子串
            min_len = value[2] - value[1] + 1;
        } else if value[0] == max_fre && (value[2] - value[1] + 1) < min_len {
            // 当频次相同时，记录子串最短的
            min_len = value[2] - value[1] + 1;
        }
    }

    min_len
}

/// 448. 找到所有数组中消失的数字
/// 给你一个含 n 个整数的数组 nums ，其中 nums[i] 在区间 [1, n] 内。请你找出所有在 [1, n] 范围内但没有出现在 nums 中的数字，并以数组的形式返回结果。
///
///
/// 示例 1：
///
/// 输入：nums = [4,3,2,7,8,2,3,1]
/// 输出：[5,6]
/// 示例 2：
///
/// 输入：nums = [1,1]
/// 输出：[2]
///
/// 提示：
///
/// n == nums.length
/// 1 <= n <= 105
/// 1 <= nums[i] <= n
/// 进阶：你能在不使用额外空间且时间复杂度为 O(n) 的情况下解决这个问题吗? 你可以假定返回的数组不算在额外空间内。
///
/// 数组的原地操作
/// 题目更高阶的要求不使用额外的空间。这增加了难度。
///
/// 如果题目是「每个数字都出现了 2 次，只有一个数字出现了 1 次」，你会做吗？这是题目136. 只出现一次的数字。朋友们应该都知道可以用异或。而本题中每个数字出现的次数可以是 0/1/2 次，已经无法用异或了。
///
/// 真正求解本题需要用到一个奇技淫巧：原地修改数组。
///
/// 这个思想来自于长度为 N 的数组可以用来统计 1 N 各数字出现的次数；题目给出的数组的长度正好为 N，所以可以原地修改数组实现计数。
///
/// 当前元素是 nums[i]，那么我们把第 nums[i]−1 位置的元素 乘以 -1，表示这个该位置出现过。
/// 当然如果 第 nums[i]−1 位置的元素已经是负数了，表示 nums[i] 已经出现过了，就不用再把第 nums[i]−1 位置的元素乘以 -1。
/// 最后，对数组中的每个位置遍历一遍，如果 i 位置的数字是正数，说明 i 未出现过。
///
/// <img src="https://pic.leetcode-cn.com/1613182887-IlNpfN-448.gif" />
///
/// 时间复杂度：O(N)
/// 空间复杂度：O(1)
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let len = nums.len();
    for i in 0..len {
        let num = nums[i];
        if nums[(num.abs() - 1) as usize] > 0 {
            nums[(num.abs() - 1) as usize] *= -1;
        }
    }

    let mut res = Vec::with_capacity(len);
    for i in 0..len {
        if nums[i] > 0 {
            res.push(i as i32 + 1)
        }
    }

    res
}

/// 442. 数组中重复的数据
/// 给你一个长度为 n 的整数数组 nums ，其中 nums 的所有整数都在范围 [1, n] 内，且每个整数出现 一次 或 两次 。请你找出所有出现 两次 的整数，并以数组形式返回。
///
/// 你必须设计并实现一个时间复杂度为 O(n) 且仅使用常量额外空间的算法解决此问题。
///
///
/// 示例 1：
///
/// 输入：nums = [4,3,2,7,8,2,3,1]
/// 输出：[2,3]
/// 示例 2：
///
/// 输入：nums = [1,1,2]
/// 输出：[1]
/// 示例 3：
///
/// 输入：nums = [1]
/// 输出：[]
///
/// 提示：
///
/// n == nums.length
/// 1 <= n <= 105
/// 1 <= nums[i] <= n
/// nums 中的每个元素出现 一次 或 两次
pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    // 由于数组中的元素大小是 1<=num[i]<=num.len()的，所以可以创建一个长度为num.len()的数组来统计nums中数字出现的次数
    let mut count_num = vec![0; nums.len()];
    for val in nums.iter() {
        // 如数字8需要存放在索引为7的位置
        count_num[(*val as usize) - 1] += 1;
    }

    let mut res = Vec::with_capacity(nums.len());
    for (val, count) in count_num.iter().enumerate() {
        if *count == 2 {
            // 由于存放的时候减去了1，所以这个地方需要加回来
            res.push(val as i32 + 1);
        }
    }

    res
}

/// 41. 缺失的第一个正数
/// 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
///
/// 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
///
/// 示例 1：
///
/// 输入：nums = [1,2,0]
/// 输出：3
/// 示例 2：
///
/// 输入：nums = [3,4,-1,1]
/// 输出：2
/// 示例 3：
///
/// 输入：nums = [7,8,9,11,12]
/// 输出：1
///
/// 提示：
///
/// 1 <= nums.length <= 5 * 105
/// -231 <= nums[i] <= 231 - 1
///
/// 哈希表
/// 对于「前言」中提到的第一种做法：
///
/// 我们可以将数组所有的数放入哈希表，随后从 1 开始依次枚举正整数，并判断其是否在哈希表中。
///
/// 仔细想一想，我们为什么要使用哈希表？这是因为哈希表是一个可以支持快速查找的数据结构：
/// 给定一个元素，我们可以在 O(1) 的时间查找该元素是否在哈希表中。因此，我们可以考虑将给定的数组设计成哈希表的「替代产品」。
///
/// 实际上，对于一个长度为 N 的数组，其中没有出现的最小正整数只能在 [1,N+1] 中。
/// 这是因为如果 [1,N] 都出现了，那么答案是 N+1，否则答案是 [1,N] 中没有出现的最小正整数。
/// 这样一来，我们将所有在 [1,N] 范围内的数放入哈希表，也可以得到最终的答案。
/// 而给定的数组恰好长度为 N，这让我们有了一种将数组设计成哈希表的思路：
///
/// 我们对数组进行遍历，对于遍历到的数 x，如果它在 [1,N] 的范围内，那么就将数组中的第 x−1 个位置（注意：数组下标从 000 开始）打上「标记」。
/// 在遍历结束之后，如果所有的位置都被打上了标记，那么答案是 N+1，否则答案是最小的没有打上标记的位置加 1。
///
/// 那么如何设计这个「标记」呢？由于数组中的数没有任何限制，因此这并不是一件容易的事情。
/// 但我们可以继续利用上面的提到的性质：由于我们只在意 [1,N] 中的数，因此我们可以先对数组进行遍历，
/// 把不在 [1,N] 范围内的数修改成任意一个大于 N 的数（例如 N+1）。
/// 这样一来，数组中的所有数就都是正数了，因此我们就可以将「标记」表示为「负号」。算法的流程如下：
///
/// 我们将数组中所有小于等于 0 的数修改为 N+1
///
/// 我们遍历数组中的每一个数 x，它可能已经被打了标记，因此原本对应的数为 ∣x∣，其中 ∣ ∣ 为绝对值符号。
/// 如果 ∣x∣∈[1,N]，那么我们给数组中的第 ∣x∣−1 个位置的数添加一个负号。注意如果它已经有负号，不需要重复添加；
///
/// 在遍历完成之后，如果数组中的每一个数都是负数，那么答案是 N+1，否则答案是第一个正数的位置加 1。
/// <img src="https://assets.leetcode-cn.com/solution-static/41/41_fig1.png" />
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    for i in 0..len {
        if nums[i] <= 0 {
            nums[i] = len as i32 + 1;
        }
    }

    for i in 0..len {
        let num = nums[i].abs();
        if num <= len as i32 {
            nums[(num - 1) as usize] = -nums[num as usize - 1].abs();
        }
    }

    for i in 0..len {
        if nums[i] > 0 {
            return i as i32 + 1;
        }
    }

    return len as i32 + 1;
}

/// 485. 最大连续 1 的个数
/// 给定一个二进制数组 nums ， 计算其中最大连续 1 的个数。
///
///
/// 示例 1：
///
/// 输入：nums = [1,1,0,1,1,1]
/// 输出：3
/// 解释：开头的两位和最后的三位都是连续 1 ，所以最大连续 1 的个数是 3.
/// 示例 2:
///
/// 输入：nums = [1,0,1,1,0,1]
/// 输出：2
///
/// 提示：
///
/// 1 <= nums.length <= 105
/// nums[i] 不是 0 就是 1.
///
/// 解题思路
/// 遍历数组，遇到1就count++ 看看count有没有超过最大纪录，超过了就更新纪录,如果遇到的不是1，就count归零,最后返回最大纪录
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut count = 0;
    for num in nums {
        if num == 1 {
            count += 1;
            max = max.max(count);
        } else {
            count = 0;
        }
    }
    max
}

/// 495. 提莫攻击
/// 在《英雄联盟》的世界中，有一个叫 “提莫” 的英雄。他的攻击可以让敌方英雄艾希（编者注：寒冰射手）进入中毒状态。
///
/// 当提莫攻击艾希，艾希的中毒状态正好持续 duration 秒。
///
/// 正式地讲，提莫在 t 发起攻击意味着艾希在时间区间 [t, t + duration - 1]（含 t 和 t + duration - 1）处于中毒状态。
/// 如果提莫在中毒影响结束 前 再次攻击，中毒状态计时器将会 重置 ，在新的攻击之后，中毒影响将会在 duration 秒后结束。
///
/// 给你一个 非递减 的整数数组 timeSeries ，其中 timeSeries[i] 表示提莫在 timeSeries[i] 秒时对艾希发起攻击，以及一个表示中毒持续时间的整数 duration 。
///
/// 返回艾希处于中毒状态的 总 秒数。
///
/// 示例 1：
///
/// 输入：timeSeries = [1,4], duration = 2
/// 输出：4
/// 解释：提莫攻击对艾希的影响如下：
/// - 第 1 秒，提莫攻击艾希并使其立即中毒。中毒状态会维持 2 秒，即第 1 秒和第 2 秒。
/// - 第 4 秒，提莫再次攻击艾希，艾希中毒状态又持续 2 秒，即第 4 秒和第 5 秒。
/// 艾希在第 1、2、4、5 秒处于中毒状态，所以总中毒秒数是 4 。
/// 示例 2：
///
/// 输入：timeSeries = [1,2], duration = 2
/// 输出：3
/// 解释：提莫攻击对艾希的影响如下：
/// - 第 1 秒，提莫攻击艾希并使其立即中毒。中毒状态会维持 2 秒，即第 1 秒和第 2 秒。
/// - 第 2 秒，提莫再次攻击艾希，并重置中毒计时器，艾希中毒状态需要持续 2 秒，即第 2 秒和第 3 秒。
/// 艾希在第 1、2、3 秒处于中毒状态，所以总中毒秒数是 3 。
///
/// 提示：
///
/// 1 <= timeSeries.length <= 104
/// 0 <= timeSeries[i], duration <= 107
/// timeSeries 按 非递减 顺序排列
/// 方法一：单次扫描
/// 我们只需要对数组进行一次扫描就可以计算出总的中毒持续时间。
/// 我们记录艾希恢复为未中毒的起始时间 expired，
/// 设艾希遭遇第 iii 次的攻击的时间为 timeSeries[i]。当艾希遭遇第 i 攻击时：
///
/// 如果当前他正处于未中毒状态，则此时他的中毒持续时间应增加 duration​，同时更新本次中毒结束时间 expired​ 等于 timeSeries[i]+duration
/// 如果当前他正处于中毒状态，由于中毒状态不可叠加，我们知道上次中毒后结束时间为 expired​，
/// 本次中毒后结束时间为 timeSeries[i]+duration​​，因此本次中毒增加的持续中毒时间为 timeSeries[i]+duration−expired
/// 我们将每次中毒后增加的持续中毒时间相加即为总的持续中毒时间。
/// 复杂度分析
///
/// 时间复杂度：O(n)，其中 nnn 是数组 timeSeries 的长度。我们只需要遍历一遍数组即可，因此总时间复杂度为 O(n)。
///
/// 空间复杂度：O(1)。只需要记录未中毒的起始时间即可，因此时间复杂度为 O(1)。
pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    // 总的中毒时间
    let mut ans = 0;
    // 过期时间
    let mut expried = 0;
    for val in time_series {
        if val >= expried {
            // 当前处于未中毒状态
            ans += duration;
        } else {
            // 当前处于中毒状态
            ans += val + duration - expried;
        }

        // 过期时间为当前时间点加中毒时间
        expried = val + duration;
    }
    ans
}

/// 414. 第三大的数
/// 给你一个非空数组，返回此数组中 第三大的数 。如果不存在，则返回数组中最大的数。
///
///
/// 示例 1：
///
/// 输入：[3, 2, 1]
/// 输出：1
/// 解释：第三大的数是 1 。
/// 示例 2：
///
/// 输入：[1, 2]
/// 输出：2
/// 解释：第三大的数不存在, 所以返回最大的数 2 。
/// 示例 3：
///
/// 输入：[2, 2, 3, 1]
/// 输出：1
/// 解释：注意，要求返回第三大的数，是指在所有不同数字中排第三大的数。
/// 此例中存在两个值为 2 的数，它们都排第二。在所有不同数字中排第三大的数为 1 。
///
/// 提示：
///
/// 1 <= nums.length <= 104
/// -231 <= nums[i] <= 231 - 1
///
/// 进阶：你能设计一个时间复杂度 O(n) 的解决方案吗
///
/// 方法二：有序集合
/// 我们可以遍历数组，同时用一个有序集合来维护数组中前三大的数。
/// 具体做法是每遍历一个数，就将其插入有序集合，若有序集合的大小超过 3，
/// 就删除集合中的最小元素。这样可以保证有序集合的大小至多为 3，且遍历结束后，若有序集合的大小为 3，
/// 其最小值就是数组中第三大的数；若有序集合的大小不足 3，那么就返回有序集合中的最大值。
pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut btree = BTreeSet::<i32>::new();
    for num in nums {
        btree.insert(num);
        if btree.len() > 3 {
            btree.pop_first();
        }
    }
    if btree.len() == 3 {
        *btree.first().unwrap()
    } else {
        *btree.last().unwrap()
    }
}

/// 628. 三个数的最大乘积
/// 给你一个整型数组 nums ，在数组中找出由三个数组成的最大乘积，并输出这个乘积。
///
///
/// 示例 1：
///
/// 输入：nums = [1,2,3]
/// 输出：6
/// 示例 2：
///
/// 输入：nums = [1,2,3,4]
/// 输出：24
/// 示例 3：
///
/// 输入：nums = [-1,-2,-3]
/// 输出：-6
///
/// 提示：
///
/// 3 <= nums.length <= 104
/// -1000 <= nums[i] <= 1000
///
/// 方法一：排序
/// 首先将数组排序。
///
/// 如果数组中全是非负数，则排序后最大的三个数相乘即为最大乘积；如果全是非正数，则最大的三个数相乘同样也为最大乘积。
///
/// 如果数组中有正数有负数，则最大乘积既可能是三个最大正数的乘积，也可能是两个最小负数（即绝对值最大）与最大正数的乘积。
///
/// 综上，我们在给数组排序后，分别求出三个最大正数的乘积，以及两个最小负数与最大正数的乘积，二者之间的最大值即为所求答案。
///
/// 方法二：线性扫描
/// 在方法一中，我们实际上只要求出数组中最大的三个数以及最小的两个数，因此我们可以不用排序，用线性扫描直接得出这五个数。
/// 复杂度分析
///
/// 时间复杂度：O(N)，其中 NNN 为数组长度。我们仅需遍历数组一次。
///
/// 空间复杂度：O(1)
pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let (mut min_one, mut min_two, mut max_one, mut max_two, mut max_three) =
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN, i32::MIN);
    for n in nums {
        if n < min_one {
            min_two = min_one;
            min_one = n;
        } else if n < min_two {
            min_two = n;
        }

        if n > max_one {
            max_three = max_two;
            max_two = max_one;
            max_one = n
        } else if n > max_two {
            max_three = max_two;
            max_two = n;
        } else if n > max_three {
            max_three = n;
        }
    }
    max(min_one * min_two * max_one, max_one * max_two * max_three)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_search_insert() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(search_insert(nums, 5), 2);
    }

    #[test]
    fn test_binary_search() {
        let nums = vec![5];
        assert_eq!(binary_search(nums, -5), -1);
    }

    #[test]
    fn test_find_error_nums() {
        let nums = [1, 1].to_vec();
        let find_error_nums = find_error_nums(nums);
        println!("{:?}", find_error_nums)
    }

    #[test]
    fn test_find_shortest_sub_array() {
        let nums = [1, 2, 2, 3, 1, 4, 2].to_vec();
        let find_shortest_sub_array = find_shortest_sub_array(nums);
        println!("{:?}", find_shortest_sub_array)
    }

    #[test]
    fn test_find_disappeared_numbers() {
        let nums = [4, 3, 2, 7, 8, 2, 3, 1].to_vec();
        let find_disappeared_numbers = find_disappeared_numbers(nums);
        println!("{:?}", find_disappeared_numbers)
    }

    #[test]
    fn test_find_duplicates() {
        let nums = [4, 3, 2, 7, 8, 2, 3, 1].to_vec();
        let find_duplicates = find_duplicates(nums);
        println!("{:?}", find_duplicates)
    }

    #[test]
    fn test_first_missing_positive() {
        let nums = [3, 4, -1, 1].to_vec();
        let first_missing_positive = first_missing_positive(nums);
        println!("{:?}", first_missing_positive);
    }

    #[test]
    fn test_find_poisoned_duration() {
        let time_series = [1, 4].to_vec();
        let find_poisoned_duration = find_poisoned_duration(time_series, 2);
        println!("{:?}", find_poisoned_duration);
    }

    #[test]
    fn test_third_max() {
        let nums = [2, 2, 3, 1].to_vec();
        let third_max = third_max(nums);
        println!("{:?}", third_max)
    }

    #[test]
    fn test_maximum_product() {
        let nums = [-100, -98, -1, 2, 3, 4].to_vec();
        let maximum_product = maximum_product(nums);
        println!("{:?}", maximum_product)
    }
}
