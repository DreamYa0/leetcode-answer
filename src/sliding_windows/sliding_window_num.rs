use std::collections::{HashMap, HashSet};

/// 2799. 统计完全子数组的数目
/// https://leetcode.cn/problems/count-complete-subarrays-in-an-array/
pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    // 先统计nums数组中不同元素的个数
    let mut set = HashSet::with_capacity(nums.len());
    for n in nums.iter() {
        set.insert(n);
    }
    // nums数组中不同元素的个数
    let len = set.len();
    // 定义结果
    let mut ans = 0;
    // 定义慢指针
    let mut slow = 0;
    // 定义哈希表来统计滑动窗口中不同元素的个数
    let mut hash = HashMap::new();
    for fast in 0..nums.len() {
        // nums[fast] 元素放入哈希表中
        hash.insert(nums[fast], hash.get(&nums[fast]).or(Some(&0)).unwrap() + 1);
        while hash.keys().len() == len {
            // nums[slow] 滑出窗口
            if *hash.get(&nums[slow]).unwrap() == 1 {
                hash.remove(&nums[slow]);
            } else {
                hash.insert(nums[slow], hash.get(&nums[slow]).unwrap() - 1);
            }
            // 右移slow指针
            slow += 1;
        }
        ans += slow;
    }
    ans as i32
}

/// 713. 乘积小于 K 的子数组
/// https://leetcode.cn/problems/subarray-product-less-than-k/
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    //同样排除k为1的情况比如  [1,1,1] k=1
    if k <= 1 {
        return 0;
    }
    // 定义结果
    let mut ans = 0;
    // 定义慢指针
    let mut slow = 0;
    // 定义窗口内元素的积
    let mut mult = 1;
    for fast in 0..nums.len() {
        mult *= nums[fast];
        while mult >= k {
            mult /= nums[slow];
            slow += 1;
        }
        // 每次fast指针位移到一个新位置，应该加上 x 种数组组合：
        // nums[fast]
        // nums[fast-1], nums[fast]
        // nums[fast-2], nums[fast-1], nums[fast]
        // nums[slow], ......, nums[fast-2], nums[fast-1], nums[fast]
        //共有 fast - slow + 1 种
        ans += fast - slow + 1;
    }
    ans as i32
}

/// 1358. 包含所有三种字符的子字符串数目
///
/// https://leetcode.cn/problems/number-of-substrings-containing-all-three-characters/
pub fn number_of_substrings(s: String) -> i32 {
    let s = s.into_bytes();
    // 定义结果
    let mut ans = 0;
    // 定义慢指针
    let mut slow = 0;
    // 定义哈希表来统计字符串出现的次数,存放 a - z 26个字母的acii码
    let mut cnt = vec![0; 3];
    for fast in 0..s.len() {
        cnt[(s[fast] - b'a') as usize] += 1;
        while cnt[(b'a' - b'a') as usize] >= 1
            && cnt[(b'b' - b'a') as usize] >= 1
            && cnt[(b'c' - b'a') as usize] >= 1
        {
            // 当满足条件时
            ans += s.len() - fast;
            // 从cnt同扣减掉 s[slow] 的统计
            cnt[(s[slow] - b'a') as usize] -= 1;
            slow += 1;
        }
    }
    ans as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_complete_subarrays() {
        let nums = vec![1, 3, 1, 2, 2];
        let count_complete_subarrays = count_complete_subarrays(nums);
        assert_eq!(count_complete_subarrays, 4);
    }

    #[test]
    fn test_number_of_substrings() {
        let s = "abcabc".to_string();
        let number_of_substrings = number_of_substrings(s);
        assert_eq!(number_of_substrings, 10);
    }
}
