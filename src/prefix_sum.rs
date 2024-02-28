use std::cmp::{max, min};
/// 53. 最大子数组和
/// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
///
/// 子数组 是数组中的一个连续部分。
///
/// 示例 1：
///
/// 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
/// 输出：6
/// 解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
/// 示例 2：
///
/// 输入：nums = [1]
/// 输出：1
/// 示例 3：
///
/// 输入：nums = [5,4,-1,7,8]
/// 输出：23
///
/// 提示：
///
/// 1 <= nums.length <= 105
/// -104 <= nums[i] <= 104
///
/// 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
///
/// 前缀和解法
/// 思路
/// 由于子数组的元素和等于两个前缀和的差，所以求出 nums\textit{nums}nums 的前缀和，问题就变成 121. 买卖股票的最佳时机 了。本题子数组不能为空，相当于一定要交易一次。
///
/// 我们可以一边遍历数组计算前缀和，一边维护前缀和的最小值（相当于股票最低价格），用当前的前缀和（卖出价格）减去前缀和的最小值（买入价格），就得到了以当前元素结尾的子数组和的最大值（利润），用它来更新答案的最大值（最大利润）。
///
/// 请注意，由于题目要求子数组不能为空，应当先计算前缀和-最小前缀和，再更新最小前缀和。相当于不能在同一天买入股票又卖出股票。
///
/// 如果先更新最小前缀和，再计算前缀和-最小前缀和，就会把空数组的元素和 000 算入答案。
///
/// 示例 1 [−2,1,−3,4,−1,2,1,−5,4] 的计算流程如下：
///
/// i	前缀和	最小前缀和	前缀和-最小前缀和
///
/// 0	−2	0	−2
///
/// 1	−1	−2	1
///
/// 2	−4	−2	−2
///
/// 3	0	−4	4
///
/// 4	−1	−4	3
///
/// 5	1	−4	5
///
/// 6	2	−4	6
///
/// 7	−3	−4	1
///
/// 8	1	−4	5
///
/// 前缀和-最小前缀和的最大值等于 6，即为答案。
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // 定义结果
    let mut res = nums[0];
    // 定义当前和
    let mut cur_sum = 0;
    // 最小前缀和
    let mut min_sum = 0;
    // 遍历数组
    for num in nums {
        // 计算当前和
        cur_sum += num;
        // 更新结果
        res = max(res, cur_sum - min_sum);
        // 更新最小前缀和
        min_sum = min(min_sum, cur_sum);
    }
    res
}

/// 1480. 一维数组的动态和
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; nums.len()];
    for (idx, v) in nums.iter().enumerate() {
        if idx > 0 {
            ans[idx] = ans[idx - 1] + *v;
        } else {
            ans[idx] = *v;
        }
    }
    ans
}

/// 1588. 所有奇数长度子数组的和
pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    // 定义前缀和数组
    let mut prefix = vec![0; arr.len() + 1];
    // 计算前缀和
    for i in 0..arr.len() {
        prefix[i + 1] = prefix[i] + arr[i];
    }
    // 定义结果
    let mut ans = 0;
    for i in 0..arr.len() {
        let mut j = 1;
        while i + j <= arr.len() {
            ans += prefix[i + j] - prefix[i];
            j += 2;
        }
    }
    ans
}

/// 2485. 找出中枢整数
pub fn pivot_integer(n: i32) -> i32 {
    let t = (n * n + n) / 2;
    let x = (t as f64).sqrt() as i32;
    if x.pow(2) == t {
        x
    } else {
        -1
    }
}

/// 1732. 找到最高海拔
pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    // 定义结果数组
    let mut res = vec![0; gain.len() + 1];
    // 遍历数组
    for i in 1..res.len() {
        res[i] = res[i - 1] + gain[i - 1];
    }
    res.iter().max().unwrap().clone()
}

/// 1893. 检查是否区域内所有整数都被覆盖
pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut flags = vec![false; 51];
    for range in ranges {
        let l = range[0].max(left) as usize;
        let r = range[1].min(right) as usize;
        for i in l..=r {
            flags[i] = true;
        }
    }
    for i in (left as usize)..=(right as usize) {
        if flags[i] == false {
            return false;
        }
    }
    true
}

/// 1422. 分割字符串的最大得分
pub fn max_score(s: String) -> i32 {
    // 定义得分
    let mut score = 0;
    let len = s.len();
    let s = s.chars().collect::<Vec<char>>();
    if s[0] == '0' {
        score += 1;
    }
    for i in 1..len {
        if s[i] == '1' {
            score += 1;
        }
    }
    // 定义结果
    let mut ans = score;
    for i in 1..len - 1 {
        if s[i] == '0' {
            score += 1;
        } else {
            score -= 1;
        }
        ans = ans.max(score);
    }
    ans
}

/// LCR 012. 寻找数组的中心下标
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    // 计算整个数组的总和
    let total = nums.iter().sum();
    // 当前和
    let mut sum = 0;
    // 设其左侧元素之和为 sum，则其右侧元素之和为 total−numsi−sum。左右侧元素相等即为 sum=total−numsi−sum，即 2×sum+numsi=total
    for i in 0..nums.len() {
        if 2 * sum + nums[i] == total {
            return i as i32;
        }
        sum += nums[i]
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(nums), 6);
    }

    #[test]
    fn test_running_sum() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(running_sum(nums), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_sum_odd_length_subarrays() {
        let arr = vec![1, 4, 2, 5, 3];
        assert_eq!(sum_odd_length_subarrays(arr), 58);
    }

    #[test]
    fn test_pivot_integer() {
        let n = 8;
        assert_eq!(pivot_integer(n), 6);
    }

    #[test]
    fn test_largest_altitude() {
        let gain = vec![-5, 1, 5, 0, -7];
        assert_eq!(largest_altitude(gain), 1);
    }

    #[test]
    fn test_is_covered() {
        let ranges = vec![vec![1, 10], vec![10, 20]];
        let left = 21;
        let right = 21;
        assert_eq!(is_covered(ranges, left, right), false);
    }

    #[test]
    fn test_max_score() {
        let s = "011101".to_string();
        assert_eq!(max_score(s), 5);
    }

    #[test]
    fn test_pivot_index() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(nums), 3);
    }
}
